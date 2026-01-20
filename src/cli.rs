// Import macro-macro dari clap untuk parsing command-line arguments
use clap::{Parser, Subcommand};

/// Struktur utama untuk parsing command-line arguments
/// Parser trait akan men-generate kode parsing otomatis
#[derive(Parser)]
#[command(name = "pdfsign")] // Nama program
pub struct Cli {
    #[command(subcommand)] // Sub-command untuk menjalankan perintah berbeda
    pub command: Commands,
}

/// Enum untuk menyimpan berbagai command yang tersedia
#[derive(Subcommand)]
pub enum Commands {
    /// Command 1: generate-key
    /// Fungsi: Membuat pasangan kunci ECC P-256 (publik & privat)
    GenerateKey,
    
    /// Command 2: sign
    /// Fungsi: Menandatangani file PDF dengan ECDSA
    Sign {
        /// Path file PDF yang akan ditandatangani
        #[arg(long)]
        input: String,

        /// Path file PDF output hasil penandatanganan
        #[arg(long)]
        output: String,

        /// Path file kunci privat (private.key)
        #[arg(long)]
        key: String,

        /// Nama penandatangan (default: "pdfsign-cli")
        #[arg(long, default_value = "pdfsign-cli")]
        name: String,

        /// Alasan penandatanganan (default: "Digitally signed")
        #[arg(long, default_value = "Digitally signed")]
        reason: String,

        /// Lokasi penandatanganan (default: kosong)
        #[arg(long, default_value = "")]
        location: String,

        /// Informasi kontak penandatangan (default: kosong)
        #[arg(long, default_value = "")]
        contact_info: String,
    },
}
