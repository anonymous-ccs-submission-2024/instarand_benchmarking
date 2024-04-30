// Import Web3.js library
import Web3 from 'web3';
import contractABI from '../contracts/artifacts/FlexiRand.json';
import { flexirand_blinding_proof, get_random_bytes, get_random_uint256_2, get_random_uint256_4 } from './input_factory';
import addresses from './addresses.json';


const RUNS = 10;

console.log(`running bench_flexirand with ${RUNS} repeitions`)
// Set up Web3 provider (for Remix JavaScript VM)
const web3 = new Web3(web3Provider);
// const eventName = 'GasMeasurementFlexiRand';

// let execution_gas_list = [[], [], [], []];
let transaction_gas_list = [[], [], [], []];
// let execution_gas_sums = [0, 0, 0, 0];
let transaction_gas_sums = [0, 0, 0, 0];

(async () => {
    try {
        const contract = new web3.eth.Contract(contractABI.abi, addresses.flexirand);
        const accounts = await web3.eth.getAccounts()

        await contract.methods.set_pk(get_random_uint256_4()).send({ from: accounts[0] })
            .on('receipt', (_receipt) => {

            })
            .on('error', (error) => {
                console.error("failed to set public key", error); // Error callback
            })

        for (let i = 0; i < RUNS; i++) {
            await bench(contract, accounts[0])
        }


        console.log(`total transaction gas over ${RUNS} req_gen transactions ${transaction_gas_sums[0]}`);
        console.log(`total transaction gas over ${RUNS} submit_blinding transactions ${transaction_gas_sums[1]}`);
        console.log(`total transaction gas over ${RUNS} pre_ver transactions ${transaction_gas_sums[2]}`);
        console.log(`total transaction gas over ${RUNS} verify transactions ${transaction_gas_sums[3]}`);

        console.log(`average transaction gas over ${RUNS} req_gen transactions ${transaction_gas_sums[0] / RUNS}`);
        console.log(`average transaction gas over ${RUNS} submit_blinding transactions ${transaction_gas_sums[1] / RUNS}`);
        console.log(`average transaction gas over ${RUNS} pre_ver transactions ${transaction_gas_sums[2] / RUNS}`);
        console.log(`average transaction gas over ${RUNS} verify transactions ${transaction_gas_sums[3] / RUNS}`);

        // console.log(`total execution gas over ${RUNS} req_gen transactions ${execution_gas_sums[0]}`);
        // console.log(`total execution gas over ${RUNS} submit_blinding transactions ${execution_gas_sums[1]}`);
        // console.log(`total execution gas over ${RUNS} pre_ver transactions ${execution_gas_sums[2]}`);
        // console.log(`total execution gas over ${RUNS} verify transactions ${execution_gas_sums[3]}`);

        // console.log(`average execution gas over ${RUNS} req_gen transactions ${execution_gas_sums[0] / RUNS}`);
        // console.log(`average execution gas over ${RUNS} submit_blinding transactions ${execution_gas_sums[1] / RUNS}`);
        // console.log(`average execution gas over ${RUNS} pre_ver transactions ${execution_gas_sums[2] / RUNS}`);
        // console.log(`average execution gas over ${RUNS} verify transactions ${execution_gas_sums[3] / RUNS}`);

        // console.log('execution gas raw data', execution_gas_list)
        console.log('transaction gas raw data', transaction_gas_list)
    } catch (e) {
        console.error("failed to execute bench flexirand", e); // Error callback
    }
})()


async function bench(contract, account) {
    let e = get_random_bytes(32);

    let reqid = await contract.methods.gen_req(e).send({ from: account })
        .then(receipt => {
            const events = receipt.events;
            if (Object.hasOwnProperty.call(events, 'ReqGen')) {

                const event = events['ReqGen'];

                // let ex_gas = Number(event.returnValues['gas'])
                let tx_gas = Number(receipt.gasUsed)

                // execution_gas_list[0].push(ex_gas)
                // execution_gas_sums[0] += ex_gas

                transaction_gas_list[0].push(tx_gas)
                transaction_gas_sums[0] += tx_gas

                return Number(event.returnValues['reqid']);

            }
        })
        .catch(error => {
            console.error("Failed in gen_req", error); // Error callback
        })

    let x = [e, reqid];
    let x_blind = get_random_uint256_2();
    let blinding_proof = flexirand_blinding_proof();
    await contract.methods.submit_blinding(x, x_blind, blinding_proof).send({ from: account })
        .then(receipt => {
            const events = receipt.events;
            if (Object.hasOwnProperty.call(events, 'BlindedInputSubmitted')) {
                const event = events['BlindedInputSubmitted'];

                // let ex_gas = Number(event.returnValues['gas'])
                let tx_gas = Number(receipt.gasUsed)

                // execution_gas_list[1].push(ex_gas)
                // execution_gas_sums[1] += ex_gas

                transaction_gas_list[1].push(tx_gas)
                transaction_gas_sums[1] += tx_gas
            }
        })
        .catch(error => {
            console.error("failed during submit_blinding", error); // Error callback
        })

    let y_blind = get_random_uint256_2();

    await contract.methods.pre_ver(reqid, y_blind).send({ from: account })
        .then(receipt => {
            const events = receipt.events;
            if (Object.hasOwnProperty.call(events, 'Prever')) {
                const event = events['Prever'];

                // let ex_gas = Number(event.returnValues['gas'])
                let tx_gas = Number(receipt.gasUsed)

                // execution_gas_list[2].push(ex_gas)
                // execution_gas_sums[2] += ex_gas

                transaction_gas_list[2].push(tx_gas)
                transaction_gas_sums[2] += tx_gas
            }
        })
        .catch(error => {
            console.error("failed flexirand prever", error); // Error callback
        })

    let proof = get_random_uint256_2();
    let y = await contract.methods._hash_proof(proof).call()
        .catch(error => {
            console.error("failed to hash y", error);
        });

    await contract.methods.fulf(x, y, proof).send({ from: account })
        .then(receipt => {
            const events = receipt.events;
            if (Object.hasOwnProperty.call(events, 'Ver')) {
                const event = events['Ver'];

                // let ex_gas = Number(event.returnValues['gas'])
                let tx_gas = Number(receipt.gasUsed)

                // execution_gas_list[3].push(ex_gas)
                // execution_gas_sums[3] += ex_gas

                transaction_gas_list[3].push(tx_gas)
                transaction_gas_sums[3] += tx_gas
            }
        })
        .catch(error => {
            console.error("failed during verify", error); // Error callback
        })



}
