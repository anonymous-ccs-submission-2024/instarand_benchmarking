// Import Web3.js library
import Web3 from 'web3';
import contractABI from '../contracts/artifacts/Vrf.json';
import { ddh_proof_fixed_gamma, get_random_bytes, get_random_uint256_2 } from './input_factory';
import addresses from './addresses.json';


const RUNS = 10;

console.log(`running bench_vrf with ${RUNS} repeitions`)
// Set up Web3 provider (for Remix JavaScript VM)
const web3 = new Web3(web3Provider);


//let execution_gas_list = [[],[]];
let transaction_gas_list = [[], []];
//let execution_gas_sums = [0,0];
let transaction_gas_sums = [0, 0];

(async () => {
    try {
        const contract = new web3.eth.Contract(contractABI.abi, addresses.vrf);
        const accounts = await web3.eth.getAccounts()


        await contract.methods.set_pk(get_random_uint256_2()).send({ from: accounts[0] })
            .on('error', (error) => {
                console.error("failed to set public key", error); // Error callback
            })

        for (let i = 0; i < RUNS; i++) {
            await bench(contract, accounts[0])
            //promises.push(bench2(x, reqid, contract, accounts[1]))
        }

        ///await Promise.all(promises)


        console.log(`total transaction gas over ${RUNS} request transactions ${transaction_gas_sums[0]}`);
        console.log(`total transaction gas over ${RUNS} fulfillment transactions ${transaction_gas_sums[1]}`);

        console.log(`average transaction gas over ${RUNS} request transactions ${transaction_gas_sums[0] / RUNS}`);
        console.log(`average transaction gas over ${RUNS} fulfillment transactions ${transaction_gas_sums[1] / RUNS}`);

        //console.log(`total execution gas over ${RUNS} request transactions ${execution_gas_sums[0]}`);
        //console.log(`total execution gas over ${RUNS} fulfillment transactions ${execution_gas_sums[1]}`);

        //console.log(`average execution gas over ${RUNS} request transactions ${execution_gas_sums[0] / RUNS}`);
        //console.log(`average execution gas over ${RUNS} fulfillment transactions ${execution_gas_sums[1] / RUNS}`);

        //console.log('execution gas raw data', execution_gas_list)
        console.log('transaction gas raw data', transaction_gas_list)
    } catch (e) {
        console.error("failed to execute bench vrf", e); // Error callback
    }
})()





async function bench(contract, account) {
    //    let reqid = -1;

    let x = get_random_bytes(32);

    let reqid = await contract.methods.req(x).send({ from: account })
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
            console.error("failed during request transaction", error); // Error callback
        })

    let gamma = get_random_uint256_2();
    let proof = ddh_proof_fixed_gamma(gamma)

    let y = await contract.methods._hash_gamma_to_y(gamma).call()

    return contract.methods.fulf([x, reqid], y, proof).send({ from: account })
        .then(receipt => {
            const events = receipt.events;
            if (Object.hasOwnProperty.call(events, 'ReqFulf')) {
                const event = events['ReqFulf'];

                // let ex_gas = Number(event.returnValues['gas'])
                let tx_gas = Number(receipt.gasUsed)

                // execution_gas_list[1].push(ex_gas)
                // execution_gas_sums[1] += ex_gas

                transaction_gas_list[1].push(tx_gas)
                transaction_gas_sums[1] += tx_gas
            }
        })
        .catch(error => {
            console.error("failed to call fulfill", error); // Error callback
        })

}
