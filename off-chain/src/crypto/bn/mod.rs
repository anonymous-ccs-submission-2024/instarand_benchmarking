use substrate_bn::{arith::U256, Fr};

pub mod hashes;

pub mod zkp_dl;
pub mod zkp_glow;

pub mod interpolation;

pub fn fr_from_u64(inp: u64) -> Fr {
    Fr::new(U256::from(inp)).expect("U256 derived from u64 => cannot return None")
}

const FR_LEN_BYTES: usize = 32;
const G1_LEN_BYTES: usize = 2 * FR_LEN_BYTES;
