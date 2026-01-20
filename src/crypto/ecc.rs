// Import library yang diperlukan
use anyhow::Result;  // Untuk error handling yang fleksibel
use p256::ecdsa::{SigningKey, Signature, signature::Signer}; // ECDSA P-256 signing
use sha2::{Sha256, Digest}; // SHA-256 hashing
use std::fs;  // Untuk file operations

/// Fungsi untuk membuat pasangan kunci ECDSA P-256
/// Output: File "private.key" dan "public.key"
pub fn generate_keypair() -> Result<()> {
    // Buat kunci privat secara random menggunakan OS random number generator
    let signing_key = SigningKey::random(&mut rand_core::OsRng);
    
    // Dari kunci privat, turunkan kunci publik
    let verify_key = signing_key.verifying_key();

    // Simpan kunci privat ke file "private.key" dalam format bytes
    fs::write("private.key", signing_key.to_bytes())?;
    
    // Simpan kunci publik ke file "public.key" dalam format encoded point
    // Parameter false = format uncompressed (76 bytes)
    fs::write("public.key", verify_key.to_encoded_point(false).as_bytes())?;

    // Tampilkan pesan sukses ke user
    println!("Keys generated: private.key & public.key (ECDSA P-256)");
    Ok(())
}

/// Fungsi untuk menandatangani data dengan ECDSA P-256
/// Parameter:
///   - data: data yang akan ditandatangani (PDF bytes)
///   - private_key: kunci privat dalam format bytes
/// Return: signature dalam format DER encoding
pub fn sign(data: &[u8], private_key: &[u8]) -> Vec<u8> {
    // Buat signing key langsung dari bytes privat key
    // SigningKey::from_bytes() menerima slice dengan ukuran fixed 32 bytes (256 bit)
    let key = SigningKey::from_bytes(private_key.into()).unwrap();
    
    // Hash data menggunakan SHA-256
    // Ini menghasilkan 32 bytes digest
    let hash = Sha256::digest(data);
    
    // Tanda tangani hash dengan signing key menggunakan ECDSA
    let sig: Signature = key.sign(&hash);
    
    // Konversi signature ke format DER dan kembalikan sebagai Vec<u8>
    // DER adalah format standar untuk encoding digital signature
    sig.to_der().as_bytes().to_vec()
}