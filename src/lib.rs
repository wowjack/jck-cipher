use generate_keys::generate_keys;

mod mangler; //mangler function used in each encryption round
mod generate_keys; //generates round keys used by the mangler function
mod round; //function to be used for each round of encryption
mod ebox; //expansion box used by mangler function
mod sbox; //substitute and shrink box used by mangler function

pub fn encrypt(input: &str, key: u128, block_size: u32, rounds: u32) -> &str {
    //generate list of round keys
    let round_keys = generate_keys(key, rounds);

    

    //pass input through round function however many times specified by rounds


    return input;
}

pub fn decrypt(input: &str, key: u128, block_size: u32, rounds: u32) -> &str {

    return input;
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEFAULT_STRING: &str = "string";
    const DEFAULT_KEY: u128 = 0;
    const DEFAULT_BLOCK_SIZE: u32 = 0;
    const DEFAULT_ROUNDS: u32 = 16;

    #[test]
    fn test_encrypt() {
        let result = encrypt(DEFAULT_STRING, DEFAULT_KEY, DEFAULT_BLOCK_SIZE, DEFAULT_ROUNDS);
        assert_eq!(result, DEFAULT_STRING);
    }

    #[test]
    fn test_decrypt() {
        let result = decrypt(DEFAULT_STRING, DEFAULT_KEY, DEFAULT_BLOCK_SIZE, DEFAULT_ROUNDS);
        assert_eq!(result, DEFAULT_STRING);
    }
}
