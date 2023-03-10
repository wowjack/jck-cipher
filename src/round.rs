use crate::{BLOCK_SIZE, mangler::mangler};


pub(crate) fn encryption_round(input: &mut String, round_key: [u8; 32]) {
    let mut input_copy = input.clone();
    let (left, right) = input_copy.split_at_mut((BLOCK_SIZE/2) as usize);
    input.replace_range(..input.len()/2, right); //right half of input becomes left half of output

    let mangled = mangler(right, round_key);

    input.replace_range(input.len()/2.., &xor_strings(left, &mangled));
}

pub(crate) fn decryption_round(input: &mut String, round_key: [u8; 32]) {
    let mut input_copy = input.clone();
    let (left, right) = input_copy.split_at_mut((BLOCK_SIZE/2) as usize);
    input.replace_range(input.len()/2.., left);

    let mangled = mangler(left, round_key);

    input.replace_range(..input.len()/2, &xor_strings(right, &mangled))
}




fn xor_strings(s1: &str, s2: &str) -> String {
    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();

    let mut xor_bytes: Vec<u8> = vec![];
    for (b1, b2) in s1_bytes.iter().zip(s2_bytes.iter()) {
        xor_bytes.push(b1 ^ b2);
    }
    
    return String::from_utf8(xor_bytes).unwrap();
}