use generate_keys::generate_keys;
use round::{encryption_round, decryption_round};

mod mangler; //mangler function used in each encryption round
mod generate_keys; //generates round keys used by the mangler function
mod round; //function to be used for each round of encryption

const BLOCK_SIZE: u8 = 16; //16 bytes / 128 bits per block


pub fn encrypt(input: &str, key: u128, rounds: usize) -> String {
    //generate list of round keys
    let round_keys = generate_keys(key, rounds);


    //apply padding
    let mut input_bytes = Vec::from(input.as_bytes());
    let padding_bytes: u8 = BLOCK_SIZE - (input_bytes.len() as u128 % BLOCK_SIZE as u128) as u8;
    input_bytes.extend(vec![padding_bytes; padding_bytes.into()]);

    
    
    //encrypt
    let mut output_buffer: Vec<String> = vec![];
    for chunk in input_bytes.chunks_exact(BLOCK_SIZE.into()) {
        output_buffer.push(encrypt_block(chunk, &round_keys));
    }

    return output_buffer.concat();
}

fn encrypt_block(input: &[u8], round_keys: &Vec<[u8; 32]>) -> String {
    let mut block_string = String::from_utf8(input.to_vec()).unwrap();
    for key in round_keys.iter() {
        encryption_round(&mut block_string, *key);
    }
    return block_string;
}


pub fn decrypt(input: &str, key: u128, rounds: usize) -> String {
    //generate list of round keys
    let mut round_keys = generate_keys(key, rounds);
    round_keys.reverse();

    //decrypt
    let mut output_buffer: Vec<String> = vec![];
    for block in input.as_bytes().chunks_exact(BLOCK_SIZE.into()) {
        output_buffer.push(decrypt_block(block, &round_keys));
    }

    //remove padding
    let mut output = output_buffer.concat();
    let x = output.as_bytes()[output.len()-1];
    output.truncate(output.len() - x as usize);

    return output;
}

fn decrypt_block(input: &[u8], round_keys: &Vec<[u8; 32]>) -> String {
    let mut block_string = String::from_utf8(input.to_vec()).unwrap();
    for key in round_keys {
        decryption_round(&mut block_string, *key);
    }
    return block_string;
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEFAULT_STRING: &str = "string";
    const DEFAULT_KEY: u128 = 123456789101112;
    const DEFAULT_ROUNDS: usize = 32;

    #[test]
    fn test1() {
        let cipher_text = encrypt(DEFAULT_STRING, DEFAULT_KEY, DEFAULT_ROUNDS);
        let plain_text = decrypt(&cipher_text, DEFAULT_KEY, DEFAULT_ROUNDS);
        assert_eq!(DEFAULT_STRING, plain_text);
    }

    #[test]
    fn test2() {
        let input = "hello from the other side";
        let cipher_text = encrypt(input, DEFAULT_KEY, DEFAULT_ROUNDS);
        let plain_text = decrypt(&cipher_text, DEFAULT_KEY, DEFAULT_ROUNDS);
        assert_eq!(input, plain_text);
    }

    #[test]
    fn test3() {
        let input = "really really long string\n with newlines and other things.\n
                           This might end up being a few blocks long\nbut thats okay because the algorithm\n
                           is equipped to handle as many blocks as you throw at it.";
        let cipher_text = encrypt(input, DEFAULT_KEY, DEFAULT_ROUNDS);
        let plain_text = decrypt(&cipher_text, DEFAULT_KEY, DEFAULT_ROUNDS);
        assert_eq!(input, plain_text);
    }
}
