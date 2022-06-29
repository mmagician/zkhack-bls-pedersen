use std::{ops::Mul, process::Output};

use ark_bls12_381::{Fq, Fr, G1Affine, G1Projective, Parameters};
use ark_crypto_primitives::crh::{
    pedersen::{bytes_to_bits, Window},
    CRHScheme,
};
use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ff::{MontFp, Zero};
use bitvec::prelude::*;
use bls_pedersen::PUZZLE_DESCRIPTION;
use bls_pedersen::{bls::verify, hash::hash_to_curve};
use bls_pedersen::{data::puzzle_data, x::read_x};
use nalgebra::{ArrayStorage, SMatrix, U1};
use nalgebra::{Const, Matrix};
use nalgebra::{Dynamic, OMatrix, U127};
use prompt::{puzzle, welcome};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

pub type DynamicMatrix = OMatrix<f64, Dynamic, Dynamic>;
pub type DynamicVector = OMatrix<G1Affine, Dynamic, U1>;
pub type DynamicVectorB = OMatrix<f64, Dynamic, U1>;
#[derive(Debug, PartialEq)]
struct SigVector(Vec<G1Affine>);
struct MsgVector(Vec<u8>);

impl Mul<SigVector> for DynamicMatrix {
    type Output = SigVector;

    fn mul(self, rhs: SigVector) -> Self::Output {
        // let mut res = DynamicVector::zeros(self.nrows());
        let mut res = Vec::new();
        for i in 0..self.nrows() {
            let mut sum = G1Affine::zero();
            for j in 0..self.ncols() {
                // if the matrix element is non-zero, add the corresponding x = sk.[G_j] to the sum
                if self[(i, j)] == 1.0 {
                    sum += &rhs.0[j];
                }
            }
            res.push(sum);
            // res[i] = sum;
        }
        SigVector(res)
    }
}

impl Mul<DynamicVector> for MsgVector {
    type Output = G1Affine;

    fn mul(self, rhs: DynamicVector) -> Self::Output {
        unimplemented!()
        // let mut res = G1Affine::zero();
        // for i in 0..self.nrows() {
        //     res += self[(i, 0)] * rhs[i];
        // }
        // res
    }
}

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
    let target_message_hash = blake2s_simd::blake2s(message);
    // let (target_message_hash, _) = hash_to_curve(message);

    let ms_iter: Vec<f64> = ms
        .iter()
        .flat_map(|m| {
            let col: Vec<f64> = m
                .iter()
                .flat_map::<_, _>(|c| {
                    let bits: Vec<f64> = BitVec::<_, Msb0>::from_element(*c)
                        .iter()
                        .map(|b| f64::from(u8::from(*b)))
                        .collect();
                    bits
                })
                .collect();
            col
        })
        .collect();

    // we wish to solve:
    // m_0.[sk[G_0]] + ... + m_n.[sk.[G_n]] = B,
    // where B is the signature
    // so we're solving for Ax = B
    // A is the matrix of hashes of all input messages
    // x is the vector sk[G_0], ..., sk[G_n]
    // B is the vector of signatures
    // once we solve for vec x, we can compute a signature for any message
    // since signature on any message Y: S(Y) = Y_0.[sk[G_0]] + ... + Y_n.[sk.[G_n]] = Y * x
    println!("num of entries: {:?}", ms_iter.len());
    let M = DynamicMatrix::from_vec(256, 256, ms_iter);
    // println!("{:?}", M);

    let m: Vec<u8> = target_message_hash
        .as_bytes()
        .to_vec()
        .iter()
        .flat_map(|c| {
            let bits: Vec<u8> = BitVec::<_, Msb0>::from_element(*c)
                .iter()
                .map(|b| u8::from(*b))
                .collect();
            bits
        })
        .collect();
    println!("m1: {:?}", m);
    let m = bytes_to_bits(target_message_hash.as_bytes())
        .iter()
        .map(|b| (*b as u8) as f64)
        .collect::<Vec<f64>>();
    println!("m: {:?}", m);
    println!("target hash: {:?}", target_message_hash);
    // let m_vec = DynamicVectorB::from_vec(m.clone());

    // let x = M.lu().solve(&m_vec);
    println!("my msg hash: {:?}", m);
    // let rank = M.rank(0.0);
    // println!("rank: {:?}", rank);

    // let m_inv_determinant = M.determinant();
    // let mut inv = M.transpose().try_inverse().unwrap();
    // println!("{:?}", inv);
    // inv *= m_inv_determinant;
    // let x: DynamicVector = inv * SigVector(sigs);
    let x = read_x();
    // assert_eq!(SigVector(sigs), M * SigVector(x));

    // println!("{:?}", x);
    let mut accumulator = G1Projective::zero();
    for (x_i, sig) in x.iter().zip(sigs.iter()) {
        accumulator += sig.mul(*x_i);
    }
    // println!("sigs: {:?}", sigs);

    // let sig: G1Affine = MsgVector(target_message_hash) * x;

    verify(pk, message, accumulator.into_affine());
}
