use super::{
    hashes::{hash_to_fr, rand_fr},
    FR_LEN_BYTES, G1_LEN_BYTES,
};
use substrate_bn::{AffineG1, Fr, Group, G1};

pub struct ZkpDl {
    pub c: Fr,
    pub s: Fr,
}

pub struct InstanceDl {
    pub g: G1,
    pub g_x: G1,
}

pub fn generate_zkp(instance: &InstanceDl, witness: Fr) -> ZkpDl {
    let r = rand_fr();
    let t = instance.g * r;

    let c = hash_to_fr(&challenge_bytes(instance, &t));
    let s = r - (c * witness);

    ZkpDl { c, s }
}

pub fn validate_zkp(instance: &InstanceDl, proof: &ZkpDl) -> bool {
    if !validate_instance(instance) {
        return false;
    }
    let t_prime = (instance.g * proof.s) + (instance.g_x * proof.c);
    let c_prime = hash_to_fr(&challenge_bytes(instance, &t_prime));

    proof.c == c_prime
}

fn validate_instance(instance: &InstanceDl) -> bool {
    !(instance.g.is_zero() || instance.g_x.is_zero())
}

// note we don't return an error if input is invalid since no tests or benches generate invalid points
fn challenge_bytes(instance: &InstanceDl, t: &G1) -> [u8; 3 * G1_LEN_BYTES] {
    let g_affine = AffineG1::from_jacobian(instance.g).expect("invalid g (should not fail)");
    let g_x_affine = AffineG1::from_jacobian(instance.g_x).expect("invalid g_x (should not fail)");
    let t_affine = AffineG1::from_jacobian(*t).expect("invalid t (should not fail)");

    let mut bytes = [0u8; 3 * G1_LEN_BYTES];

    let _ = g_affine
        .x()
        .to_big_endian(&mut bytes[..FR_LEN_BYTES])
        .expect("serialization should not fail");
    let _ = g_affine
        .y()
        .to_big_endian(&mut bytes[FR_LEN_BYTES..2 * FR_LEN_BYTES])
        .expect("serialization should not fail");
    let _ = g_x_affine
        .x()
        .to_big_endian(&mut bytes[2 * FR_LEN_BYTES..3 * FR_LEN_BYTES])
        .expect("serialization should not fail");
    let _ = g_x_affine
        .y()
        .to_big_endian(&mut bytes[3 * FR_LEN_BYTES..4 * FR_LEN_BYTES])
        .expect("serialization should not fail");
    let _ = t_affine
        .x()
        .to_big_endian(&mut bytes[4 * FR_LEN_BYTES..5 * FR_LEN_BYTES])
        .expect("serialization should not fail");
    let _ = t_affine
        .y()
        .to_big_endian(&mut bytes[5 * FR_LEN_BYTES..])
        .expect("serialization should not fail");
    bytes
}
