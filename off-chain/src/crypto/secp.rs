use k256::{
    elliptic_curve::{Field, Group},
    ProjectivePoint, Scalar,
};
use rand::rngs::OsRng;

use super::rng::seedable_rng;

pub fn hash_to_curve(inp: &[u8]) -> ProjectivePoint {
    let rng = seedable_rng(inp);
    ProjectivePoint::random(rng)
}

pub fn hash_to_scalar(inp: &[u8]) -> Scalar {
    let rng = seedable_rng(inp);
    Field::random(rng)
}

pub fn rand_scalar() -> Scalar {
    Field::random(OsRng)
}

#[cfg(test)]
mod test {
    use super::{hash_to_curve, hash_to_scalar, rand_scalar};
    use crate::{TEST_STRING_1, TEST_STRING_2};

    #[test]
    fn test_rand_scalar() {
        let r1 = rand_scalar();
        let r2 = rand_scalar();

        assert_ne!(r1, r2);
    }
    #[test]
    fn test_hash_to_scalar_same_input() {
        let inp = TEST_STRING_1.as_bytes();
        let out1 = hash_to_scalar(inp);
        let out2 = hash_to_scalar(inp);

        assert_eq!(out1, out2);
    }
    #[test]
    fn test_hash_to_scalar_diff_inputs() {
        let inp_1 = TEST_STRING_1.as_bytes();
        let inp_2 = TEST_STRING_2.as_bytes();
        let out1 = hash_to_scalar(inp_1);
        let out2 = hash_to_scalar(inp_2);

        assert_ne!(out1, out2);
    }
    #[test]
    fn test_hash_to_curve_same_input() {
        let inp = TEST_STRING_1.as_bytes();
        let out1 = hash_to_curve(inp);
        let out2 = hash_to_curve(inp);

        assert_eq!(out1, out2);
    }
    #[test]
    fn test_hash_to_curve_diff_inputs() {
        let inp_1 = TEST_STRING_1.as_bytes();
        let inp_2 = TEST_STRING_2.as_bytes();
        let out1 = hash_to_curve(inp_1);
        let out2 = hash_to_curve(inp_2);

        assert_ne!(out1, out2);
    }
}
