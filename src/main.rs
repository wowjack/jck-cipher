//CLI application for encrypting & decrypting strings and files
use jck_cipher::*;

fn main() {
    let plain_text = "Hello";
    let key = u128::from_be_bytes([12, 34, 52, 78, 90, 66, 123, 94, 53, 86, 243, 12, 43, 26, 93, 154]);
    let rounds = 1000;

    println!("Plain text:\n\"{}\"\n", plain_text);

    let cipher_text = encrypt(&plain_text, key, rounds);
    println!("Cipher text:\n\"{}\"\n", cipher_text);

    let decrypted_text = decrypt(&cipher_text, key, rounds);

    println!("Decrypted text:\n\"{}\"\n", decrypted_text);
}