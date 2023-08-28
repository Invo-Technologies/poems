use clap::Parser;
use std::str::FromStr;
use dotenv::dotenv;
use std::env;

/// Simple program to handle custom poems commands
#[derive(Parser, Debug, Clone)]
#[clap(
    version = "2.0.2",
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
    Register,
    /// Sets up the environment
    Env,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Decrypt" => Ok(Command::Decrypt),
            "Register" => Ok(Command::Register),
            "Env" => Ok(Command::Env),
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
        Some(Command::Register) => {
            // Call the registration binary
            std::process::Command::new("register")
                .status()
                .expect("Failed to execute registration binary");
        }
        Some(Command::Env) => {
            // Call the environment binary
            std::process::Command::new("env")
                .status()
                .expect("Failed to execute environment binary");
        }
        None => {
            println!("Please provide a valid subcommand. Use --help for more information.");
        }
    }
}
