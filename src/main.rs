use std::{
    env::Args,
    error,
    fmt::{Display, Write},
};

use sha512_hasher::hash_sha512;

fn main() -> Result<(), Box<dyn error::Error>> {
    let arguments: Args = std::env::args();

    // println!("Arguments: {:?}", arguments.collect::<Vec<String>>());

    let arguments: Vec<String> = parse_args(arguments.collect())?;

    for arg in &arguments {
        let hash: [u8; 64] = hash_sha512(arg.as_bytes());

        let hash_str = hash.iter().fold(String::new(), |mut acc, &byte| {
            acc.write_str(&format!("{:02x}", byte)).unwrap();
            acc
        });

        println!("\n{arg}: {hash_str}");
    }

    Ok(())
}

fn parse_args(arguments: Vec<String>) -> Result<Vec<String>, ParseError> {
    let args = arguments.into_iter().skip(1);

    args.clone().for_each(|arg| {
        if arg.starts_with("--") {
            let arg = arg.trim_start_matches("--");
            match arg {
                "help" => {
                    println!("\n{}\n", help_message());
                    std::process::exit(0);
                }
                "version" => {
                    println!(
                        "\nVersion: {}\n",
                        std::env::var("CARGO_PKG_VERSION").unwrap()
                    );
                    std::process::exit(0);
                }
                _ => {
                    eprintln!("{arg} --> {}", ParseError::Unknown);
                    std::process::exit(1);
                }
            }
        }
    });

    Ok(args.collect::<Vec<String>>())
}

#[derive(Debug)]
enum ParseError {
    Unknown,
}

impl std::error::Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Unknown => write!(f, "Unknown argument"),
        }
    }
}

fn help_message() -> String {
    let mut help = String::new();
    help.push_str("Usage: hash [OPTIONS] [ARGS]\n");
    help.push_str("Hashes the given arguments using SHA-512\n");
    help.push_str("Options:\n");
    help.push_str("  --help     Prints help information\n");
    help.push_str("  --version  Prints version information\n");
    help
}
