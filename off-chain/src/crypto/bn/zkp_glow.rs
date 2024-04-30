use super::{
    hashes::{hash_to_fr, rand_fr},
    FR_LEN_BYTES, G1_LEN_BYTES,
};
use substrate_bn::{AffineG1, Fr, Group, G1};

#[derive(Clone)]
pub struct ZkpGlow {
    pub c: Fr,
    pub s: Fr,
}

pub struct InstanceGlow {
    pub g: G1,
    pub g_x: G1,
    pub h: G1,
    pub h_x: G1,
}

pub fn generate_zkp_glow(instance: &InstanceGlow, witness: Fr) -> ZkpGlow {
    let r = rand_fr();
    let t_g = instance.g * r;
    let t_h = instance.h * r;

    let c = hash_to_fr(&challenge_bytes(instance, &t_g, &t_h));
    let s = r - (c * witness);

    ZkpGlow { c, s }
}

pub fn validate_zkp_glow(instance: &InstanceGlow, proof: &ZkpGlow) -> bool {
    if !validate_instance(instance) {
        return false;
    }
    let t_g_prime = (instance.g * proof.s) + (instance.g_x * proof.c);
    let t_h_prime = (instance.h * proof.s) + (instance.h_x * proof.c);
    let c_prime = hash_to_fr(&challenge_bytes(instance, &t_g_prime, &t_h_prime));

    proof.c == c_prime
}

fn validate_instance(instance: &InstanceGlow) -> bool {
    !(instance.g.is_zero() || instance.g_x.is_zero())
}

// note we don't return an error if input is invalid since no tests or benches generate invalid points
fn challenge_bytes(instance: &InstanceGlow, t_g: &G1, t_h: &G1) -> [u8; 6 * G1_LEN_BYTES] {
    let g_affine = AffineG1::from_jacobian(instance.g).expect("invalid g (should not fail)");
    let g_x_affine = AffineG1::from_jacobian(instance.g_x).expect("invalid g_x (should not fail)");
    let h_affine = AffineG1::from_jacobian(instance.g).expect("invalid h (should not fail)");
    let h_x_affine = AffineG1::from_jacobian(instance.h_x).expect("invalid h_x (should not fail)");
    let t_g_affine = AffineG1::from_jacobian(*t_g).expect("invalid t_g (should not fail)");
    let t_h_affine = AffineG1::from_jacobian(*t_h).expect("invalid t_h (should not fail)");

    let mut bytes = [0u8; 6 * G1_LEN_BYTES];

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
    let _ = h_affine
        .x()
        .to_big_endian(&mut bytes[4 * FR_LEN_BYTES..5 * FR_LEN_BYTES])
        .expect("serialization should not fail");
    let _ = h_affine
        .y()
        .to_big_endian(&mut bytes[5 * FR_LEN_BYTES..6 * FR_LEN_BYTES])
        .expect("serialization should not fail");
    let _ = h_x_affine
        .x()
        .to_big_endian(&mut bytes[6 * FR_LEN_BYTES..7 * FR_LEN_BYTES])
        .expect("serialization should not fail");
    let _ = h_x_affine
        .y()
        .to_big_endian(&mut bytes[7 * FR_LEN_BYTES..8 * FR_LEN_BYTES])
        .expect("serialization should not fail");
    let _ = t_g_affine
        .x()
        .to_big_endian(&mut bytes[8 * FR_LEN_BYTES..9 * FR_LEN_BYTES])
        .expect("serialization should not fail");
    let _ = t_g_affine
        .y()
        .to_big_endian(&mut bytes[9 * FR_LEN_BYTES..10 * FR_LEN_BYTES])
        .expect("serialization should not fail");
    let _ = t_h_affine
        .x()
        .to_big_endian(&mut bytes[10 * FR_LEN_BYTES..11 * FR_LEN_BYTES])
        .expect("serialization should not fail");
    let _ = t_h_affine
        .y()
        .to_big_endian(&mut bytes[11 * FR_LEN_BYTES..])
        .expect("serialization should not fail");

    bytes
}
