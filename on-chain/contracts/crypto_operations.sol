// SPDX-License-Identifier: GPL-3.0
pragma solidity >=0.8.2 <0.9.0;

import "./DDH.sol";
import "./BLS.sol";

contract CryptoOperations {
    event GasMeasuredOperations(uint256 gas);

    function bench_secp256k1_hash_to_curve(
        uint256[2] memory pk,
        uint256 input
    ) public {
        emit GasMeasuredOperations(
            DDH.bench_secp256k1_hash_to_curve(pk, input)
        );
    }

    function bench_ddh_vrf_ver(
        bytes memory inp,
        bytes32 y,
        uint256[2] memory pk,
        DDH.Proof memory proof
    ) public {
        emit GasMeasuredOperations(DDH.bench_ddh_vrf_ver(inp, y, pk, proof));
    }

    function bench_bn254_hash_to_curve(
        bytes32 domain,
        bytes memory message
    ) public {
        emit GasMeasuredOperations(
            BLS.bench_bn254_hash_to_curve(domain, message)
        );
    }
    function bench_bls_vrf_ver_str(
        bytes32 y,
        bytes32 domain,
        bytes memory inp_str,
        uint256[4] memory pk,
        uint256[2] memory sig
    ) public {
        emit GasMeasuredOperations(
            BLS.bench_bls_vrf_ver_str(y, domain, inp_str, pk, sig)
        );
    }

    /* UNUSED
     *
     * function bench_bls_vrf_ver_ecp(
     *     bytes32 y,
     *     uint256[2] memory inp,
     *     uint256[4] memory pk,
     *     uint256[2] memory sig
     * ) public {
     *     emit GasMeasuredOperations(BLS.bench_bls_vrf_ver_ecp(y, inp, pk, sig));
     * }
     */
    function bench_bls_sig_ver_str(
        bytes32 domain,
        bytes memory inp_str,
        uint256[4] memory pk,
        uint256[2] memory sig
    ) public {
        emit GasMeasuredOperations(
            BLS.bench_bls_sig_ver_str(domain, inp_str, pk, sig)
        );
    }

    function bench_bls_verify_pairing(
        uint256[2] memory inp,
        uint256[4] memory pk,
        uint256[2] memory sig
    ) public {
        emit GasMeasuredOperations(BLS.bench_verify_pairing(inp, pk, sig));
    }

    function _hash_gamma_to_y(
        uint256[2] memory gamma
    ) public pure returns (bytes32) {
        return keccak256(abi.encodePacked(gamma));
    }
}
