pub mod crypto;
pub mod impls;
pub mod types;

use rand::Rng;

#[cfg(test)]
pub const TEST_STRING_1: &str = "test string 1";
#[cfg(test)]
pub const TEST_STRING_2: &str = "test string 2";

#[cfg(test)]
pub const DEFAULT_THRESHOLD: usize = 5;
#[cfg(not(test))]
pub const DEFAULT_THRESHOLD: usize = 1;

#[cfg(test)]
pub const DEFAULT_NUM_NODES: usize = 9;
#[cfg(not(test))]
pub const DEFAULT_NUM_NODES: usize = 1;

pub fn random_input() -> [u8; 32] {
    rand::thread_rng().gen::<[u8; 32]>()
}
