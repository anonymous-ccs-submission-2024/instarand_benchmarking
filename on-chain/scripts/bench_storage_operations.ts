// Import Web3.js library
import Web3 from 'web3';
import contractABI from '../contracts/artifacts/StorageOperations.json';
import { get_random_bytes, get_random_uint256_2, get_random_uint256 } from './input_factory';
import addresses from './addresses.json';



const RUNS = 10;

console.log(`running bench_storage_operations with ${RUNS} repeitions`)
// Set up Web3 provider (for Remix JavaScript VM)
const web3 = new Web3(web3Provider);
const eventName = 'GasMeasuredOperations';

let gasUsed = [[], [], [], [], []];
let sums = [0, 0, 0, 0, 0];

let names = [
    "bench_increment_nonce",
    "bench_store_hash",
    "bench_hash_and_store",
    "bench_delete_commitment",
    "bench_store_ecp"
];

(async () => {
    try {
        const contract = new web3.eth.Contract(contractABI.abi, addresses.storage_operations);
        const accounts = await web3.eth.getAccounts()


        for (let i = 0; i < RUNS; i++) {
            await bench(contract, accounts[0])
        }

        console.log(`total execution gas for ${RUNS} increment_nonce operations ${sums[0]}`);
        console.log(`total execution gas for ${RUNS} store_hash operations ${sums[1]}`);
        console.log(`total execution gas for ${RUNS} hash_inp_and_store operations ${sums[2]}`);
        console.log(`total execution gas for ${RUNS} delete_data_from_mapping operations ${sums[3]}`);
        console.log(`total execution gas for ${RUNS} store_bn254_g1_element_in_mapping operations ${sums[4]}`);

        console.log(`average execution gas for ${RUNS} increment_nonce operations ${sums[0] / RUNS}`);
        console.log(`average execution gas for ${RUNS} store_hash operations ${sums[1] / RUNS}`);
        console.log(`average execution gas for ${RUNS} hash_inp_and_store operations ${sums[2] / RUNS}`);
        console.log(`average execution gas for ${RUNS} delete_data_from_mapping operations ${sums[3] / RUNS}`);
        console.log(`average execution gas for ${RUNS} store_bn254_g1_element_in_mapping operations ${sums[4] / RUNS}`);

        for (let i = 0; i < 5; i++) {
            console.log(`raw data for ${names[i]}`)
            console.log(gasUsed[i]);
        }
    } catch (e) {
        console.log(e.message)
    }
})()


async function bench(contract, account) {
    let nonce = await contract.methods.bench_increment_nonce().send({ from: account })
        .then(receipt => {
            const events = receipt.events;
            if (Object.hasOwnProperty.call(events, eventName)) {
                const event = events[eventName];
                let gas = Number(event.returnValues['gas'])
                gasUsed[0].push(gas)
                sums[0] += gas

                return Number(event.returnValues['nonce']);
            } else {
                console.error()
            }
        })
        .catch(error => {
            console.error("failed to call bench_increment_nonce", error); // Error callback
        })
    let hash = get_random_bytes(32)

    await contract.methods.bench_store_hash(nonce, hash).send({ from: account })
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
            console.error("failed to call bench_store_hash", error); // Error callback
        })

    // note ordering is important. We delete the entry before overwriting
    await contract.methods.bench_delete_commitment(nonce).send({ from: account })
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
            console.error("failed to call bench_delete_commitment", error); // Error callback
        })

    let inp_to_hash = get_random_bytes(32)
    await contract.methods.bench_hash_and_store(nonce, inp_to_hash).send({ from: account })
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
            console.error("failed to call bench_hash_and_store", error); // Error callback
        })


    let ecp_to_store = get_random_uint256_2()
    await contract.methods.bench_store_ecp(nonce, ecp_to_store).send({ from: account })
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
            console.error("failed to call bench_delete_commitment", error); // Error callback
        })
}
