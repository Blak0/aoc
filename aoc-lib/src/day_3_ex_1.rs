use std::cmp;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

use itertools::Itertools;

pub fn solution() {
    let path = Path::new("files/3.txt").canonicalize().unwrap();

    let f = File::open(path).expect("file not found");

    let reader = BufReader::new(f);
    let mut lines = reader.lines().peekable();

    let n = lines
        .peek()
        .expect("Failed to read file")
        .as_deref()
        .expect("File is empty")
        .len();

    let mut bytes = Vec::from_iter(std::iter::repeat(0).take(n));

    lines.for_each(|x| {
        x.unwrap().chars().enumerate().for_each(|(idx, c)| match c {
            '1' => {
                bytes[idx] += 1;
            }
            '0' => {
                bytes[idx] -= 1;
            }
            _ => {
                panic!("Only 0 and 1 allowed in inputs!")
            }
        });
    });

    let gamma = bytes
        .iter()
        .map(|byte_count| match byte_count.cmp(&0) {
            cmp::Ordering::Less | cmp::Ordering::Equal => 0,
            cmp::Ordering::Greater => 1,
        })
        .collect_vec();

    let delta = gamma
        .iter()
        .map(|byte| if *byte == 1 { 0 } else { 1 })
        .collect_vec();

    let from_bin = |bit_vec: &[i32]| -> i32 {
        bit_vec
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (idx, byte)| acc + 2i32.pow(idx as u32) * byte)
    };

    println!("{}", from_bin(&delta) * from_bin(&gamma));
}
