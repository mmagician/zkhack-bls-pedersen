use ark_bls12_381::{G1Affine, G1Projective};
use ark_crypto_primitives::crh::{
    pedersen::{Window, CRH},
    CRH as CRHScheme,
};
use bitvec::prelude::*;
use bls_pedersen::data::puzzle_data;
use bls_pedersen::PUZZLE_DESCRIPTION;
use bls_pedersen::{bls::verify, hash::hash_to_curve};
use nalgebra::{U127, OMatrix, Dynamic};
use nalgebra::{ArrayStorage, SMatrix};
use nalgebra::{Const, Matrix};
use prompt::{puzzle, welcome};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

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
    let (_, m) = hash_to_curve(message);

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

    println!("num of entries: {:?}", ms_iter.len());
    type DynamicMatrix = OMatrix<f32, Dynamic, Dynamic>;
    let M = DynamicMatrix::from_vec(256, 256, ms_iter);
    let rank = M.rank(0.0);
    println!("{:?}", rank);

    let inv = M.try_inverse().unwrap();
    println!("{:?}", inv);
    // verifypk, message, sig);
}
