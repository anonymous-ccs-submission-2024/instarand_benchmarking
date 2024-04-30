pub trait Vrf: Sized {
    type SK;
    type PK;
    type Out;

    fn keygen() -> (Self::SK, Self::PK);

    fn eval(inp: &[u8], sk: &Self::SK, pk: &Self::PK) -> Self::Out;

    fn ver(inp: &[u8], pk: &Self::PK, out: &Self::Out) -> bool;
}

#[cfg(test)]
pub(crate) mod test {
    use crate::{TEST_STRING_1, TEST_STRING_2};

    use super::Vrf;

    pub fn vrf_eval_ver<T: Vrf>() {
        let (sk, pk) = &T::keygen();
        let inp = TEST_STRING_1.as_bytes();
        let out = &T::eval(inp, sk, pk);
        assert!(T::ver(inp, pk, out));
    }

    pub fn vrf_wrong_input_fails<T: Vrf>() {
        let (sk, pk) = &T::keygen();
        let inp_1 = TEST_STRING_1.as_bytes();
        let inp_2 = TEST_STRING_2.as_bytes();
        let out = &T::eval(inp_1, sk, pk);
        assert!(!T::ver(inp_2, pk, out));
    }

    pub fn vrf_wrong_pk_fails<T: Vrf>() {
        let (sk_1, pk_1) = &T::keygen();
        let (_, pk_2) = &T::keygen();
        let inp = TEST_STRING_1.as_bytes();
        let out = &T::eval(inp, sk_1, pk_1);
        assert!(!T::ver(inp, pk_2, out));
    }
}
