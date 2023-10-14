//! Advent of Code 2015
//! 
//! --- Day 3: Perfectly Spherical Houses in a Vacuum ---
//! Santa is delivering presents to an infinite two-dimensional grid of houses.
//! 
//! He begins by delivering a present to the house at his starting location, and then an
//! elf at the North Pole calls him via radio and tells him where to move next. Moves are
//! always exactly one house to the north (^), south (v), east (>), or west (<). After
//! each move, he delivers another present to the house at his new location.
//! 
//! However, the elf back at the north pole has had a little too much eggnog, and so his
//! directions are a little off, and Santa ends up visiting some houses more than once.
//! How many houses receive at least one present?
//! 
//! For example:
//! 
//! - > delivers presents to 2 houses: one at the starting location, and one to the east.
//! - ^>v< delivers presents to 4 houses in a square, including twice to the house at his
//!   starting/ending location.
//! - ^v^v^v^v^v delivers a bunch of presents to some very lucky children at only 2 houses.

use std::{collections::HashSet, path::Path};


fn part_1(input: &str) -> usize {
    let mut coords: HashSet<(isize, isize)> = HashSet::new();
    let mut row: isize = 0;
    let mut col: isize = 0;
    coords.insert((row, col));
    for arrow in input.chars() {
        match arrow {
            '^' => row += 1,
            'v' => row -= 1,
            '>' => col += 1,
            '<' => col -= 1,
            '\n' => {},
            _ => unreachable!("unexpected char found")
        }
        coords.insert((row, col));
    }
    coords.len()
}


fn part_2(input: &str) -> usize {
    let mut coords: HashSet<(isize, isize)> = HashSet::new();
    let (mut r1, mut c1) = (0_isize, 0_isize);
    let (mut r2, mut c2) = (0_isize, 0_isize);
    coords.insert((r1, c1));
    for (i, arrow) in input.chars().enumerate() {
        let (row, col) = if i % 2 == 0 {
            (&mut r1, &mut c1)
        } else {
            (&mut r2, &mut c2)
        };
        match arrow {
            '^' => *row += 1,
            'v' => *row -= 1,
            '>' => *col += 1,
            '<' => *col -= 1,
            '\n' => {},
            _ => unreachable!("unexpected char found")
        }
        coords.insert((*row, *col));
    }
    coords.len()
}


fn load_input() -> std::io::Result<String> {
    let mod_path = Path::new(file!())
        .parent()
        .unwrap()
        .join("input.txt");
    std::fs::read_to_string(mod_path)
}


#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let input = super::load_input().unwrap();
        let output = super::part_1(&input);
        assert_eq!(output, 2565);
    }

    #[test]
    fn part_2() {
        let input = super::load_input().unwrap();
        let output = super::part_2(&input);
        assert_eq!(output, 2639);
    }
}
