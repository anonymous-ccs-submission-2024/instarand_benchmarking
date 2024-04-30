// SPDX-License-Identifier: GPL-3.0
pragma solidity >=0.8.2 <0.9.0;

import "./DDH.sol";

contract Vrf {
    // server public key stored on SC
    uint256[2] pk;
    // counter to ensure each request gets a unique id
    uint256 internal reqid = 0;

    // on-chain storage for unfulfilled requests
    // maps request ID to hash of client input which acts as commitment
    mapping(uint256 => bytes32) private requests;

    // events emitted to provide details to listeners about corresponding transactions
    event ReqGen(uint256 reqid, bytes x);
    event ReqFulf(uint256 reqid, bytes32 y);

    // formatted input combining client input with reqid
    struct FormattedInput {
        bytes x;
        uint256 reqid;
    }

    function set_pk(uint256[2] memory _pk) public {
        pk = _pk;
    }

    // generate VRF request
    function req(bytes memory x) public returns (uint256) {
        // increment request id
        reqid = reqid + 1;
        // store commitment to client input x
        requests[reqid] = keccak256(abi.encodePacked(x));
        // emit event for request generation for listening parties
        emit ReqGen(reqid, x);

        return reqid;
    }

    // fulfill VRF request
    function fulf(
        FormattedInput memory inp,
        bytes32 y,
        DDH.Proof memory proof
    ) public {
        // parse formatted input as (x, reqid)
        bytes memory x = inp.x;
        uint256 _reqid = inp.reqid;

        // ensure x is valid input w.r.t. an unfulfilled request with rID = _reqid
        require(requests[_reqid] == keccak256(abi.encodePacked(x)));
        // validate Goldberg VRF. This includes hashing inp to a point on secp256k1
        DDH.verify_ddh_vrf(abi.encodePacked(x, _reqid), y, pk, proof);
        // mark request as fulfilled by deleting stored commitmenet
        // (will cause first check to fail)
        delete requests[_reqid];
        // emit event for request generation for listening parties
        emit ReqFulf(_reqid, y);
    }

    // helper function used for benching
    function _hash_gamma_to_y(
        uint256[2] memory gamma
    ) public pure returns (bytes32) {
        return keccak256(abi.encodePacked(gamma));
    }
}
