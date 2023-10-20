//! --- Day 5: Doesn't He Have Intern-Elves For This? ---
//! Santa needs help figuring out which strings in his text file are naughty or nice.
//! 
//! A nice string is one with all of the following properties:
//! 
//! It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
//! It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
//! It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.
//! For example:
//! 
//! - ugknbfddgicrmopn is nice because it has at least three vowels (u...i...o...), a
//!   double letter (...dd...), and none of the disallowed substrings.
//! - aaa is nice because it has at least three vowels and a double letter, even though
//!   the letters used by different rules overlap.
//! - jchzalrnumimnmhp is naughty because it has no double letter.
//! - haegwjzuvuyypxyu is naughty because it contains the string xy.
//! - dvszwmarrgswjxmb is naughty because it contains only one vowel.
//!
//! How many strings are nice?

// ==== Part One

fn three_vowels(s: &str) -> bool {
    let vowels = "aeiou";
    let mut n: usize = 0;
    for c in s.chars() {
        if vowels.contains(c) {
            n += 1;
        }
        if n >= 3 {
            return true
        }
    }
    false
}


fn serial_letter(s: &str) -> bool {
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if Some(&c) == chars.peek() {
            return true
        }
    }
    false
}


fn no_naughty_substr(s: &str) -> bool {
    let naughty = &["ab", "cd", "pq", "xy"];
    let mut chars = s.chars().peekable();
    while let Some(a) = chars.next() {
        if let Some(b) = chars.peek() {
            let pair = format!("{a}{b}");
            if naughty.contains(&pair.as_str()) {
                return false
            }
        }
    }
    true
}

fn part_1(input: &[String]) -> usize {
    let n_nice = input.iter()
        .fold(0_usize, |mut n, s| {
            if three_vowels(&s)
                && serial_letter(&s)
                && no_naughty_substr(&s)
            {
                n += 1;
            }
            n
        });
    n_nice
}

// ==== Part Two

fn has_any_pair_twice(s: &str) -> bool {
    if s.len() < 4 {
        return false
    }
    let mut pairs: Vec<[char; 2]> = Vec::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if let Some(peek) = chars.peek() {
            let pair = [c, *peek];
            if pairs.contains(&pair) {
                if pairs.last().unwrap() != &pair {
                    // this prevents counting something like "aaa"
                    return true
                }
            }
            pairs.push(pair);
        }
    }
    return false
}


fn has_sandwhich(s: &str) -> bool {
    if s.len() < 3 {
        return false
    }
    let mut chars = s.chars().peekable();
    let mut current_pair: [char; 2] = [
        chars.next().unwrap(),
        chars.next().unwrap()
    ];
    while let Some(c) = chars.next() {
        if &c == &current_pair[0] || chars.peek() == Some(&current_pair[1]) {
            return true
        }
        current_pair.swap(0, 1);
        current_pair[1] = c;
    }
    false
}


fn part_2(input: &[String]) -> usize {
    let n_nice = input.iter()
        .fold(0_usize, |n, s| {
            if has_any_pair_twice(&s) && has_sandwhich(&s) {
                n + 1
            } else {
                n
            }
        });
    n_nice
}


#[cfg(test)]
mod tests {
    use std::{path::Path, fs::File, io::{BufRead, BufReader}};

    fn load_input() -> std::io::Result<Vec<String>> {
        let file_path = Path::new(file!())
            .parent()
            .expect("impossible, every file has a parent")
            .join("input.txt");
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let out = reader.lines()
            // just have a panic if line can't be read -- it's only for testing
            .map(|line| line.unwrap())
            .collect();
        Ok(out)
    }

    #[test]
    fn part_1() {
        let input = load_input().unwrap();
        let output = super::part_1(&input);
        assert_eq!(output, 258);
    }

    #[test]
    fn part_2() {
        let input = load_input().unwrap();
        let output = super::part_2(&input);
        assert_eq!(output, 53);
    }
}
