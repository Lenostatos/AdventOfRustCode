use md5::{Digest, Md5};

fn main() {
    let mut hasher = Md5::new();

    let mut found_hash = false;
    let mut number: usize = 1;
    let mut result_hex = String::from("");

    while !found_hash && number < 10000000 {
        // process input
        hasher.update(b"ckczppom");
        hasher.update(number.to_string().into_bytes());

        // acquire hash
        let result = hasher.finalize_reset();
        result_hex = hex::encode(result);

        number += 1;

        if result_hex.starts_with("000000") {
            found_hash = true;
            number -= 1;
        }
    }

    println!("hash: {result_hex} | number: {number}");
}
