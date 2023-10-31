//! --- Day 10: Elves Look, Elves Say ---
//! Today, the Elves are playing a game called look-and-say. They take turns making
//! sequences by reading aloud the previous sequence and using that reading as the next
//! sequence. For example, `211` is read as `"one two, two ones"`, which becomes `1221`
//! (`1` `2`, `2` `1`s).
//! 
//! Look-and-say sequences are generated iteratively, using the previous value as input
//! for the next step. For each step, take the previous value, and replace each run of
//! digits (like `111`) with the number of digits (`3`) followed by the digit itself
//! (`1`).
//! 
//! For example:
//! 
//! - `1` becomes `11` (`1` copy of digit `1`).
//! - `11` becomes `21` (`2` copies of digit `1`).
//! - `21` becomes `1211` (one `2` followed by one `1`).
//! - `1211` becomes `111221` (one `1`, one `2`, and two `1`s).
//! - `111221` becomes `312211` (three `1`s, two `2`s, and one `1`).
//!
//! Starting with the digits in your puzzle input, apply this process 40 times. What is
//! the length of the result?

fn next_sequence(input: &str) -> String {
    let mut output = String::from("");
    let mut count: usize = 1;
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        match chars.peek() {
            Some(nxt) => {
                if nxt == &c {
                    count += 1
                } else {
                    output.push_str(&format!("{count}{c}"));
                    count = 1;
                }
            },
            None => output.push_str(&format!("{count}{c}")),
        }
    }
    output
}


fn part_1(input: &str, n_iters: usize) -> usize {
    let mut output = String::from(input);
    for _ in 0..n_iters {
        output = next_sequence(&output);
    }
    output.len()
}


#[cfg(test)]
mod tests {

    #[test]
    fn part_1_sm() {
        let input = "1211";
        let output = super::part_1(input, 1);
        assert_eq!(output, 6);
    }

    #[test]
    fn part_1() {
        let input = "1113222113";
        let output = super::part_1(input, 40);
        assert_eq!(output, 252594);
    }

}
