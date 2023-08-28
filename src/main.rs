use clap::Parser;
use std::str::FromStr;
use dotenv::dotenv;
use std::env;

/// Simple program to handle custom poems commands
#[derive(Parser, Debug, Clone)]
#[clap(
    version = "2.0",
    author = "Dylan Kawalec <dkawalec@ourinvo.com>",
    about = "Encrypt inputs into a Zero Knowledge Proof Hash Mixer, and return the values and decrypt using decryption keys"
)]
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
    /// Sets up the environment
    Environment,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Decrypt" => Ok(Command::Decrypt),
            "Registration" => Ok(Command::Registration),
            "Environment" => Ok(Command::Environment),
            _ => Err(format!("Unknown command: {}", s)),
        }
    }
}

fn main() {
    dotenv().ok();
    let my_variable = env::var("APPNAME").unwrap_or("default_value".to_string());
    let args = PoemsCLI::parse();

    match args.cmd {
        Some(Command::Decrypt) => {
            // Call the decrypt binary
            std::process::Command::new("decrypt")
                .status()
                .expect("Failed to execute decrypt binary");
        }
        Some(Command::Registration) => {
            // Call the registration binary
            std::process::Command::new("registration")
                .status()
                .expect("Failed to execute registration binary");
        }
        Some(Command::Environment) => {
            // Call the environment binary
            std::process::Command::new("environment")
                .status()
                .expect("Failed to execute environment binary");
        }
        None => {
            println!("Please provide a valid subcommand. Use --help for more information.");
        }
    }
}
