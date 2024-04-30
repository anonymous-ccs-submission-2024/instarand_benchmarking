use crate::crypto::secp::{hash_to_curve, hash_to_scalar, rand_scalar};
use crate::types::vrf::Vrf;

use super::GoldbergVrf;

use k256::elliptic_curve::{group::GroupEncoding, ops::MulByGenerator};
use k256::{ProjectivePoint, Scalar};

pub struct OutputGoldbergVrf {
    pub gamma: ProjectivePoint,
    pub c: Scalar,
    pub s: Scalar,
    pub beta: [u8; 33],
}

impl Vrf for GoldbergVrf {
    type SK = Scalar;
    type PK = ProjectivePoint;
    type Out = OutputGoldbergVrf;

    fn keygen() -> (Self::SK, Self::PK) {
        let sk = rand_scalar();
        let pk = ProjectivePoint::mul_by_generator(&sk);
        (sk, pk)
    }

    fn eval(inp: &[u8], sk: &Self::SK, _pk: &Self::PK) -> Self::Out {
        let h = hash_to_curve(inp);
        let gamma = h * sk;
        let k = rand_scalar();

        let g_x = ProjectivePoint::mul_by_generator(sk);
        let g_k = ProjectivePoint::mul_by_generator(&k);
        let h_k = h * k;

        let c = challenge_goldberg_secp256k1(&h, &g_x, &gamma, &g_k, &h_k);
        let s = k - (c * sk);

        let beta = hash_out(&gamma);

        OutputGoldbergVrf { gamma, c, s, beta }
    }

    fn ver(inp: &[u8], pk: &Self::PK, out: &Self::Out) -> bool {
        if is_infinity(&out.gamma) {
            return false;
        }
        // since secp256k1 has cofactor of 1 we don't need to check if out.gamma is a group member
        if out.beta != hash_out(&out.gamma) {
            return false;
        }
        let u = (pk * &out.c) + (ProjectivePoint::GENERATOR * out.s);

        let h = hash_to_curve(inp);

        let v = (out.gamma * out.c) + (h * out.s);

        let c_prime = challenge_goldberg_secp256k1(&h, pk, &out.gamma, &u, &v);
        out.c == c_prime
    }
}

fn challenge_goldberg_secp256k1(
    h: &ProjectivePoint,
    g_x: &ProjectivePoint,
    h_x: &ProjectivePoint,
    g_k: &ProjectivePoint,
    h_k: &ProjectivePoint,
) -> Scalar {
    let mut challenge_bytes = Vec::<u8>::new();
    let mut g_bytes = ProjectivePoint::GENERATOR.to_bytes().as_slice().to_vec();
    let mut h_bytes = h.to_bytes().as_slice().to_vec();
    let mut g_x_bytes = g_x.to_bytes().as_slice().to_vec();
    let mut h_x_bytes = h_x.to_bytes().as_slice().to_vec();
    let mut g_k_bytes = g_k.to_bytes().as_slice().to_vec();
    let mut h_k_bytes = h_k.to_bytes().as_slice().to_vec();

    challenge_bytes.append(&mut g_bytes);
    challenge_bytes.append(&mut h_bytes);
    challenge_bytes.append(&mut g_x_bytes);
    challenge_bytes.append(&mut h_x_bytes);
    challenge_bytes.append(&mut g_k_bytes);
    challenge_bytes.append(&mut h_k_bytes);

    hash_to_scalar(&challenge_bytes)
}

fn hash_out(gamma: &ProjectivePoint) -> [u8; 33] {
    // since secp256k1 has cofactor of 1 we don't need to compute gamma * CURVE_COFACTOR
    assert!(!ProjectivePoint::IDENTITY.eq(gamma));
    gamma
        .to_bytes()
        .as_slice()
        .try_into()
        .expect("ProjectivePoint should deserialize to [u8; 33]")
}

fn is_infinity(p: &ProjectivePoint) -> bool {
    ProjectivePoint::IDENTITY.eq(p)
}

#[cfg(test)]
mod test {
    use super::GoldbergVrf;
    use crate::types::vrf::test::{vrf_eval_ver, vrf_wrong_input_fails, vrf_wrong_pk_fails};

    #[test]
    fn bls_vrf_eval_ver() {
        vrf_eval_ver::<GoldbergVrf>();
    }
    #[test]
    fn bls_vrf_wrong_input_fails() {
        vrf_wrong_input_fails::<GoldbergVrf>();
    }
    #[test]
    fn bls_vrf_wrong_pk_fails() {
        vrf_wrong_pk_fails::<GoldbergVrf>();
    }
}
