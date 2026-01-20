mod cli;
mod crypto;
mod pdf;

use clap::Parser;
use anyhow::Result;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::GenerateKey => crypto::ecc::generate_keypair()?,
        Commands::Sign { input, output, key } => {
            pdf::sign::sign_pdf(&input, &output, &key)?
        }
    }

    Ok(())
}