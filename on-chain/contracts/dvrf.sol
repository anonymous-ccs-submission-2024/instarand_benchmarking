// SPDX-License-Identifier: GPL-3.0
pragma solidity >=0.8.2 <0.9.0;

import "./BLS.sol";

contract Dvrf {
    // server public key stored on SC
    uint256[4] pk;
    // separation domain string used when hashing to curve
    bytes32 domain;
    // counter to ensure each request gets a unique id
    uint256 internal reqid = 0;

    // on-chain storage for unfulfilled requests
    // maps request ID to hash of client input which acts as commitment
    mapping(uint256 => bytes32) private requests;

    // events emitted to provide details to listeners about corresponding transactions
    event ReqGen(uint256 reqid, bytes x);
    event ReqFulf(uint256 reqid, bytes32 y);
    // event GasMeasurementDvrf(uint256 gas, uint256 reqid);

    // formatted input combining client input with reqid
    struct FormattedInput {
        bytes x;
        uint256 reqid;
    }

    constructor() {
        domain = bytes32(uint256(uint160(address(this))));
    }

    function set_pk(uint256[4] memory _pk) public {
        pk = _pk;
    }

    // generate dVRF request
    function req(bytes memory x) public returns (uint256) {
        // increment request id and store commitment to request input
        reqid = reqid + 1;
        //store commitment to input x on-chain
        requests[reqid] = keccak256(abi.encodePacked(x));
        // emit event for request generation for listening parties
        emit ReqGen(reqid, x);

        return reqid;
    }

    // fulfill dVRF request
    function fulf(
        FormattedInput memory inp, // (x, reqid)
        bytes32 y,
        uint256[2] calldata proof
    ) public {
        // parse formatted input as (x, reqid)
        bytes memory x = inp.x;
        uint256 _reqid = inp.reqid;

        // ensure x is valid input w.r.t. an unfulfilled request with rID = _reqid
        require(requests[_reqid] == keccak256(abi.encodePacked(x)));
        // validate GLOW dVRF. This includes hashing inp to a point on bn254 G1
        BLS.verify_glow_dvrf_str(
            y,
            domain,
            abi.encodePacked(x, _reqid),
            pk,
            proof
        );
        // mark request as fulfilled by deleting stored commitmenet
        // (will cause first check to fail)
        delete requests[_reqid];
        // emit event for request generation for listening parties
        emit ReqFulf(_reqid, y);
    }

    // helper function used for benching
    function _hash_proof(
        uint256[2] memory proof
    ) public pure returns (bytes32) {
        return keccak256(abi.encodePacked(proof));
    }
}
