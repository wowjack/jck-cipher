
pub(crate) fn mangler(mut input: u64, seeded_rng: rand::rngs::StdRng) -> u64 {
    let mut bit_vec: Vec<u8> = vec![];

    print!("bit vec of {input}: ");

    for _ in 0..64 {
        bit_vec.push((input & 1) as u8);
        input >>= 1;
    }
    println!("{:?}", bit_vec);
    return 0;
}