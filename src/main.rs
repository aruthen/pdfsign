// Deklarasi modul-modul yang digunakan dalam project
mod cli;      // Command-line interface (parsing arguments)
mod crypto;   // Cryptography module (ECC signing)
mod pdf;      // PDF manipulation module

use clap::Parser;      // Parser untuk command-line arguments
use anyhow::Result;    // Result type untuk error handling yang fleksibel
use cli::{Cli, Commands}; // Import struktur CLI dan enum Commands

/// Fungsi utama program
/// Menangani logika dasarnya:
/// 1. Parse command-line arguments dari user
/// 2. Jalankan perintah yang sesuai (generate-key atau sign)
fn main() -> Result<()> {
    // Parse command-line arguments yang diberikan user
    let cli = Cli::parse();

    // Cocokkan command yang dipilih user
    match cli.command {
        // Perintah: generate-key
        // Membuat pasangan kunci publik-privat ECDSA P-256
        Commands::GenerateKey => crypto::ecc::generate_keypair()?,
        
        // Perintah: sign
        // Menandatangani file PDF dengan kunci privat
        Commands::Sign { input, output, key, name, reason, location, contact_info } => {
            // Buat struktur metadata untuk signature
            let metadata = pdf::sign::SignatureMetadata {
                name,           // Nama penandatangan
                reason,         // Alasan penandatanganan
                location,       // Lokasi penandatanganan
                contact_info,   // Informasi kontak penandatangan
            };
            // Panggil fungsi untuk menandatangani PDF
            pdf::sign::sign_pdf(&input, &output, &key, metadata)?
        }
    }

    // Kembalikan Ok jika tidak ada error
    Ok(())
}