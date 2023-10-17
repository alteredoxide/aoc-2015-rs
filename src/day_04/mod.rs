//! --- Day 4: The Ideal Stocking Stuffer ---
//! Santa needs help mining some AdventCoins (very similar to bitcoins) to use as gifts
//! for all the economically forward-thinking little girls and boys.
//! 
//! To do this, he needs to find MD5 hashes which, in hexadecimal, start with at least
//! five zeroes. The input to the MD5 hash is some secret key (your puzzle input, given
//! below) followed by a number in decimal. To mine AdventCoins, you must find Santa the
//! lowest positive number (no leading zeroes: 1, 2, 3, ...) that produces such a hash.
//! 
//! For example:
//! 
//! - If your secret key is abcdef, the answer is 609043, because the MD5 hash of
//!   abcdef609043 starts with five zeroes (000001dbbfa...), and it is the lowest such
//!   number to do so.
//! - If your secret key is pqrstuv, the lowest number it combines with to make an MD5
//!   hash starting with five zeroes is 1048970; that is, the MD5 hash of pqrstuv1048970
//!   looks like 000006136ef....

use std::path::Path;

use md5;


fn load_input() -> std::io::Result<String> {
    let file_path = Path::new(file!())
        .parent()
        .expect("file not found")
        .join("input.txt");
    std::fs::read_to_string(file_path)
}


fn part_1(input: &str) -> usize {
    let mut i: usize = 0;
    println!("{}", input);
    loop {
        let combined = format!("{input}{i}");
        let digest = md5::compute(&combined);
        let hex_str = format!("{:x}", digest);

        if hex_str.starts_with("00000") {
            return i;
        }

        i += 1;
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let input = super::load_input().unwrap();
        let output = super::part_1(input.trim());
        assert_eq!(output, 254575);
    }

    fn part_2() {

    }
}
