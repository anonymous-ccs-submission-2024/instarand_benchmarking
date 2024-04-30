use super::GoldbergVrf;
use crate::types::instarand::InstarandClient;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use k256::elliptic_curve::group::GroupEncoding;

impl InstarandClient for GoldbergVrf {
    fn generate_server_input(u: &[u8], pk: &Self::PK) -> Vec<u8> {
        let mut inp = Vec::new();
        inp.append(&mut u.to_vec());
        inp.append(&mut pk.to_affine().to_bytes().to_vec());
        inp
    }

    fn hash_output(client_prefix: &[u8], out: &Self::Out) -> [u8; 32] {
        let mut inp = Vec::new();

        let w_i = out.beta;
        inp.append(&mut client_prefix.to_vec());
        inp.append(&mut w_i.to_vec());

        let mut h = [0u8; 32];

        let hasher = &mut Sha3::keccak256();
        hasher.input(&inp);
        hasher.result(&mut h);
        h
    }
}
