use anyhow::Result;
use p256::ecdsa::{SigningKey, Signature, signature::Signer};
use rand_core::OsRng;
use sha2::{Sha256, Digest};
use std::fs;
use p256::elliptic_curve::generic_array::GenericArray;

pub fn generate_keypair() -> Result<()> {
    let signing_key = SigningKey::random(&mut OsRng);
    let verify_key = signing_key.verifying_key();

    fs::write("private.key", signing_key.to_bytes())?;
    fs::write("public.key", verify_key.to_encoded_point(false).as_bytes())?;

    println!("Keys generated: private.key & public.key");
    Ok(())
}

pub fn sign(data: &[u8], private_key: &[u8]) -> Vec<u8> {
    let key_array = GenericArray::from_slice(private_key);
    let key = SigningKey::from_bytes(key_array).unwrap();
    let hash = Sha256::digest(data);
    let sig: Signature = key.sign(&hash);
    sig.to_der().as_bytes().to_vec()
}