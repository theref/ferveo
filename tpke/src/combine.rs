#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::*;
use ark_ec::ProjectiveCurve;

pub fn prepare_combine_fast<E: PairingEngine>(
    public_decryption_contexts: &[PublicDecryptionContextFast<E>],
    shares: &[DecryptionShareFast<E>],
) -> Vec<E::G2Prepared> {
    let mut domain = vec![]; // omega_i, vector of domain points
    let mut n_0 = E::Fr::one();
    for d_i in shares.iter() {
        domain.push(public_decryption_contexts[d_i.decrypter_index].domain);
        n_0 *= public_decryption_contexts[d_i.decrypter_index].lagrange_n_0; // n_0_i = 1 * t^1 * t^2 ...
    }
    let s = SubproductDomain::<E::Fr>::new(domain);
    let mut lagrange = s.inverse_lagrange_coefficients(); // 1/L_i
                                                          // Given a vector of field elements {v_i}, compute the vector {coeff * v_i^(-1)}
    ark_ff::batch_inversion_and_mul(&mut lagrange, &n_0); // n_0 * L_i
                                                          // L_i * [b]Z_i
    izip!(shares.iter(), lagrange.iter())
        .map(|(d_i, lambda)| {
            let decrypter = &public_decryption_contexts[d_i.decrypter_index];
            let blinded_key_share =
                decrypter.blinded_key_share.blinded_key_share;
            E::G2Prepared::from(
                // [b]Z_i * L_i
                blinded_key_share.mul(*lambda).into_affine(),
            )
        })
        .collect::<Vec<_>>()
}

pub fn prepare_combine_simple<E: PairingEngine>(
    domain: &[E::Fr],
) -> Vec<E::Fr> {
    // See https://en.wikipedia.org/wiki/Lagrange_polynomial#Optimal_algorithm
    // In this formula x_i = 0, hence numerator is x_m
    lagrange_basis_at::<E>(domain, &E::Fr::zero())
}

/// Calculate lagrange coefficients using optimized formula
/// See https://en.wikipedia.org/wiki/Lagrange_polynomial#Optimal_algorithm
pub fn lagrange_basis_at<E: PairingEngine>(
    shares_x: &[E::Fr],
    x_i: &E::Fr,
) -> Vec<<E>::Fr> {
    let mut lagrange_coeffs = vec![];
    for x_j in shares_x {
        let mut prod = E::Fr::one();
        for x_m in shares_x {
            if x_j != x_m {
                prod *= (*x_m - x_i) / (*x_m - *x_j);
            }
        }
        lagrange_coeffs.push(prod);
    }
    lagrange_coeffs
}

pub fn share_combine_fast<E: PairingEngine>(
    shares: &[DecryptionShareFast<E>],
    prepared_key_shares: &[E::G2Prepared],
) -> E::Fqk {
    let mut pairing_product: Vec<(E::G1Prepared, E::G2Prepared)> = vec![];

    for (d_i, prepared_key_share) in izip!(shares, prepared_key_shares.iter()) {
        // e(D_i, [b*omega_i^-1] Z_{i,omega_i})
        pairing_product.push((
            // D_i
            E::G1Prepared::from(d_i.decryption_share),
            // Z_{i,omega_i}) = [dk_{i}^{-1}]*\hat{Y}_{i_omega_j}]
            // Reference: https://nikkolasg.github.io/ferveo/pvss.html#validator-decryption-of-private-key-shares
            // Prepared key share is a sum of L_i * [b]Z_i
            prepared_key_share.clone(),
        ));
    }
    E::product_of_pairings(&pairing_product)
}

pub fn share_combine_simple<E: PairingEngine>(
    shares: &[DecryptionShareSimple<E>],
    lagrange_coeffs: &[E::Fr],
) -> E::Fqk {
    let mut product_of_shares = E::Fqk::one();

    // Sum of C_i^{L_i}z
    for (c_i, alpha_i) in izip!(shares, lagrange_coeffs) {
        // Exponentiation by alpha_i
        let ss = c_i.decryption_share.pow(alpha_i.into_repr());
        product_of_shares *= ss;
    }

    product_of_shares
}

#[cfg(test)]
mod tests {
    type Fr = <ark_bls12_381::Bls12_381 as ark_ec::PairingEngine>::Fr;

    #[test]
    fn test_lagrange() {
        use ark_poly::EvaluationDomain;
        use ark_std::One;
        let fft_domain =
            ark_poly::Radix2EvaluationDomain::<Fr>::new(500).unwrap();

        let mut domain = Vec::with_capacity(500);
        let mut point = Fr::one();
        for _ in 0..500 {
            domain.push(point);
            point *= fft_domain.group_gen;
        }

        let mut lagrange_n_0 = domain.iter().product::<Fr>();
        if domain.len() % 2 == 1 {
            lagrange_n_0 = -lagrange_n_0;
        }
        let s = subproductdomain::SubproductDomain::<Fr>::new(domain);
        let mut lagrange = s.inverse_lagrange_coefficients();
        ark_ff::batch_inversion_and_mul(&mut lagrange, &lagrange_n_0);
    }
}
