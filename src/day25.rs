// Oh man, I don't remember anything about cryptography. :-(

pub fn run() {
    let input_bytes = include_bytes!("day25_input");
    let input_string = String::from_utf8(input_bytes.to_vec()).unwrap();
    let mut lines = input_string.lines();
    let pub1 = lines.next().unwrap().parse::<usize>().unwrap();
    let pub2 = lines.next().unwrap().parse::<usize>().unwrap();

    part1(pub1, pub2);
}

fn compute_public_key(loop_size: usize, subject_number: usize) -> usize {
    let mut value = 1;
    for _ in 0..loop_size {
        value *= subject_number;
        value %= 20201227;
    }
    value
}

fn brute_force_loop_size(pub_key: usize, subject_number: usize) -> usize {
    let mut loop_size = 1;
    let mut value = 1;
    loop {
        value *= subject_number;
        value %= 20201227;

        // Way too slow:
        // if compute_public_key(loop_size, subject_number) == pub_key {
        if value == pub_key {
            break;
        }
        loop_size += 1;
    }
    loop_size
}

fn part1(pub1: usize, pub2: usize) {
    let loop_size1 = brute_force_loop_size(pub1, 7);
    let loop_size2 = brute_force_loop_size(pub2, 7);
    let key1 = compute_public_key(loop_size1, pub2);
    let key2 = compute_public_key(loop_size2, pub1);
    assert_eq!(key1, key2);

    println!("Day 25, part 1: {}", key1);
}
