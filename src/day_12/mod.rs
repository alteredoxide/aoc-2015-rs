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
//! --- Part Two ---
//! Uh oh - the Accounting-Elves have realized that they double-counted everything red.
//! 
//! Ignore any object (and all of its children) which has any property with the value
//! `"red"`. Do this only for objects (`{...}`), not arrays (`[...]`).
//! 
//! - `[1,2,3]` still has a sum of `6`.
//! - `[1,{"c":"red","b":2},3]` now has a sum of `4`, because the middle object is
//!   ignored.
//! - `{"d":"red","e":[1,2,3,4],"f":5}` now has a sum of `0`, because the entire structure
//!   is ignored.
//! - `[1,"red",5]` has a sum of `6`, because `"red"` in an array has no effect.

use std::num::ParseIntError;

use serde_json::{self, Value, Number, Map};


fn update_sum(sum: &mut i64, digit: &mut String) -> Result<(), ParseIntError> {
    if !digit.is_empty() {
        *sum += digit.parse::<i64>()?;
        digit.clear();
    }
    Ok(())
}


fn part_1(input: &str) -> Result<i64, ParseIntError> {
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


fn count_number(x: Number) -> Option<i64> {
    x.as_i64()
}


fn count_vec(data: Vec<Value>) -> i64 {
    let mut count: i64 = 0;
    for val in data {
        match val {
            Value::Number(x) => count += count_number(x).expect("no!"),
            Value::Array(v) => count += count_vec(v),
            Value::Object(m) => count += count_map(m),
            _ => {}
        }
    }
    count
}


fn count_map(data: Map<String, Value>) -> i64 {
    let mut count: i64 = 0;
    for (_, v) in data {
        match v {
            Value::Array(v) => count += count_vec(v),
            Value::Number(x) => count += count_number(x).expect("no!"),
            Value::String(s) => if s == "red" { return 0 } else { continue },
            Value::Object(m) => count += count_map(m),
            _ => {}
        }
    }
    count
}


fn part_2(input: &str) -> Result<i64, String> {
    let decoded = serde_json::from_str(input).map_err(|e| e.to_string())?;
    let data = match decoded {
        Value::Array(v) => v,
        Value::Null => return Err(String::from("no data")),
        _ => return Err(String::from("expected an array for input data"))
    };
    let mut count: i64 = 0;
    for datum in data {
        match datum {
            Value::Array(v) => count += count_vec(v),
            Value::Number(x) => count += count_number(x).expect("no!"),
            Value::Object(m) => count += count_map(m),
            _ => {},
        }
    }
    Ok(count)
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

    #[test]
    fn part_2_sm() {
        let input = load_input("input_sm_red.txt").unwrap();
        let output = super::part_2(&input).unwrap();
        assert_eq!(output, 35);
    }

    #[test]
    fn part_2() {
        let input = load_input("input.txt").unwrap();
        let output = super::part_2(&input).unwrap();
        assert_eq!(output, 87842);
    }

}
