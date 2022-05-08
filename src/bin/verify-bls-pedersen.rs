use ark_bls12_381::{G1Affine, G1Projective};
use ark_crypto_primitives::crh::{
    pedersen::{Window, CRH},
    CRH as CRHScheme,
};
use bitvec::prelude::*;
use bls_pedersen::data::puzzle_data;
use bls_pedersen::PUZZLE_DESCRIPTION;
use bls_pedersen::{bls::verify, hash::hash_to_curve};
use nalgebra::SMatrix;
use prompt::{puzzle, welcome};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

#[macro_use]
extern crate ndarray;
extern crate ndarray_linalg;

use ndarray::arr2;
use ndarray::prelude::*;
use ndarray::{Array, Array1};
use ndarray_linalg::Solve;

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    let (pk, ms, sigs) = puzzle_data();
    // for (m, sig) in ms.iter().zip(sigs.iter()) {
    //     verify(pk, m, *sig);
    // }

    let rng_pedersen = &mut ChaCha20Rng::from_seed([
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1,
    ]);
    let message = "a&m".as_bytes();
    let (m_bytes, m_proj) = hash_to_curve(message);

    let ms_iter: Vec<f32> = ms
        .iter()
        .flat_map(|m| {
            let col: Vec<f32> = m
                .iter()
                .flat_map::<_, _>(|c| {
                    let bits: Vec<f32> = BitVec::<_, Msb0>::from_element(*c)
                        .iter()
                        .map(|b| f32::from(u8::from(*b)))
                        .collect();
                    bits
                })
                .collect();
            col
        })
        .collect();

    println!("{:?}", ms_iter);
    // let M = SMatrix::<f32, 256, 256>::from_vec(ms_iter);

    let m_bits: Vec<f32> = m_bytes
        .iter()
        .flat_map::<_, _>(|c| {
            let bits: Vec<f32> = BitVec::<_, Msb0>::from_element(*c)
                .iter()
                .map(|b| f32::from(u8::from(*b)))
                .collect();
            bits
        })
        .collect();

    let a = Array::from_shape_vec((256, 256), ms_iter).unwrap();
    let b: Array1<f32> = m_bits.into();

    let x = a.solve_into(b).unwrap();

    println!("{:?}", x);
    // let inv = M.rank();
    // println!("{:?}", inv);
    // try_inverse().unwrap();
    // verifypk, message, sig);
}
