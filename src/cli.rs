use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Layout CLI", version, about = "Structured Rust CLI Layout")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Greet someone
    Greet {
        #[arg(short, long)]
        name: String,
    },
    /// Add two numbers
    Math {
        #[arg(short)]
        a: i32,
        #[arg(short)]
        b: i32,
    },
}

