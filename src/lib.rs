pub mod data;

use ark_bls12_381::{Bls12_381, G1Affine, G1Projective, G2Affine};
use ark_crypto_primitives::crh::{
    pedersen::{Window, CRH},
    CRH as CRHScheme,
};
use ark_ec::{AffineCurve, PairingEngine};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

use std::ops::Neg;

use ark_ff::One;

#[derive(Clone)]
struct ZkHackPedersenWindow {}

impl Window for ZkHackPedersenWindow {
    const WINDOW_SIZE: usize = 1;
    const NUM_WINDOWS: usize = 256;
}

pub fn verify(pk: G2Affine, msg: &[u8], sig: G1Affine) {
    let (_, h) = hash_to_curve(msg);
    assert!(Bls12_381::product_of_pairings(&[
        (
            sig.into(),
            G2Affine::prime_subgroup_generator().neg().into()
        ),
        (h.into(), pk.into()),
    ])
    .is_one());
}

pub fn hash_to_curve(msg: &[u8]) -> (Vec<u8>, G1Affine) {
    let rng_pedersen = &mut ChaCha20Rng::from_seed([
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1,
    ]);
    let parameters = CRH::<G1Projective, ZkHackPedersenWindow>::setup(rng_pedersen).unwrap();
    let b2hash = blake2s_simd::blake2s(msg);
    (
        b2hash.as_bytes().to_vec(),
        CRH::<G1Projective, ZkHackPedersenWindow>::evaluate(&parameters, b2hash.as_bytes())
            .unwrap(),
    )
}
