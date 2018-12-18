// -*- mode: rust; -*-
//
// This file is part of ed25519-dalek.
// Copyright (c) 2018 Isis Lovecruft
// See LICENSE for licensing information.
//
// Authors:
// - Isis Agora Lovecruft <isis@patternsinthevoid.net>

#[macro_use]
extern crate criterion;
extern crate schnorr_dalek;
extern crate rand;
extern crate sha3;
// extern crate sha2;

use criterion::Criterion;

mod ed25519_benches {
    use super::*;
    use schnorr_dalek::{SecretKey, Keypair, PublicKey, Signature, verify_batch};
    use schnorr_dalek::context::signing_context;
    use rand::thread_rng;
    use rand::ThreadRng;
	use sha3::Shake128;
    // use sha2::Sha512;

    // TODO: fn sign_mini(c: &mut Criterion)

    fn sign(c: &mut Criterion) {
        let mut csprng: ThreadRng = thread_rng();
        let keypair: Keypair = Keypair::generate(&mut csprng);
        let msg: &[u8] = b"";

		let ctx = signing_context::<Shake128>(b"this signature does this thing");
        c.bench_function("Ed25519 signing", move |b| {
                         b.iter(| | keypair.sign(&ctx, msg))
        });
    }

    fn verify(c: &mut Criterion) {
        let mut csprng: ThreadRng = thread_rng();
        let keypair: Keypair = Keypair::generate(&mut csprng);
        let msg: &[u8] = b"";
		let ctx = signing_context::<Shake128>(b"this signature does this thing");
        let sig: Signature = keypair.sign(&ctx, msg);
        
        c.bench_function("Ed25519 signature verification", move |b| {
                         b.iter(| | keypair.verify(&ctx, msg, &sig))
        });
    }

    fn verify_batch_signatures(c: &mut Criterion) {
        static BATCH_SIZES: [usize; 8] = [4, 8, 16, 32, 64, 96, 128, 256];

        c.bench_function_over_inputs(
            "Ed25519 batch signature verification",
            |b, &&size| {
                let mut csprng: ThreadRng = thread_rng();
                let keypairs: Vec<Keypair> = (0..size).map(|_| Keypair::generate(&mut csprng)).collect();
                let msg: &[u8] = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
                let messages: Vec<&[u8]> = (0..size).map(|_| msg).collect();
				let ctx = signing_context::<Shake128>(b"this signature does this thing");
                let signatures:  Vec<Signature> = keypairs.iter().map(|key| key.sign(&ctx, &msg)).collect();
                let public_keys: Vec<PublicKey> = keypairs.iter().map(|key| key.public).collect();

                b.iter(|| verify_batch(&ctx, &messages[..], &signatures[..], &public_keys[..]));
            },
            &BATCH_SIZES,
        );
    }

    fn key_generation(c: &mut Criterion) {
        let mut csprng: ThreadRng = thread_rng();

        c.bench_function("Ed25519 keypair generation", move |b| {
                         b.iter(| | Keypair::generate(&mut csprng))
        });
    }

    criterion_group!{
        name = ed25519_benches;
        config = Criterion::default();
        targets =
            sign,
            verify,
            verify_batch_signatures,
            key_generation,
    }
}

criterion_main!(
    ed25519_benches::ed25519_benches,
);
