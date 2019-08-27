extern crate md5;

// Under the hood, md5::Digest is [u8; 16]

fn find_hash(input: &str, n: usize) -> u64 {
    let mut counter = 0;
    loop {
        let hash_input = format!("{}{}", input, counter);
        let digest = md5::compute(&hash_input);
        if begins_with_n_zeroes(&digest, n) {
            return counter;
        }
        counter += 1;
    }
}

fn begins_with_n_zeroes(digest: &md5::Digest, n: usize) -> bool {
    assert!(n <= 16);
    if n == 0 {
        return digest[0] != 0;
    }
    let mid: usize = n / 2;
    for i in 0..mid {
        if digest[i] != 0 {
            return false;
        }
    }
    // If n is odd, check the upper half of the mid-th byte.
    if n % 2 == 1 {
        return digest[mid] & 0xf0 == 0;
    }
    return true;
}

fn main() {
    println!("{}", find_hash("bgvyzdsv", 5));
    println!("{}", find_hash("bgvyzdsv", 6));
}
