use std::iter::Iterator;
use substrate_bn::{Fr, Group, G1};

use super::fr_from_u64;

pub fn interpolate(inp: Vec<(usize, G1)>) -> G1 {
    let (ids, evals): (Vec<_>, Vec<_>) = inp.into_iter().map(|(id, eval)| (id, eval)).unzip();
    let coefficients = lagrange_at_zero(&ids);

    let mut out = G1::zero();

    for (coefficient, eval) in coefficients.into_iter().zip(evals) {
        out = out + (eval * coefficient);
    }
    out
}

fn lagrange_at_zero(subset: &[usize]) -> Vec<Fr> {
    let len = subset.len();

    if len == 0 {
        return Vec::new();
    }
    if len == 1 {
        return vec![Fr::one()];
    }

    // note we do not check for duplicates since this can be done as shares are received

    let mut t = Fr::one();
    let mut coefficients = Vec::with_capacity(len);
    coefficients.push(t);

    for i in subset.iter().take(len - 1) {
        t = t * fr_from_u64(*i as u64);
        coefficients.push(t);
    }

    t = Fr::one();
    for (i, x) in subset[1..].iter().enumerate().rev() {
        t = t * fr_from_u64(*x as u64);
        coefficients[i] = coefficients[i] * t;
    }

    for (i, (lagrange, x_i)) in coefficients.iter_mut().zip(subset).enumerate() {
        let mut denominator = Fr::one();
        for (_, x_j) in subset.iter().enumerate().filter(|(j, _)| *j != i) {
            let diff = fr_from_u64(*x_j as u64) - fr_from_u64(*x_i as u64);
            denominator = denominator * diff;
        }

        let inverse = denominator
            .inverse()
            .expect("should not have error for test inputs");

        *lagrange = inverse * *lagrange;
    }
    coefficients
}
