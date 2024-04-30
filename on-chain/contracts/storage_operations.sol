// SPDX-License-Identifier: GPL-3.0
pragma solidity >=0.8.2 <0.9.0;

contract StorageOperations {
    event GasMeasuredOperations(uint256 gas, uint256 nonce);

    uint256 private nonce = 0;
    mapping(uint256 => bytes32) private commitments;
    mapping(uint256 => uint256[2]) private bn254_points;

    function bench_increment_nonce() public {
        uint256 startGas = gasleft();
        nonce += 1;
        uint256 endGas = gasleft();
        emit GasMeasuredOperations(startGas - endGas, nonce);
    }

    function bench_store_hash(uint256 key, bytes32 inp) public {
        uint256 startGas = gasleft();
        commitments[key] = inp;
        uint256 endGas = gasleft();
        emit GasMeasuredOperations(startGas - endGas, key);
    }

    function bench_hash_and_store(uint256 key, bytes memory inp) public {
        uint256 startGas = gasleft();
        commitments[key] = keccak256(inp);
        uint256 endGas = gasleft();
        emit GasMeasuredOperations(startGas - endGas, key);
    }

    function bench_delete_commitment(uint256 key) public {
        uint256 startGas = gasleft();
        delete commitments[key];
        uint256 endGas = gasleft();
        emit GasMeasuredOperations(startGas - endGas, key);
    }

    function bench_store_ecp(uint256 key, uint256[2] memory inp) public {
        uint256 startGas = gasleft();
        bn254_points[key] = inp;
        uint256 endGas = gasleft();
        emit GasMeasuredOperations(startGas - endGas, key);
    }
}
