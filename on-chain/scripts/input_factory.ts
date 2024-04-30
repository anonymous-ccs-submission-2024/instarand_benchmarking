export function get_random_bytes(len) {
    let randomNumber = '';
    for (let i = 0; i < 2 * len; i++) { // 64 hex characters = 256 bits
        randomNumber += Math.floor(Math.random() * 16).toString(16); // Generate a random hexadecimal digit (0-15)
    }
    return '0x' + randomNumber; // Convert to hexadecimal string
}

export function get_random_uint256() {
    return get_random_bytes(32);
}

export function get_random_uint256_2() {
    return [get_random_uint256(), get_random_uint256()];
}

export function get_random_uint256_4() {
    return [get_random_uint256(),  get_random_uint256(),  get_random_uint256(),  get_random_uint256()];
}


export function get_random_address() {
    return get_random_bytes(20);
}


export function keccak(inp) {
    return web3.utils.soliditySha3(inp);
}

export function uint256_2_and_hash(): [string[], string] {
    let uint256_2 = get_random_uint256_2();
    console.log("prehash")
    let hash = keccak(uint256_2)
    console.log("posthash")
    return [uint256_2, hash]
}

export function random_bls_proof() {
    let pi = get_random_uint256_2();
    let y = keccak(pi);
    return [y, pi];
}

export function flexirand_blinding_proof() {
    return get_random_uint256_2()
}

export function random_ddh_proof()
    //:[[string, string], [string, string], string, string, string, string, [string, string], [string, string], string]
{
    let pk = get_random_uint256_2();
    let gamma = get_random_uint256_2();
    let c = get_random_uint256();
    let s = get_random_uint256()
    let seed = get_random_uint256()
    let uWitness = get_random_address();
    let cGammaWitness = get_random_uint256_2();
    let sHashWitness = get_random_uint256_2();
    let zInv = get_random_uint256()

    return [pk, gamma, c, s, seed, uWitness, cGammaWitness, sHashWitness, zInv];
}


export function ddh_proof_fixed_gamma(gamma)
    //:[[string, string], [string, string], string, string, string, string, [string, string], [string, string], string]
{
    let pk = get_random_uint256_2();
    let c = get_random_uint256();
    let s = get_random_uint256();
    let seed = get_random_uint256()
    let address = get_random_address();
    let cGammaWitness = get_random_uint256_2();
    let sHashWitness = get_random_uint256_2();
    let zInv = get_random_uint256()

    return [pk, gamma, c, s, seed, address, cGammaWitness, sHashWitness, zInv];
}