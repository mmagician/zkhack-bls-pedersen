use ark_bls12_381::{G1Affine, G1Projective, G2Affine};
use ark_ec::AffineCurve;
use ark_ec::ProjectiveCurve;
use ark_serialize::CanonicalDeserialize;
use bitvec::prelude::*;
use bls_pedersen::data::puzzle_data;
use bls_pedersen::verify;
use nalgebra::*;
use ndarray::arr2;
use num_bigint::BigInt;
use num_traits::identities::Zero;
use std::convert::TryInto;
use std::error::Error;
use std::io::Cursor;
use std::ops::Mul;
use std::ops::Neg;
type F = fraction::Fraction;
use ark_ff::Field;
use ark_ff::{field_new, fields::*};
use ark_serialize::CanonicalSerialize;
use ark_bls12_381::Fq;

fn explode_u8(b: &u8) -> [u8; 8] {
    [
        (b >> 7) & 0x1,
        (b >> 6) & 0x1,
        (b >> 5) & 0x1,
        (b >> 4) & 0x1,
        (b >> 3) & 0x1,
        (b >> 2) & 0x1,
        (b >> 1) & 0x1,
        b & 0x1,
    ]
}

fn main() -> Result<(), Box<dyn Error>> {
    let (pk, ms, sigs) = puzzle_data();
    println!("Verifying existing messages...");
    let mut i = 0;
    // for (m, sig) in ms.iter().zip(sigs.iter()) {
    //     verify(pk, m, *sig);
    //     println!("{}/{}", i, ms.len());
    //     i = i + 1;
    // }
    println!("Done!");

    let m = "alxs".as_bytes();
    let hash = blake2s_simd::blake2s(m)
        .as_bytes()
        .to_vec()
        .into_iter()
        .flat_map(|b| explode_u8(&b))
        .map(|u| u as f32)
        .collect::<Vec<_>>();

    // println!("{:?}", m_b2hash);

    let ms_hashes = ms
        .iter()
        .flat_map(|m| {
            blake2s_simd::blake2s(m)
                .as_bytes()
                .to_vec()
                .into_iter()
                .flat_map(|b| explode_u8(&b))
                .map(|u| u as f32)
        })
        .collect::<Vec<_>>();

    // println!("{:?}", ms_hashes);

    // Solve A x = v
    let v = DMatrix::from_vec(256, 1, hash);
    let mx = DMatrix::from_vec(256, 256, ms_hashes);
    println!("About to");
    // println!("{}", mx.is_invertible());

    let hell_yeah = mx
        .lu()
        .solve(&v)
        .unwrap()
        .data
        .as_vec()
        .iter()
        .map(|h: &f32| F::from(*h * 50000000 as f32))
        .collect::<Vec<_>>();

    // let v: f32 = 1;
    // let d = v * 1000000;
    // let mx_inv = match mx.try_inverse() {
    //     Some(m) => m,
    //     None => return Ok(()),
    // };
    println!("{:?}", hell_yeah);

    // let hell_yeah = &v * &mx_inv;
    // println!("{:?}", hell_yeah);

    // let sigs_mx = DMatrix::from_row_iter(
    //     256,
    //     48,
    //     sigs.iter().map(|s| s.to_vec()).flatten().map(|u| u as i8),
    // );

    // let sig_m = hell_yeah * sigs_mx;

    let mut i = 0;
    let mut acc = G1Projective::zero();
    for (factor, sig) in hell_yeah.iter().zip(sigs.iter()) {
        let numer = match factor.numer() {
            Some(num) => num,
            None => return Ok(()),
        };
        let denom = match factor.denom() {
            Some(num) => num,
            None => return Ok(()),
        };

        //TODO this is wrong
        let mut res = sig.mul(*numer);
        // res -= sig.mul(*denom);

        if let fraction::Sign::Minus = factor.sign().unwrap() {
            res = -res;
        };

        acc += res;
    }

    let det: ark_bls12_381::Fq2 = field_new!(Fq, "50000000");
    acc.into_affine().verify(pk, m, acc.into());

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
