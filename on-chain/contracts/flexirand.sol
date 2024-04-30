// SPDX-License-Identifier: GPL-3.0
pragma solidity >=0.8.2 <0.9.0;

import "./BLS.sol";

contract FlexiRand {
    // server public key stored on SC
    uint256[4] pk;
    // separation domain string used when hashing to curve
    bytes32 domain;
    // counter to ensure each request gets a unique id
    uint256 internal reqid = 0;

    // on-chain storage for unfulfilled requests
    // maps request ID to hash of client input which acts as commitment
    mapping(uint256 => bytes32) private requests;
    // on-chain storage of blinded_inputs
    // maps request ID to blinded input
    mapping(uint256 => uint256[2]) private inputs;
    // on-chain storage of blinded_outputs
    // maps request ID to blinded output
    mapping(uint256 => uint256[2]) private outputs;

    constructor() {
        domain = bytes32(uint256(uint160(address(this))));
    }

    // events emitted to provide details to listeners about corresponding transactions
    event ReqGen(uint256 reqid, bytes e);
    event BlindedInputSubmitted(
        uint256 reqid,
        uint256[2] x_blind,
        ZkpKdl proof
    );
    event Prever(uint256 reqid, uint256[2] y_blind);
    event Ver(uint256 reqid, bytes32 y);

    // formatted input combining initial client input with reqid
    struct FormattedInput {
        bytes e;
        uint256 reqid;
    }

    // struct respresenting schnorr proof of knowledge of dsicrete log
    struct ZkpKdl {
        uint256 s;
        uint256 c;
    }

    function set_pk(uint256[4] memory _pk) public {
        pk = _pk;
    }

    // generate initial FlexiRand request
    function gen_req(bytes memory e) public returns (uint256) {
        // increment request id and store commitment to request input
        reqid += 1;
        //store commitment to input e on-chain
        requests[reqid] = keccak256(abi.encodePacked(e));
        // emit event for request generation for listening parties
        emit ReqGen(reqid, e);

        return reqid;
    }

    // submit blinding factor along with proof that blinding is valid w.r.t. formatted input x
    function submit_blinding(
        FormattedInput memory x,
        uint256[2] memory x_blind,
        ZkpKdl memory proof
    ) public {
        // ensure x corresponds to an existing request
        require(requests[x.reqid] == keccak256(abi.encodePacked(x.e)));
        // ensure blinding hasn't already been submitted for x
        require(inputs[x.reqid][0] == 0 && inputs[x.reqid][1] == 0);
        // TODO there is a problem here if some malicious client submits invalid blinding since we do not do inp_ver on-chain
        // store blinded input
        inputs[x.reqid] = x_blind;
        // emit event for submission of blinded input for listening parties
        emit BlindedInputSubmitted(x.reqid, x_blind, proof);
    }

    // validate blinded output
    function pre_ver(uint256 _reqid, uint256[2] memory y_blind) public {
        // require pre_ver has not yet been called for request _reqid
        require(outputs[_reqid][0] == 0 && outputs[_reqid][1] == 0);

        // load stored blinded input
        uint256[2] memory x_blind = inputs[_reqid];
        // validate blinded input has been initialized
        require(!(x_blind[0] == 0 && x_blind[1] == 0));
        // validate elliptic curve pairing
        BLS.verify_pairing(x_blind, pk, y_blind);
        // store blinded output
        outputs[_reqid] = y_blind;
        // emit event for listening parties
        emit Prever(_reqid, y_blind);
    }

    // submit and validate unblinded output
    function fulf(
        FormattedInput memory x,
        bytes32 y,
        uint256[2] memory proof
    ) public {
        // ensure x corresponds to an existing request
        require(requests[x.reqid] == keccak256(abi.encodePacked(x.e)));

        // load blinded input and output
        uint256[2] memory x_blind = inputs[x.reqid];
        uint256[2] memory y_blind = outputs[x.reqid];

        // ensure blinded input and output have been initialized
        require(!(x_blind[0] == 0 && x_blind[1] == 0));
        require(!(y_blind[0] == 0 && y_blind[1] == 0));

        // validate unblinded output by calling dVRF
        BLS.verify_glow_dvrf_str(
            y,
            domain,
            abi.encodePacked(x.e, x.reqid),
            pk,
            proof
        );
        // mark request as completed by deleting request data
        delete requests[x.reqid];
        // emit event for listening parties
        emit Ver(x.reqid, y);
    }

    // helper function used for benching
    function _hash_proof(
        uint256[2] memory proof
    ) public pure returns (bytes32) {
        return keccak256(abi.encodePacked(proof));
    }
}
