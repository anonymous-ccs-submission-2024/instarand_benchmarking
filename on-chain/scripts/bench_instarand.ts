import Web3 from 'web3';
import contractABI from '../contracts/artifacts/InstaRand.json';
import { ddh_proof_fixed_gamma, get_random_bytes, get_random_uint256_2, get_random_uint256_4 } from './input_factory';
import addresses from './addresses.json';

const RUNS = 10;

console.log(`running bench_instarand with ${RUNS} repeitions`)

// Set up Web3 provider (for Remix JavaScript VM)
const web3 = new Web3(web3Provider);

// let execution_gas_list = [[],[],[]];
let transaction_gas_list = [[], [], []];
// let execution_gas_sums = [0,0,0];
let transaction_gas_sums = [0, 0, 0];

(async () => {
    try {
        const contract = new web3.eth.Contract(contractABI.abi, addresses.instarand);
        const accounts = await web3.eth.getAccounts()

        await contract.methods.set_pk(get_random_uint256_4()).send({ from: accounts[0] })
            .on('receipt', (_receipt) => {

            })
            .on('error', (error) => {
                console.error("failed to set public key", error); // Error callback
            })


        for (let i = 0; i < RUNS; i++) {
            await bench(contract, accounts[0], i)
        }


        console.log(`total transaction gas over ${RUNS} key_reg transactions ${transaction_gas_sums[0]}`);
        console.log(`total transaction gas over ${RUNS} pre_ver transactions ${transaction_gas_sums[1]}`);
        console.log(`total transaction gas over ${RUNS} instant_ver transactions ${transaction_gas_sums[2]}`);

        console.log(`average transaction gas over ${RUNS} key_reg transactions ${transaction_gas_sums[0] / RUNS}`);
        console.log(`average transaction gas over ${RUNS} pre_ver transactions ${transaction_gas_sums[1] / RUNS}`);
        console.log(`average transaction gas over ${RUNS} instant_ver transactions ${transaction_gas_sums[2] / RUNS}`);

        // console.log(`total execution gas over ${RUNS} key_reg transactions ${execution_gas_sums[0]}`);
        // console.log(`total execution gas over ${RUNS} pre_ver transactions ${execution_gas_sums[1]}`);
        // console.log(`total execution gas over ${RUNS} instant_ver transactions ${execution_gas_sums[2]}`);

        // console.log(`average execution gas over ${RUNS} key_reg transactions ${execution_gas_sums[0] / RUNS}`);
        // console.log(`average execution gas over ${RUNS} pre_ver transactions ${execution_gas_sums[1] / RUNS}`);
        // console.log(`average execution gas over ${RUNS} instant_ver transactions ${execution_gas_sums[2] / RUNS}`);

        // console.log('execution gas raw data', execution_gas_list)
        console.log('transaction gas raw data', transaction_gas_list)
    } catch (e) {
        console.error("failed to execute bench instarand", e); // Error callback
    }
})()


async function bench(contract, account, i) {
    let pk_c = get_random_uint256_2();
    let e = get_random_bytes(32);
    let reqid = await contract.methods.register_client_key(e, pk_c).send({ from: account })
        .then(receipt => {
            const events = receipt.events;
            if (Object.hasOwnProperty.call(events, 'KeyRegistered')) {
                const event = events['KeyRegistered'];

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

    let x = [pk_c, e, reqid]
    let sig = get_random_uint256_2()
    await contract.methods.pre_ver(x, sig).send({ from: account })
        .then(receipt => {
            const events = receipt.events;
            if (Object.hasOwnProperty.call(events, 'Prever')) {
                const event = events['Prever'];

                // let ex_gas = Number(event.returnValues['gas'])
                let tx_gas = Number(receipt.gasUsed)

                // execution_gas_list[1].push(ex_gas)
                // execution_gas_sums[1] += ex_gas

                transaction_gas_list[1].push(tx_gas)
                transaction_gas_sums[1] += tx_gas
            }
        })
        .catch(error => {
            console.log("ERR")
            console.error("failed isntarand prever", error); // Error callback
        })



    //function fulfill(ClientInput memory x, uint256 i, bytes32 w_i, DDH.Proof memory pi_i) public {

    let gamma = get_random_uint256_2();
    let pi_i = ddh_proof_fixed_gamma(gamma)

    let w_i = await contract.methods._hash_gamma_to_y(gamma).call()


    await contract.methods.fulfill(x, i, w_i, pi_i).send({ from: account })
        .then(receipt => {
            const events = receipt.events;
            if (Object.hasOwnProperty.call(events, 'Ver')) {
                const event = events['Ver'];

                // let ex_gas = Number(event.returnValues['gas'])
                let tx_gas = Number(receipt.gasUsed)

                // execution_gas_list[2].push(ex_gas)
                // execution_gas_sums[2] += ex_gas

                transaction_gas_list[2].push(tx_gas)
                transaction_gas_sums[2] += tx_gas
            }
        })
        .catch(error => {
            console.error("failed to call fulfill", error); // Error callback
        })

}
