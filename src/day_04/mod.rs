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
//!
//! --- Part Two ---
//! Now find one that starts with six zeroes.

use std::{path::Path, thread::JoinHandle, sync::{Arc, atomic::AtomicIsize}};

use crossbeam_channel::{Receiver, Sender};
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


fn check(
    input: String,
    rx: Receiver<usize>,
    tx: Sender<usize>,
    prefix: &str,
) {
    while let Ok(i) = rx.recv() {
        let combined = format!("{input}{i}");
        let digest = md5::compute(&combined);
        let hex_str = format!("{digest:x}");
        if hex_str.starts_with(prefix) {
            tx.send(i).expect("this shouldn't fail for the given setup");
            break
        }
    }
}


fn check2(
    input: String,
    rx: Receiver<usize>,
    solution: Arc<AtomicIsize>,
    prefix: &str,
) {
    while let Ok(i) = rx.recv() {
        let combined = format!("{input}{i}");
        let digest = md5::compute(&combined);
        let hex_str = format!("{digest:x}");
        if hex_str.starts_with(&prefix) {
            solution.store(i as isize, std::sync::atomic::Ordering::Relaxed);
            break
        }
    }
}


fn part_1_par(input: &str, n_threads: usize, prefix: &str) -> Result<usize, String> {
    let (txi, rxi) = crossbeam_channel::bounded::<usize>(n_threads * 2);
    let (tx_soln, rx_soln) = crossbeam_channel::bounded::<usize>(0);
    let handles: Vec<JoinHandle<()>> = (0..n_threads).map(|_| {
        let prefix = prefix.to_owned();
        let rxi = rxi.clone();
        let tx_soln = tx_soln.clone();
        let input = input.to_owned();
        std::thread::spawn(move || check(input, rxi, tx_soln, &prefix))
    }).collect();
    let mut i = 0;
    let solution: usize;
    loop {
        if let Ok(soln) = rx_soln.try_recv() {
            drop(txi);
            solution = soln;
            break
        }
        txi.send(i).map_err(|e| e.to_string())?;
        i += 1;
    }
    for handle in handles.into_iter() {
        handle.join().map_err(|e| format!("{:?}", e))?
    }
    Ok(solution)
}


/// NOTE: just trying a different take on the solution compared to `part_1_par()`.
fn part_1_par2(input: &str, n_threads: usize, prefix: &str) -> Result<usize, String> {
    let (txi, rxi) = crossbeam_channel::bounded::<usize>(n_threads * 2);
    let solution = Arc::new(AtomicIsize::new(-1));
    let handles: Vec<JoinHandle<()>> = (0..n_threads).map(|_| {
        let prefix = prefix.to_owned();
        let rxi = rxi.clone();
        let solution = solution.clone();
        let input = input.to_owned();
        std::thread::spawn(move || check2(input, rxi, solution, &prefix))
    }).collect();
    let mut i = 0;
    let mut solni: isize;
    loop {
        solni = solution.load(std::sync::atomic::Ordering::Relaxed);
        if solni > -1 {
            drop(txi);
            break
        }
        txi.send(i).map_err(|e| e.to_string())?;
        i += 1;
    }
    for handle in handles.into_iter() {
        handle.join().map_err(|e| format!("{:?}", e))?
    }
    Ok(solni as usize)
}


#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let input = super::load_input().unwrap();
        let output = super::part_1(input.trim());
        assert_eq!(output, 254575);
    }
    
    #[test]
    fn part_1_par1() {
        // NOTE: time to run becomes unstable above 16 threads, even when more are
        // available.
        let n_threads = 8;
        let input = super::load_input().unwrap();
        let output = super::part_1_par(input.trim(), n_threads, "00000").unwrap();
        assert_eq!(output, 254575);
    }
    
    #[test]
    fn part_1_par2() {
        // NOTE: time to run becomes unstable above 16 threads, even when more are
        // available.
        let n_threads = 8;
        let input = super::load_input().unwrap();
        let output = super::part_1_par2(input.trim(), n_threads, "00000").unwrap();
        assert_eq!(output, 254575);
    }

    #[test]
    fn part_2_par1() {
        // NOTE: time to run becomes unstable above 16 threads, even when more are
        // available.
        let n_threads = 8;
        let input = super::load_input().unwrap();
        let output = super::part_1_par(input.trim(), n_threads, "000000").unwrap();
        assert_eq!(output, 1038736);

    }
}
