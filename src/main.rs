//CLI application for encrypting & decrypting strings and files
use jck_cipher::*;

fn main() {
    let key = u128::from_be_bytes([12, 34, 52, 78, 90, 66, 123, 94, 53, 86, 243, 12, 43, 26, 93, 154]);
    jck_cipher::encrypt("hello", key, 64, 1000);
}