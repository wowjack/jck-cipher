

//round keys will repeat every 256 rounds
pub(crate) fn generate_keys(main_key: u128, rounds: u32) -> Vec<[u8; 32]> {
    let mut round_keys: Vec<[u8; 32]> = vec![];
    
    //Split the main key into four chunks
    let mut byte_list: [u8; 16] = main_key.to_be_bytes();
    let (chunk_a, chunk_b) = byte_list.split_at_mut(8);
    let (chunk_1, chunk_2) = chunk_a.split_at_mut(4);
    let (chunk_3, chunk_4) = chunk_b.split_at_mut(4);
    let mut chunks = [chunk_1, chunk_2, chunk_3, chunk_4];

    for i in 0..rounds { //apply rotate function each round then combine to form key
        do_rotates(&mut chunks, i);
        let mut xlchunk = chunks.concat();
        xlchunk.extend_from_within(0..);
        round_keys.push(xlchunk.try_into().unwrap());
    }
    return round_keys;
}

fn do_rotates(chunks: &mut [&mut [u8]; 4], round_num: u32) {
    if round_num % 128 == 0 {
        chunks.reverse();
    }

    if round_num % 32 == 0 {
        chunks.rotate_left(1);
    }

    if round_num%8 == 0 {//on rounds divisible by eight, rotate each 4 byte chunk once to the left
        chunks.iter_mut().for_each(|chunk| chunk.rotate_left(1));
    } 

    //rotate each byte once to the left
    chunks.iter_mut().for_each(|chunk| {
        chunk.iter_mut().for_each(|byte| {
            *byte = byte.rotate_left(1);
        })
    });
}