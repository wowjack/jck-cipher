use crate::{BLOCK_SIZE, mangler::mangler};


pub(crate) fn encryption_round(input: &mut String, seeded_rng: rand::rngs::StdRng) {
    let mut input_copy = input.clone();
    let (left, right) = input_copy.split_at_mut((BLOCK_SIZE/2) as usize);
    input.insert_str(0, right); //right half of input becomes left half of output

    let right_bytes = right.as_bytes().to_owned();
    let right_bits = u64::from_ne_bytes(right_bytes.as_slice().try_into().unwrap());

    mangler(right_bits, seeded_rng);

    input.insert_str((BLOCK_SIZE/2) as usize, right);
}

pub(crate) fn decryption_round(input: &mut String, seeded_rng: rand::rngs::StdRng) {

}