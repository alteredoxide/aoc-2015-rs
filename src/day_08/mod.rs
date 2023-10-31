//! --- Day 8: Matchsticks ---
//! Space on the sleigh is limited this year, and so Santa will be bringing his list as a
//! digital copy. He needs to know how much space it will take up when stored.
//! 
//! It is common in many programming languages to provide a way to escape special
//! characters in strings. For example, C, JavaScript, Perl, Python, and even PHP handle
//! special characters in very similar ways.
//! 
//! However, it is important to realize the difference between the number of characters in
//! the code representation of the string literal and the number of characters in the
//! in-memory string itself.
//! 
//! For example:
//! 
//! - `""` is `2` characters of code (the two double quotes), but the string contains zero
//!   characters.
//! - `"abc"` is `5` characters of code, but `3` characters in the string data.
//! - `"aaa\"aaa"` is `10` characters of code, but the string itself contains six `"a"`
//!   characters and a single, escaped quote character, for a total of `7` characters in
//!   the string data.
//! - `"\x27"` is `6` characters of code, but the string itself contains just one - an
//!   apostrophe (`'`), escaped using hexadecimal notation.
//!
//! Santa's list is a file that contains many double-quoted string literals, one on each
//! line. The only escape sequences used are `\\` (which represents a single backslash),
//! `\"` (which represents a lone double-quote character), and `\x` plus two hexadecimal
//! characters (which represents a single character with that ASCII code).
//! 
//! Disregarding the whitespace in the file, what is the number of characters of code for
//! string literals minus the number of characters in memory for the values of the strings
//! in total for the entire file?
//! 
//! For example, given the four strings above, the total number of characters of string
//! code (`2 + 5 + 10 + 6 = 23`) minus the total number of characters in memory for string
//! values (`0 + 3 + 7 + 1 = 11`) is `23 - 11 = 12`.
//!
//! --- Part Two ---
//! Now, let's go the other way. In addition to finding the number of characters of code,
//! you should now encode each code representation as a new string and find the number of
//! characters of the new encoded representation, including the surrounding double quotes.
//! 
//! For example:
//! 
//! - `""` encodes to `"\"\""`, an increase from `2` characters to `6`.
//! - `"abc"` encodes to `"\"abc\""`, an increase from `5` characters to `9`.
//! - `"aaa\"aaa"` encodes to `"\"aaa\\\"aaa\""`, an increase from `10` characters to
//!   `16`.
//! - `"\x27"` encodes to `"\"\\x27\""`, an increase from `6` characters to `11`.
//!
//! Your task is to find the total number of characters to represent the newly encoded
//! strings minus the number of characters of code in each original string literal. For
//! example, for the strings above, the total encoded length (`6 + 9 + 16 + 11 = 42`)
//! minus the characters in the original code representation (`23`, just like in the first
//! part of this puzzle) is `42 - 23 = 19`.

fn count_memory_chars(s: &str) -> usize {
    let mut chars = s.trim().chars().peekable();
    let mut count: usize = 0;
    while let Some(c) = chars.next() {
        if c == '\\' {
            let nxt = chars.next().expect("a character did not follow an escape");
            if nxt == 'x' {
                let _ = chars.next().expect("expected two hexadecimal chars");
                let _ = chars.next().expect("expected two hexadecimal chars");
            }
        } else if c == '"' {
            continue
        }
        count += 1;
    }
    count
}


fn count_code_chars(s: &str) -> usize {
    let count: usize = s
        .trim()
        .chars()
        .fold(0usize, |acc, _| acc + 1);
    count
}


/// This approach would is not only more consise, but it it also more robust: while the
/// constraints of this problem only expect a couple escape char scenarios, this approach
/// would also handle other escape chars (e.g. '\t') and patterns.
fn count_newstr_chars_std(s: &str) -> usize {
    let escaped = s.escape_default().to_string();
    let newstr = format!("\"{}\"", escaped);
    let count = count_code_chars(&newstr);
    count
}


/// This approach, while not as robust as the one using `escape_default()`, it provides
/// more insight into what is actually happening.
fn count_newstr_chars_manual(s: &str) -> usize {
    let mut newstr = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => newstr.push_str("\\\""),
            '\\' => newstr.push_str("\\\\"),
            _ => newstr.push(c),
        };
    }
    newstr.push_str("\"");
    let count = count_code_chars(&newstr);
    count
}


fn part_1(input: &[String]) -> usize {
    let mut memory_count: usize = 0;
    let mut code_count: usize = 0;
    for s in input {
        memory_count += count_memory_chars(s);
        code_count += count_code_chars(s);
    }
    code_count - memory_count
}


fn part_2(input: &[String]) -> usize {
    let mut code_count: usize = 0;
    let mut encoded_count: usize = 0;
    for s in input {
        code_count += count_code_chars(s);
        encoded_count += count_newstr_chars_manual(s);
    }
    encoded_count - code_count
}


#[cfg(test)]
mod tests {
    use std::{path::Path, fs::File, io::{BufRead, BufReader}};

    fn load_input(file_name: &str) -> std::io::Result<Vec<String>> {
        let path = Path::new(file!())
            .parent()
            .expect("every file should have a parent");
        let file = File::open(path.join(file_name))?;
        let reader = BufReader::new(file);
        let lines = reader
            .lines()
            .collect();
        lines
    }

    #[test]
    fn part_1_sm() {
        let input = load_input("input_sm.txt").unwrap();
        let output = super::part_1(&input);
        assert_eq!(output, 12);
    }

    #[test]
    fn part_1() {
        let input = load_input("input.txt").unwrap();
        let output = super:: part_1(&input);
        assert_eq!(output, 1333);
    }

    #[test]
    fn part_2_sm() {
        let input = load_input("input_sm.txt").unwrap();
        let output = super::part_2(&input);
        assert_eq!(output, 19);
    }
    #[test]
    fn part_2() {
        let input = load_input("input.txt").unwrap();
        let output = super::part_2(&input);
        assert_eq!(output, 2046);
    }
}
