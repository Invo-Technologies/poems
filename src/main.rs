use clap::Parser;
use std::str::FromStr;


/// Simple program to handle custom poems commands
#[derive(Parser, Debug, Clone)]
#[clap(version = "1.0", author = "Your Name. <your.email@example.com>", about = "Handles custom poems commands")]
struct PoemsCLI {
    #[clap(subcommand)]
    cmd: Option<Command>,
}

#[derive(Parser, Debug, Clone)]
enum Command {
    /// Executes the decryption program
    Decrypt,
    /// Executes the registration/key generation without the decryption program
    Registration,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Decrypt" => Ok(Command::Decrypt),
            "Registration" => Ok(Command::Registration),
            _ => Err(format!("Unknown command: {}", s)),
        }
    }
}

fn main() {
    let args = PoemsCLI::parse();

    match args.cmd {
        Some(Command::Decrypt) => {
            // Call the decrypt binary
            std::process::Command::new("poems-decrypt")
                .status()
                .expect("Failed to execute decrypt binary");
        }
        Some(Command::Registration) => {
            // Call the registration binary
            std::process::Command::new("poems-registration")
                .status()
                .expect("Failed to execute registration binary");
        }
        None => {
            println!("Please provide a valid subcommand. Use --help for more information.");
        }
    }
}

