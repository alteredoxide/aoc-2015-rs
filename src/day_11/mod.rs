//! --- Day 11: Corporate Policy ---
//! Santa's previous password expired, and he needs help choosing a new one.
//! 
//! To help him remember his new password after the old one expires, Santa has devised a
//! method of coming up with a password based on the previous one. Corporate policy
//! dictates that passwords must be exactly eight lowercase letters (for security
//! reasons), so he finds his new password by incrementing his old password string
//! repeatedly until it is valid.
//! 
//! Incrementing is just like counting with numbers: `xx`, `xy`, `xz`, `ya`, `yb`, and so
//! on. Increase the rightmost letter one step; if it was `z`, it wraps around to `a`, and
//! repeat with the next letter to the left until one doesn't wrap around.
//! 
//! Unfortunately for Santa, a new Security-Elf recently started, and he has imposed some
//! additional password requirements:
//! 
//! - Passwords must include one increasing straight of at least three letters, like
//!   `abc`, `bcd`, `cde`, and so on, up to `xyz`. They cannot skip letters; `abd` doesn't
//!   count.
//! - Passwords may not contain the letters `i`, `o`, or `l`, as these letters can be
//!   mistaken for other characters and are therefore confusing.
//! - Passwords must contain at least two different, non-overlapping pairs of letters,
//!   like `aa`, `bb`, or `zz`.
//!
//! For example:
//! 
//! - `hijklmmn` meets the first requirement (because it contains the straight `hij`) but
//!   fails the second requirement requirement (because it contains `i` and `l`).
//! - `abbceffg` meets the third requirement (because it repeats `bb` and `ff`) but fails
//!   the first requirement.
//! - `abbcegjk` fails the third requirement, because it only has one double letter
//!   (`bb`).
//! - The next password after `abcdefgh` is `abcdffaa`.
//! - The next password after `ghijklmn` is `ghjaabcc`, because you eventually skip all
//!   the passwords that start with `ghi...`, since `i` is not allowed.
//!
//! Given Santa's current password (your puzzle input), what should his next password be?

use std::string::FromUtf8Error;

fn contains_ascending(pwd: &str) -> bool {
    let mut chars = pwd.chars().peekable();
    let mut count: usize = 1;
    while let Some(c) = chars.next() {
        if count == 3 { return true }
        match chars.peek() {
            Some(nxt) => {
                let x = c as u8;
                let y = *nxt as u8;
                if y <= x {
                    count = 1;
                    continue
                }
                if y - x == 1 {
                    count += 1;
                } else {
                    count = 1;
                }
            }
            None => {}
        }
    }
    count >= 3
}


fn contains_iol(pwd: &str) -> bool {
    pwd.find(&['i', 'o', 'l']).is_some()
}


fn contains_doubles(pwd: &str) -> bool {
    let mut chars = pwd.chars().peekable();
    let mut count: usize = 0;
    while let Some(c) = chars.next() {
        if count == 2 { return true }
        match chars.peek() {
            Some(nxt) => {
                if nxt == &c {
                    let _ = chars.next();
                    count += 1;
                }
            },
            None => {}
        }
    }
    count >= 2
}


fn pwd_is_valid(pwd: &str) -> bool {
    contains_ascending(pwd) && !contains_iol(pwd) && contains_doubles(pwd)
}


fn increment_char_idx(s: &str, idx: usize) -> Result<String, FromUtf8Error> {
    let mut bytes = s.as_bytes().to_vec();
    let new = match bytes[idx] {
        x if x - 98 == 26 => 97,
        x => x + 1
    };
    bytes[idx] = new;
    String::from_utf8(bytes)
}

fn increment_char_idx_inplace(s: &mut str, idx: usize) -> Result<(), String> {
    let bytes = unsafe { s.as_bytes_mut() };
    let new = match bytes[idx] {
        x if x - 96 == 26 => 97,
        x => x + 1
    };
    if new < 97 || new > (97 + 25) {
        return Err(String::from("new char ord would be out of bounds: {new}"))
    }
    bytes[idx] = new;
    Ok(())
}


fn reset_tail_inplace(pwd: &mut str, idx: usize) {
    let bytes = unsafe { pwd.as_bytes_mut() };
    for i in idx..bytes.len() {
        bytes[i] = 97;
    }
}


fn increment_str_inplace(s: &mut str) {
    let bytes = unsafe { s.as_bytes_mut() };
    for x in bytes.iter_mut().rev() {
        match x {
            122 => *x = 97,
            _ => {
                *x += 1;
                break
            }
        }
    }
}


fn increment_pwd(pwd: &mut str) -> Result<String, String> {
    increment_str_inplace(pwd);
    if let Some(i) = pwd.find(&['i', 'o', 'l']) {
        // increment char at idx
        increment_char_idx_inplace(pwd, i)?;
        // reset all chars after that to `a`
        reset_tail_inplace(pwd, i+1);
    }
    // TODO: optimize the next steps to be not entirely brute force
    while !(contains_ascending(&pwd) && contains_doubles(&pwd)) {
        increment_str_inplace(pwd);
    }
    Ok(pwd.to_owned())
}


fn part_1(input: &mut str) -> String {
    increment_pwd(input).unwrap();
    input.to_owned()
}


#[cfg(test)]
mod tests {

    #[test]
    fn part_1() {
        let mut input = String::from("cqjxjnds");
        //let mut input = String::from("aqixjndz");
        let output = super::part_1(&mut input);
        assert_eq!(output, "cqjxxyzz");
    }
}
