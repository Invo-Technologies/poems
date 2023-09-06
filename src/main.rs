use clap::Parser;
use std::str::FromStr;
use dotenv::dotenv;
use std::env;
/// Simple program to handle custom poems commands
#[derive(Parser, Debug, Clone)]
#[clap(
    version = "2.3",
    author = "Your Name. <your.email@example.com>",
    about = "Handles custom poems commands"
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

    Environment
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
    
    let _my_variable = env::var("APPNAME").unwrap_or("default_value".to_string());
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
            // Call the registration binary
            std::process::Command::new("environment")
                .status()
                .expect("Failed to execute registration binary");
        }
        None => {
            println!("Please provide a valid subcommand. Use --help for more information.");
        }
    }
}
