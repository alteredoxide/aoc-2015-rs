//! --- Day 12: JSAbacusFramework.io ---
//! Santa's Accounting-Elves need help balancing the books after a recent order.
//! Unfortunately, their accounting software uses a peculiar storage format. That's where
//! you come in.
//! 
//! They have a JSON document which contains a variety of things: arrays (`[1,2,3]`),
//! objects (`{"a":1, "b":2}`), numbers, and strings. Your first job is to simply find all
//! of the numbers throughout the document and add them together.
//! 
//! For example:
//! 
//! - `[1,2,3]` and `{"a":2,"b":4}` both have a sum of `6`.
//! - `[[[3]]]` and `{"a":{"b":4},"c":-1}` both have a sum of `3`.
//! - `{"a":[-1,1]}` and `[-1,{"a":1}]` both have a sum of `0`.
//! - `[]` and `{}` both have a sum of `0`.
//!
//! You will not encounter any strings containing numbers.
//! 
//! What is the sum of all numbers in the document?

use std::num::ParseIntError;


fn update_sum(sum: &mut i32, digit: &mut String) -> Result<(), ParseIntError> {
    if !digit.is_empty() {
        *sum += digit.parse::<i32>()?;
        digit.clear();
    }
    Ok(())
}


fn part_1(input: &str) -> Result<i32, ParseIntError> {
    let mut chars = input.chars().peekable();
    let mut sum = 0;
    let mut digit = String::from("");
    while let Some(c) = chars.next() {
        match c {
            '-' => if digit.is_empty() {
                match chars.peek() {
                    Some(nxt) => match nxt {
                        '0'..='9' => digit.push(c),
                        _ => {}
                    },
                    None => update_sum(&mut sum, &mut digit)?,
                }
            },
            '0'..='9' => digit.push(c),
            _ => update_sum(&mut sum, &mut digit)?,
        }
    }
    Ok(sum)
}


#[cfg(test)]
mod tests {
    use std::path::Path;

    fn load_input(fname: &str) -> std::io::Result<String> {
        let path = Path::new(file!())
            .parent()
            .expect("every file has a parent")
            .join(fname);
        std::fs::read_to_string(path)
    }

    #[test]
    fn part_1_sm() {
        let input = load_input("input_sm.txt").unwrap();
        let output = super::part_1(&input).unwrap();
        assert_eq!(output, 41);
    }

    #[test]
    fn part_1() {
        let input = load_input("input.txt").unwrap();
        let output = super::part_1(&input).unwrap();
        assert_eq!(output, 191164);
    }

}
