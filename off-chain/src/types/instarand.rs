use super::vrf::Vrf;

pub trait InstarandClient: Vrf {
    fn generate_server_input(u: &[u8], pk: &Self::PK) -> Vec<u8>;

    fn hash_output(client_prefix: &[u8], out: &Self::Out) -> [u8; 32];
}
