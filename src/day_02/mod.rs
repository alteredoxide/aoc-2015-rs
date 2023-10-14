//! Advent of Code 2015
//! 
//! --- Day 2: I Was Told There Would Be No Math ---
//! The elves are running low on wrapping paper, and so they need to submit an order for
//! more. They have a list of the dimensions (length l, width w, and height h) of each
//! present, and only want to order exactly as much as they need.
//! 
//! Fortunately, every present is a box (a perfect right rectangular prism), which makes
//! calculating the required wrapping paper for each gift a little easier: find the
//! surface area of the box, which is 2*l*w + 2*w*h + 2*h*l. The elves also need a little
//! extra paper for each present: the area of the smallest side.
//! 
//! For example:
//! 
//! - A present with dimensions 2x3x4 requires 2*6 + 2*12 + 2*8 = 52 square feet of
//!   wrapping paper plus 6 square feet of slack, for a total of 58 square feet.
//! - A present with dimensions 1x1x10 requires 2*1 + 2*10 + 2*10 = 42 square feet of
//!   wrapping paper plus 1 square foot of slack, for a total of 43 square feet.
//!
//! All numbers in the elves' list are in feet. How many total square feet of wrapping
//! paper should they order?

use std::{path::Path, fs::File, io::{BufRead, BufReader}};


fn load_input() -> std::io::Result<Vec<(usize, usize, usize)>> {
    let mod_path = Path::new(file!())
        .parent()
        .expect("the module file has no parent path")
        .join("input.txt");
    let file = File::open(mod_path)?;
    let reader = BufReader::new(file);
    let out = reader
        .lines()
        .map(|line| {
            let vec = line
                .unwrap()
                .split("x")
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            if vec.len() != 3 { panic!("there should be three values in every line") }
            (vec[0], vec[1], vec[2])
        })
        .collect();
    Ok(out)
}


fn part_1(input: &[(usize, usize, usize)]) -> usize {
    let mut total_area = 0;
    for (l, w, h) in input {
        let a_min = (l*w).min(l*h).min(w*h);
        total_area += 2*l*w + 2*w*h + 2*h*l + a_min;
    }
    total_area
}


#[cfg(test)]
mod test {
    #[test]
    fn part_1() {
        let input = super::load_input().unwrap();
        let output = super::part_1(&input);
        assert_eq!(output, 1606483);
    }
}
