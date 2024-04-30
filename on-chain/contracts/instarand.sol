// SPDX-License-Identifier: GPL-3.0
pragma solidity >=0.8.2 <0.9.0;

import "./BLS.sol";
import "./DDH.sol";

contract InstaRand {
    // server public key stored on SC
    uint256[4] pk_server;
    // separation domain string used when hashing to curve
    bytes32 domain;
    // counter to ensure each request gets a unique id
    uint256 internal reqid = 0;

    // on-chain storage for unfulfilled requests
    // maps request ID to hash of client input which acts as commitment
    mapping(uint256 => bytes32) private requests;
    // on-chain storage for server VRF evaluations which are reused
    // maps reqid to server output
    mapping(bytes32 => uint256[2]) internal server_outputs;

    // events emitted to provide details to listeners about corresponding transactions
    event KeyRegistered(uint256[2] pk, bytes e, uint256 reqid);
    event Prever(FormattedInput x, uint256[2] sig);
    event Ver(uint256[2] pk_c, uint256 i, bytes32 z_i);

    // formatted input combining initial client input with reqid
    struct FormattedInput {
        uint256[2] pk;
        bytes e;
        uint256 reqid;
    }

    constructor() {
        domain = bytes32(uint256(uint160(address(this))));
    }

    // set server public key
    function set_pk(uint256[4] memory pk) public {
        pk_server = pk;
    }

    // register client public key pk with associated input string e
    function register_client_key(bytes memory e, uint256[2] memory pk) public {
        // increment request id and store commitment to request input
        reqid += 1;
        //store commitment to input e on-chain
        requests[reqid] = keccak256(abi.encodePacked(e, pk, reqid));
        // emit event for listening parties

        emit KeyRegistered(pk, e, reqid);
    }

    // validate server evaluation on fomratted input containing client pk, client input string, and reqid
    function pre_ver(
        FormattedInput memory x,
        uint256[2] memory signature
    ) public {
        // encode x as a string
        bytes memory x_ = abi.encodePacked(x.e, x.pk, x.reqid);
        bytes32 key = keccak256(x_);
        // require x_ is consistent with existing request which has not yet been fulfilled
        require(requests[x.reqid] == key);

        // require no evaluation has been stored yet
        require(
            server_outputs[keccak256(x_)][0] == 0 &&
                server_outputs[keccak256(x_)][1] == 0
        );
        // validate BLS signature
        BLS.verify_bls_sig_str(domain, x_, pk_server, signature);
        // store validated signature
        server_outputs[key] = signature;
        // emit event for listening parties
        emit Prever(x, signature);
    }

    // fulfill instarand
    function fulfill(
        FormattedInput memory x, // = (e, pk, reqid)
        uint256 i,
        bytes32 w_i,
        DDH.Proof memory pi_i
    ) public {
        // encode x as a string
        bytes memory x_ = abi.encodePacked(x.e, x.pk, x.reqid);
        bytes32 key = keccak256(x_);

        // require x_ is consistent with existing request which has not yet been fulfilled
        require(requests[x.reqid] == key);
        // load server VRF output
        uint256[2] memory y = server_outputs[key];

        // ensure loaded VRF output exists
        require(y[0] != 0 || y[1] != 0);
        // generate formatted input for client vrf
        bytes memory inp = abi.encode(x_, y, i);
        // verify ddh vrf
        DDH.verify_ddh_vrf(inp, w_i, x.pk, pi_i);
        // compute random output by hashing together (x_, y, i)
        bytes32 z_i = keccak256(abi.encodePacked(inp, w_i));
        // mark request as completed by deleting commitment
        delete requests[x.reqid];
        emit Ver(x.pk, i, z_i);
    }

    // helper function used for benching
    function _hash_gamma_to_y(
        uint256[2] memory gamma
    ) public pure returns (bytes32) {
        return keccak256(abi.encodePacked(gamma));
    }
}
