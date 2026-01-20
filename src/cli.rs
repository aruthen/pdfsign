use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pdfsign")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    GenerateKey,
    Sign {
        #[arg(long)]
        input: String,

        #[arg(long)]
        output: String,

        #[arg(long)]
        key: String,
    },
}
