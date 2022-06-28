use ark_bls12_381::{G1Affine, G1Projective, G2Affine};
use ark_crypto_primitives::crh::pedersen::bytes_to_bits;
use ark_ec::AffineCurve;
use ark_ec::ProjectiveCurve;
use bls_pedersen::data::puzzle_data;
use bls_pedersen::verify;
use nalgebra::*;
use num::integer::lcm;
use num_traits::identities::Zero;
use std::error::Error;
type F = fraction::Fraction;
use ark_bls12_381::Fr;
use ark_ff::fields::*;
use ark_ff::Field;

fn main() -> Result<(), Box<dyn Error>> {
    let (pk, ms, sigs) = puzzle_data();
    println!("Verifying existing messages...");
    // let mut i = 0;
    // for (m, sig) in ms.iter().zip(sigs.iter()) {
    //     verify(pk, m, *sig);
    //     println!("{}/{}", i, ms.len());
    //     i = i + 1;
    // }
    println!("Done!");

    let m = "alxs".as_bytes();
    let hash = bytes_to_bits(blake2s_simd::blake2s(m).as_bytes())
        .into_iter()
        .map(|x| x as i32 as f32)
        .collect::<Vec<_>>();

    // println!("{:?}", m_b2hash);

    let ms_hashes = ms
        .iter()
        .flat_map(|m| {
            bytes_to_bits(blake2s_simd::blake2s(m).as_bytes())
                .into_iter()
                .map(|x| x as i32 as f32)
        })
        .collect::<Vec<_>>();

    // println!("{:?}", ms_hashes);

    // Solve A x = v
    // Each bit in the hash representes whether a fixed G will be added to compute the signature
    // For each row of the matrix A, we have:
    // a_i[1].G_1 + ... + a_i[n].G_n = B_i where B_i is the hash-to-curve output of the message a_i
    // So S_i = [sk]B_i = [sk].( a_i[1].G_1 ) + ... + [sk].( a_i[n].G_n ) = a_i[1].( [sk].G_1 ) + ...
    // So if some combination of input hashes = h, then the same combination of S_i = S(h)
    // S(h) = [sk].( \sigma(a_i[1]).G_1 )
    // \sigma(S_i) = ( \sigma(a_i[1]) ).( [sk].G_1 )
    // A: the set of hashes of all input messages
    // x: linear representation of v in basis formed by A
    // v: our hash i.e. our factors for the G_i

    // Alternatively, maybe:
    // we wish to solve:
    // m_0.[sk[G_0]] + ... + m_n.[sk.[G_n]] = B,
    // where B is the signature
    // so we're solving for Ax = B
    // A is the matrix of hashes of all input messages
    // x is the vector sk[G_0], ..., sk[G_n]
    // B is the vector of signatures
    // once we solve for vec x, we can compute a signature for any message
    // since signature on any message Y: S(Y) = Y_0.[sk[G_0]] + ... + Y_n.[sk.[G_n]] = Y * x
    let v = DMatrix::from_vec(256, 1, hash);
    let mx = DMatrix::from_vec(256, 256, ms_hashes);
    println!("About to");
    println!("is invertible: {}", mx.clone().is_invertible());

    let hell_yeah = &mx
        .lu()
        .solve(&v)
        .unwrap()
        .data
        .as_vec()
        .iter()
        .map(|h: &f32| F::from(*h))
        .collect::<Vec<_>>();

    let lcm = hell_yeah
        .iter()
        .fold(1, |acc, x| lcm(acc, *x.denom().unwrap()));
    println!("LCM: {}", lcm);

    // let v: f32 = 1;
    // let d = v * 1000000;
    // let mx_inv = match mx.try_inverse() {
    //     Some(m) => m,
    //     None => return Ok(()),
    // };
    // println!("{:?}", hell_yeah);

    // let hell_yeah = &v * &mx_inv;
    // println!("{:?}", hell_yeah);

    // let sigs_mx = DMatrix::from_row_iter(
    //     256,
    //     48,
    //     sigs.iter().map(|s| s.to_vec()).flatten().map(|u| u as i8),
    // );

    // let sig_m = hell_yeah * sigs_mx;

    let mut acc = G1Projective::zero();
    for (factor, sig) in hell_yeah.iter().zip(sigs.iter()) {
        let numer = factor.numer().unwrap();
        let denom = factor.denom().unwrap();

        println!("factor: {}", lcm / denom * numer);
        let res = lcm / denom * numer;
        // res -= sig.mul(*denom);

        if let fraction::Sign::Minus = factor.sign().unwrap() {
            acc -= sig.mul(res);
        } else {
            acc += sig.mul(res);
        }
    }

    acc = acc.mul(&Fr::from(lcm).inverse().unwrap().into_repr());
    verify(pk, m, acc.into());

    // let sigs = sigs_strs
    //     .iter()
    //     .map(|&s| G1Affine::deserialize(&mut Cursor::new(hex::decode(s).unwrap())).unwrap())
    //     .collect();

    // let sig =
    // G1Affine::deserialize(&mut Cursor::new(hex::decode(sig_m.as_vector()).unwrap())).unwrap();

    // verify(pk, m, sig);

    // let mut sum: Vec<u8> = [0; 256].to_vec();
    // for m in ms {
    //     sum = sum
    //         .iter()
    //         .zip(m.iter())
    //         .map(|(&b, &v)| b + v)
    //         .collect();
    // }

    Ok(())
}
