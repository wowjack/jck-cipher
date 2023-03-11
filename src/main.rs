use std::{process::exit, io::{Write, stdin, stdout}};

//CLI application for encrypting & decrypting strings and files
use jck_cipher::*;
use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
/// A DES style encryption and decryption tool
struct Arguments {
    #[command(subcommand)]
    command: Action,
}

#[derive(Subcommand)]
#[allow(non_camel_case_types)]
enum Action {
    encrypt(EncAndDecArgs),
    decrypt(EncAndDecArgs),
    keygen(KeygenArgs)
}

#[derive(Args)]
struct EncAndDecArgs {
    #[arg(short, long, value_name = "JCK key file")]
    keyfile: std::path::PathBuf,

    #[arg(short, long, value_name = "Input ciphertext file")]
    input: std::path::PathBuf,

    #[arg(short, long, value_name = "Output file")]
    output: Option<std::path::PathBuf>,

    #[arg(short, long, default_value_t = 16)]
    rounds: usize
}

#[derive(Args)]
struct KeygenArgs {
    #[arg(short, long, default_value_t = String::from("key.JCK"))]
    file: String
}

fn main() {
    let args = Arguments::parse();
    match args.command {
        Action::encrypt(args) => encrypt_file(args),
        Action::decrypt(args) => decrypt_file(args),
        Action::keygen(args) => generate_key(args)
    }
}


fn encrypt_file(args: EncAndDecArgs) {
    validate_args(&args);
    let key = get_key(&args);
    let input = get_input_file_contents(&args);
    let output = encrypt(&input, key, args.rounds);
    match args.output {
        None => {
            println!("{output}");
        },
        Some(path) => {
            std::fs::write(path, output).unwrap();
        }
    }
}

fn decrypt_file(args: EncAndDecArgs) {
    validate_args(&args);
    let key = get_key(&args);
    let input = get_input_file_contents(&args);
    let output = decrypt(&input, key, args.rounds);
    match args.output {
        None => {
            println!("{output}");
        },
        Some(path) => {
            std::fs::write(path, output).unwrap();
        }
    }
}

fn generate_key(args: KeygenArgs) {
    let path = std::path::PathBuf::from(args.file);
    if path.is_dir() {
        println!("Cannot write to directory");
        exit(0);
    }
    if path.exists() {
        println!("FILE {:?} ALREADY EXISTS!!!!!!!!!!!!", path);
        print!("Continuing will overwrite this file. Do you want to continue? [y/n] ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() != "y" {
            println!("Exiting...");
            exit(0);
        }
    }
    println!("Creating keyfile {:?}", path);
    let key: u128 = rand::random();
    match std::fs::write(path, key.to_string()) {
        Ok(_) => (),
        Err(error) => {
            println!("Failed to create file: {error}");
        }
    }
}

fn validate_args(args: &EncAndDecArgs) {
    if args.input.exists() == false || args.input.is_dir() {
        println!("Input file does not exist");
        println!("Exiting...");
        exit(0);
    }
    if args.keyfile.exists() == false || args.keyfile.is_dir() {
        println!("Key file does not exist");
        println!("Exiting...");
        exit(0);
    }
    if let Some(path) = &args.output {
        if path.is_dir() {
            println!("Output file does not exist");
            exit(0);
        }
        if path.exists() {
            println!("OUTPUT FILE {:?} ALREADY EXISTS!!!!!!!!!!!!", path);
            print!("Continuing will overwrite this file. Do you want to continue? [y/n] ");
            stdout().flush().unwrap();
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            if input.trim().to_lowercase() != "y" {
                println!("Exiting...");
                exit(0);
            }
        }
    }
}

fn get_key(args: &EncAndDecArgs) -> u128 {
    match std::fs::read(args.keyfile.clone()) {
        Ok(key_vec) => {
            u128::from_str_radix(&String::from_utf8(key_vec).unwrap(), 10).unwrap()
        },
        Err(error) => {
            println!("Error reading key from file: {error}");
            exit(0);
        }
    }
}

fn get_input_file_contents(args: &EncAndDecArgs) -> String {
    match std::fs::read_to_string(args.input.clone()) {
        Ok(contents) => contents,
        Err(error) => {
            println!("Error reading from input file: {error}");
            exit(0);
        }
    }
}

