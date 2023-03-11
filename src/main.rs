use std::{process::exit, io::Write};

//CLI application for encrypting & decrypting strings and files
use jck_cipher::*;
use clap::{Parser, Subcommand, Args, ValueEnum};

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
    output: std::path::PathBuf,
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

}

fn decrypt_file(args: EncAndDecArgs) {

}

fn generate_key(mut args: KeygenArgs) {
    let path = std::path::PathBuf::from(args.file);
    if path.exists() {
        println!("FILE {:?} ALREADY EXISTS!!!!!!!!!!!!", path);
        print!("Continuing will overwrite this file. Do you want to continue? [y/n] ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() != "y" {
            println!("Exiting...");
            exit(0);
        }
        std::fs::remove_file(path.clone()).unwrap();
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