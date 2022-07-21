use std::cmp;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use std::path::Path;

use itertools::Itertools;

fn most_probable(bit: i32) -> u8 {
    match bit.cmp(&0) {
        cmp::Ordering::Less => 0,
        cmp::Ordering::Greater | cmp::Ordering::Equal => 1,
    }
}

fn least_probable(bit: i32) -> u8 {
    match bit.cmp(&0) {
        cmp::Ordering::Less => 1,
        cmp::Ordering::Greater | cmp::Ordering::Equal => 0,
    }
}

fn find_bit_key(lines: &[Vec<u8>], bit_count_collapse: fn(i32) -> u8) -> Vec<u8> {
    let n = lines.get(0).expect("Lines are empty!").len();

    let pos_vs_neg_count_iter = (0..n).map(|idx| {
        lines
            .iter()
            .map(|line_bits| match line_bits[idx] {
                0 => -1,
                1 => 1,
                _ => panic!("Non binary number!"),
            })
            .sum()
    });

    pos_vs_neg_count_iter.map(bit_count_collapse).collect_vec()
}

fn filter_out(bit_lines: &[Vec<u8>], filter_predicate: fn(i32) -> u8) -> Vec<u8> {
    let mut filtered_lines: Vec<Vec<u8>> = bit_lines.to_vec();
    let mut bit_key = find_bit_key(&filtered_lines, filter_predicate);
    let n = bit_lines.get(0).expect("Lines are empty!").len();

    for idx in 0..n {
        filtered_lines = filtered_lines
            .into_iter()
            .filter(|bits| bits[idx] == bit_key[idx])
            .collect_vec();
        if filtered_lines.len() == 1 {
            break;
        }
        bit_key = find_bit_key(&filtered_lines, filter_predicate);
    }
    filtered_lines.get(0).unwrap().clone()
}

pub fn solution() {
    let path = Path::new("files/3.txt").canonicalize().unwrap();

    let f = File::open(path).expect("file not found");

    let reader = BufReader::new(f);

    let lines = Vec::from_iter(reader.lines().filter_map(|x| x.ok()));

    let lines_as_u8_vec = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|x| x.to_string().parse())
                .enumerate()
                .map(|(idx, opt)| {
                    opt.unwrap_or_else(|e| panic!("Line {idx} couldn't be parsed: {e}"))
                })
                .collect_vec()
        })
        .collect_vec();

    let to_bin = |bit_vec: &[u8]| -> i32 {
        bit_vec
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (idx, byte)| {
                acc + 2i32.pow(idx as u32) * (*byte as i32)
            })
    };

    let oxygen = filter_out(&lines_as_u8_vec, most_probable);
    let dioxide = filter_out(&lines_as_u8_vec, least_probable);

    println!("{}", to_bin(&oxygen) * to_bin(&dioxide))
}
