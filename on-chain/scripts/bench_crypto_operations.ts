// Import Web3.js library
import Web3 from 'web3';
import contractABI from '../contracts/artifacts/CryptoOperations.json';
import { ddh_proof_fixed_gamma, get_random_bytes, get_random_uint256_2, get_random_uint256_4, get_random_uint256 } from './input_factory';
import addresses from './addresses.json';



const RUNS = 10;

console.log(`running bench_crypto_operations with ${RUNS} repeitions`)
// Set up Web3 provider (for Remix JavaScript VM)
const web3 = new Web3(web3Provider);
const eventName = 'GasMeasuredOperations';

let gasUsed = [[], [], [], [], [], []];
let sums = [0, 0, 0, 0, 0, 0];
let names = [
    "bench_secp256k1_hash_to_curve",
    "bench_ddh_vrf_ver",
    "bench_bn254_hash_to_curve",
    "bench_bls_vrf_ver_str",
    "bench_bls_verify_pairing",
    "bench_bls_sig_ver_str",
];
(async () => {
    try {
        const contract = new web3.eth.Contract(contractABI.abi, addresses.crypto_operations);
        const accounts = await web3.eth.getAccounts()

        await Promise.all([
            bench_secp256k1_hash_to_curve(contract, accounts[0]),
            bench_ddh_vrf_ver(contract, accounts[1]),
            bench_bn254_hash_to_curve(contract, accounts[2]),
            bench_bls_vrf_ver_str(contract, accounts[3]),
            bench_bls_verify_pairing(contract, accounts[4]),
            bench_bls_sig_ver_str(contract, accounts[5]),
        ])


        console.log(`total execution gas for ${RUNS} hash_to_secp256k1 operations ${sums[0]}`);
        console.log(`total execution gas for ${RUNS} ddh_vrf_verification operations ${sums[1]}`);
        console.log(`total execution gas for ${RUNS} hash_to_bn254_g1 operations ${sums[2]}`);
        console.log(`total execution gas for ${RUNS} glow_vrf_verification operations ${sums[3]}`);
        console.log(`total execution gas for ${RUNS} verify_pairing_signature operations ${sums[4]}`);
        console.log(`total execution gas for ${RUNS} bench_bls_sig_ver_str operations ${sums[5]}`);

        console.log(`average execution gas for ${RUNS} hash_to_secp256k1 operations ${sums[0] / RUNS}`);
        console.log(`average execution gas for ${RUNS} ddh_vrf_verification operations ${sums[1] / RUNS}`);
        console.log(`average execution gas for ${RUNS} hash_to_bn254_g1 operations ${sums[2] / RUNS}`);
        console.log(`average execution gas for ${RUNS} glow_vrf_verification operations ${sums[3] / RUNS}`);
        console.log(`average execution gas for ${RUNS} verify_pairing_signature operations ${sums[4] / RUNS}`);
        console.log(`average execution gas for ${RUNS} bench_bls_sig_ver_str operations ${sums[5] / RUNS}`);

        for (let i = 0; i < 6; i++) {
            console.log(`raw data for ${names[i]}`)
            console.log(gasUsed[i]);
        }
    } catch (e) {
        console.log(e.message)
    }
})()


async function bench_secp256k1_hash_to_curve(contract, account) {
    for (let i = 0; i < RUNS; i++) {
        let pk = get_random_uint256_2();
        let y = get_random_uint256();
        await contract.methods.bench_secp256k1_hash_to_curve(pk, y).send({ from: account })
            .on('receipt', (receipt) => {
                const events = receipt.events;
                if (Object.hasOwnProperty.call(events, eventName)) {
                    const event = events[eventName];
                    let gas = Number(event.returnValues['gas'])
                    gasUsed[0].push(gas)
                    sums[0] += gas
                } else {
                    console.error()
                }
            })
            .on('error', (error) => {
                console.error("failed to call bench_secp256k1_hash_to_curve", error); // Error callback
            })
    }
}

async function bench_ddh_vrf_ver(contract, account) {
    for (let i = 0; i < RUNS; i++) {
        let inp = get_random_bytes(32);
        let pk = get_random_uint256_2();
        let gamma = get_random_uint256_2();
        let proof = ddh_proof_fixed_gamma(gamma)
        let y = await contract.methods._hash_gamma_to_y(gamma).call()
        await contract.methods.bench_ddh_vrf_ver(inp, y, pk, proof).send({ from: account })
            .on('receipt', (receipt) => {
                const events = receipt.events;
                if (Object.hasOwnProperty.call(events, eventName)) {
                    const event = events[eventName];
                    let gas = Number(event.returnValues['gas'])
                    gasUsed[1].push(gas)
                    sums[1] += gas
                } else {
                    console.error()
                }
            })
            .on('error', (error) => {
                console.error("failed to call bench_ddh_vrf_ver", error); // Error callback
            })
    }
}

async function bench_bn254_hash_to_curve(contract, account) {
    for (let i = 0; i < RUNS; i++) {
        let domain = get_random_bytes(32);
        let msg = get_random_bytes(32);
        await contract.methods.bench_bn254_hash_to_curve(domain, msg).send({ from: account })
            .on('receipt', (receipt) => {
                const events = receipt.events;
                if (Object.hasOwnProperty.call(events, eventName)) {
                    const event = events[eventName];
                    let gas = Number(event.returnValues['gas'])
                    gasUsed[2].push(gas)
                    sums[2] += gas
                } else {
                    console.error()
                }
            })
            .on('error', (error) => {
                console.error("failed to call bench_bn254_hash_to_curve", error); // Error callback
            })
    }
}
async function bench_bls_vrf_ver_str(contract, account) {
    for (let i = 0; i < RUNS; i++) {
        let sig = get_random_uint256_2()
        let y = await contract.methods._hash_gamma_to_y(sig).call()
        let domain = get_random_bytes(32);
        let inp = get_random_bytes(32);
        let pk = get_random_uint256_4();
        await contract.methods.bench_bls_vrf_ver_str(y, domain, inp, pk, sig).send({ from: account })
            .on('receipt', (receipt) => {
                const events = receipt.events;
                if (Object.hasOwnProperty.call(events, eventName)) {
                    const event = events[eventName];
                    let gas = Number(event.returnValues['gas'])
                    gasUsed[3].push(gas)
                    sums[3] += gas
                } else {
                    console.error()
                }
            })
            .on('error', (error) => {
                console.error("failed to call bench_bls_vrf_ver_str", error); // Error callback
            })
    }
}


async function bench_bls_verify_pairing(contract, account) {
    for (let i = 0; i < RUNS; i++) {
        let inp = get_random_uint256_2();
        let pk = get_random_uint256_4();
        let sig = get_random_uint256_2();
        await contract.methods.bench_bls_verify_pairing(inp, pk, sig).send({ from: account })
            .on('receipt', (receipt) => {
                const events = receipt.events;
                if (Object.hasOwnProperty.call(events, eventName)) {
                    const event = events[eventName];
                    let gas = Number(event.returnValues['gas'])
                    gasUsed[4].push(gas)
                    sums[4] += gas
                } else {
                    console.error()
                }
            })
            .on('error', (error) => {
                console.error("failed to call bench_bls_verify_pairing", error); // Error callback
            })
    }
}

async function bench_bls_sig_ver_str(contract, account) {
    for (let i = 0; i < RUNS; i++) {
        let sig = get_random_uint256_2()
        let domain = get_random_bytes(32);
        let inp = get_random_bytes(32);
        let pk = get_random_uint256_4();
        await contract.methods.bench_bls_sig_ver_str(domain, inp, pk, sig).send({ from: account })
            .on('receipt', (receipt) => {
                const events = receipt.events;
                if (Object.hasOwnProperty.call(events, eventName)) {
                    const event = events[eventName];
                    let gas = Number(event.returnValues['gas'])
                    gasUsed[5].push(gas)
                    sums[5] += gas
                } else {
                    console.error()
                }
            })
            .on('error', (error) => {
                console.error("failed to call bench_bls_sig_ver_str", error); // Error callback
            })
    }
}