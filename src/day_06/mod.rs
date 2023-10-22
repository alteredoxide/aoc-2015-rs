//! --- Day 6: Probably a Fire Hazard ---
//! Because your neighbors keep defeating you in the holiday house decorating contest year
//! after year, you've decided to deploy one million lights in a 1000x1000 grid.
//! 
//! Furthermore, because you've been especially nice this year, Santa has mailed you
//! instructions on how to display the ideal lighting configuration.
//! 
//! Lights in your grid are numbered from 0 to 999 in each direction; the lights at each
//! corner are at 0,0, 0,999, 999,999, and 999,0. The instructions include whether to turn
//! on, turn off, or toggle various inclusive ranges given as coordinate pairs. Each
//! coordinate pair represents opposite corners of a rectangle, inclusive; a coordinate
//! pair like 0,0 through 2,2 therefore refers to 9 lights in a 3x3 square. The lights all
//! start turned off.
//! 
//! To defeat your neighbors this year, all you have to do is set up your lights by doing
//! the instructions Santa sent you in order.
//! 
//! For example:
//! 
//! - turn on 0,0 through 999,999 would turn on (or leave on) every light.
//! - toggle 0,0 through 999,0 would toggle the first line of 1000 lights, turning off the
//!   ones that were on, and turning on the ones that were off.
//! - turn off 499,499 through 500,500 would turn off (or leave off) the middle four
//!   lights.
//!
//! After following the instructions, how many lights are lit?
//!
//! --- Part Two ---
//! You just finish implementing your winning light pattern when you realize you
//! mistranslated Santa's message from Ancient Nordic Elvish.
//! 
//! The light grid you bought actually has individual brightness controls; each light can
//! have a brightness of zero or more. The lights all start at zero.
//! 
//! The phrase turn on actually means that you should increase the brightness of those
//! lights by 1.
//! 
//! The phrase turn off actually means that you should decrease the brightness of those
//! lights by 1, to a minimum of zero.
//! 
//! The phrase toggle actually means that you should increase the brightness of those
//! lights by 2.
//! 
//! What is the total brightness of all lights combined after following Santa's
//! instructions?
//! 
//! For example:
//! 
//! - turn on 0,0 through 0,0 would increase the total brightness by 1.
//! - toggle 0,0 through 999,999 would increase the total brightness by 2000000.
//!
//! MY NOTE: I could have used [[bool; 1000]; 1000], but I chose to use a bitset/bitmap
//! because while the bool array will take an entire 1Mb in memory, the bitmap will only
//! consume 125Kb. It does, however, make the implementation of this solution much longer.
//!
//! MY NOTE: For Part Two, I could have passed in &[Vec<usize>] instead of the flattened
//! array, since I'm no longer able to take advantage of the smaller size of the bitset,
//! but I left it flat for two simple reasons:
//! 1. It's sligtly easier to construct the lights array
//! 2. I could just reuse the majority of the code from `part_1()`, along with the
//!    `flat_idx_ranges()` function.


use std::num::ParseIntError;

#[derive(Clone, Copy, Debug)]
enum Action {
    Off,
    On,
    Toggle,
}


impl From<&str> for Action {
    fn from(value: &str) -> Self {
        if value.starts_with("toggle") {
            return Action::Toggle
        } else if value.starts_with("turn on") {
            return Action::On
        } else if value.starts_with("turn off") {
            return Action::Off
        }
        unreachable!("we shouldn't be here")
    }
}


fn single_coord_from_str(s: &str) -> Result<[usize; 2], String> {
    let v = s.split(",")
        .map(|x| x.parse::<usize>())
        .collect::<Result<Vec<usize>, ParseIntError>>()
        .map_err(|e| e.to_string())?;
    if v.len() != 2 {
        return Err(format!("malformed string: {}", s))
    }
    Ok([v[0], v[1]])
}


fn coords_from_str(s: &str) -> Result<[[usize; 2]; 2], String> {
    let splits = s.split(" through ").collect::<Vec<&str>>();
    if splits.len() != 2 {
        return Err(format!("malformed string:  {}", s))
    }
    let (s1, s2) = (
        splits[0].split_whitespace().last(),
        splits[1].split_whitespace().last()
    );
    if s1.is_none() || s2.is_none() {
        return Err(format!("malformed string:  {}", s))
    }
    let c1 = single_coord_from_str(s1.unwrap())?;
    let c2 = single_coord_from_str(s2.unwrap())?;
    Ok([c1, c2])
}


#[derive(Debug)]
struct Instruction {
    pub action: Action,
    pub coords: [[usize; 2]; 2],
}


impl TryFrom<String> for Instruction {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let action: Action = value.as_str().into();
        let coords = coords_from_str(&value)?;
        Ok(Self { action, coords })
    }
}


/// `coord` should be [row, col], and 0-based indexing.
fn flat_idx(coord: &[usize; 2], n_cols: usize) -> usize {
    coord[0] * n_cols + coord[1]
}


fn flat_idx_ranges(coords: &[[usize; 2]; 2], total_cols: usize) -> Vec<Vec<usize>> {
    // + 1 because it's inclusive of the grater value
    let n_rows = coords[1][0] - coords[0][0] + 1;
    let n_cols = coords[1][1] - coords[0][1] + 1;
    let mut out = Vec::with_capacity(n_rows);
    for i in 0..n_rows {
        let ri = coords[0][0] + i;
        let ci = coords[0][1];
        let start_idx = flat_idx(&[ri, ci], total_cols);
        let mut row = Vec::with_capacity(n_cols);
        for j in 0..n_cols {
            row.push(start_idx + j);
        }
        out.push(row);
    }
    out
}


fn byte_idx(idx: usize) -> usize {
    // if 0 <= idx <= 7: return 0;
    // if 8 <= idx <= 15: return 1;
    // ...
    idx / 8
}


fn bit_idx(idx: usize) -> usize {
    // the `7 - ()` isn't strictly necessary: it's just there to preserve
    // let -> right order in the bit set
    7 - (idx % 8)
}


fn update_bit(bitset: &mut [u8], idx: usize, op: i8) -> Result<(), String> {
    let byte_i = byte_idx(idx);
    let pos = bit_idx(idx);
    match op {
        0 => {
            bitset[byte_i] = bitset[byte_i] & !(1u8 << pos);
        },
        1 => {
            bitset[byte_i] = bitset[byte_i] | (1u8 << pos);
        },
        -1 => {
            bitset[byte_i] = bitset[byte_i] ^ (1u8 << pos);
        },
        _ => return Err(format!("invalid op: {}", op))
    }
    Ok(())
}


fn part_1(input: &[Instruction]) -> Result<usize, String> {
    const N_BYTES: usize = 1_000_000 / 8;
    let total_cols = 1000;
    let mut bitset = [0u8; N_BYTES];
    for instruct in input {
        let ranges = flat_idx_ranges(&instruct.coords, total_cols);
        for range in ranges {
            for idx in range {
                match instruct.action {
                    Action::Off => update_bit(&mut bitset, idx, 0)?,
                    Action::On => update_bit(&mut bitset, idx, 1)?,
                    Action::Toggle => update_bit(&mut bitset, idx, -1)?,
                }
            }
        }
    }
    let count = bitset
        .iter()
        .map(|byte| byte.count_ones() as usize)
        .sum();
    Ok(count)
}


fn part_2(input: &[Instruction], lights: &mut [usize], n_cols: usize) -> usize {
    for instruct in input {
        let ranges = flat_idx_ranges(&instruct.coords, n_cols);
        for range in ranges {
            for idx in range {
                match instruct.action {
                    Action::Off => match lights[idx] > 0 {
                        true => lights[idx] -= 1,
                        false => {},
                    },
                    Action::On => lights[idx] += 1,
                    Action::Toggle => lights[idx] += 2,
                }
            }
        }
    }
    lights.iter().sum()
}


#[cfg(test)]
mod tests {
    use std::{path::Path, fs::File, io::{BufRead, BufReader}};

    use super::Instruction;

    fn load_input(fname: &str) -> std::io::Result<Vec<Instruction>> {
        let file_path = Path::new(file!())
            .parent()
            .expect("every file has a parent")
            .join(fname);
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let data = reader.lines()
            .filter_map(|l| {
                let s = l.unwrap();
                if s.is_empty() {
                    return None
                }
                let instruct = s.try_into().expect("invalid data format");
                Some(instruct)
            })
            .collect();
        Ok(data)
    }

    #[test]
    fn flat_idx() {
        let cases: Vec<([usize; 2], [usize; 2], usize)> = vec![
            // ([n_rows, n_cols], [r, c], expected)
            ([3, 3], [2, 2], 8),
            ([3, 3], [2, 1], 7),
            ([3, 3], [1, 1], 4),
            ([5, 3], [4, 1], 13),
            ([5, 7], [4, 5], 33),
        ];
        for case in cases {
            let n_cols = case.0[1];
            let coord = case.1;
            let expected = case.2;
            let output = super::flat_idx(&coord, n_cols);
            assert_eq!(output, expected);
        }
    }

    #[test]
    fn flat_idx_ranges() {
        let cases: Vec<([usize; 2], [[usize; 2]; 2], Vec<Vec<usize>>)> = vec![
            // ([n_rows, n_cols], [[r, c], [r, c]], expected)
            ([5, 5], [[1, 1], [2, 2]], vec![vec![6, 7], vec![11, 12]]),
        ];
        for case in cases {
            let total_cols = case.0[1];
            let coords = case.1;
            let expected = case.2;
            let output = super::flat_idx_ranges(&coords, total_cols);
            assert_eq!(output, expected);
        }
    }

    #[test]
    fn update_bit() {
        const N_BYTES: usize = 16 / 8;
        let mut bitset = [0u8; N_BYTES];
        let idx = 11;
        super::update_bit(&mut bitset, idx, 1).unwrap();
        assert_eq!(&format!("{:08b}", bitset[0]), "00000000");
        assert_eq!(&format!("{:08b}", bitset[1]), "00010000");
    }

    #[test]
    fn update_bit_toggle() {
        const N_BYTES: usize = 16 / 8;
        let mut bitset = [0u8, 255];
        super::update_bit(&mut bitset, 1, -1).unwrap();
        super::update_bit(&mut bitset, 11, -1).unwrap();
        assert_eq!(&format!("{:08b}", bitset[0]), "01000000");
        assert_eq!(&format!("{:08b}", bitset[1]), "11101111");
    }

    #[test]
    fn part_1() {
        let input = load_input("input.txt").unwrap();
        let count = super::part_1(&input).unwrap();
        assert_eq!(count, 400410);
    }

    #[test]
    fn part_2() {
        let input = load_input("input.txt").unwrap();
        let mut lights = vec![0_usize; 1_000_000];
        let n_cols = 1000;
        let output = super::part_2(&input, &mut lights, n_cols);
        assert_eq!(output, 26);
    }
}
