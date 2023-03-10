use rand::{seq::SliceRandom, SeedableRng, Rng, distributions::Uniform};


pub(crate) fn mangler(input: &str, key: [u8; 32]) -> String {
    let input_bytes = input.as_bytes().to_owned();
    let mut input_bits = u64::from_ne_bytes(input_bytes.as_slice().try_into().unwrap());

    let mut bit_vec: Vec<u8> = vec![];
    for _ in 0..64 {
        bit_vec.push((input_bits & 1) as u8);
        input_bits >>= 1;
    }
    
    let mut seeded_rng = rand::rngs::StdRng::from_seed(key);

    let range = Uniform::from(0..=1 as u8);
    let mut rand_vec = (&mut seeded_rng).sample_iter(range).take(bit_vec.len()).collect();
    bit_vec.append(&mut rand_vec);
    
    bit_vec.shuffle(&mut seeded_rng);

    bit_vec.truncate(64);

    let mut output_bits: u64 = 0;
    for (i, bit) in bit_vec.iter().enumerate() {
        output_bits += (*bit as u64) << i;
    }
    return String::from_utf8(output_bits.to_ne_bytes().iter().map(|byte| byte%128).collect()).unwrap();
}