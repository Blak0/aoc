use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

use itertools::Itertools;

pub fn solution() {
    let path = Path::new("files/7.txt").canonicalize().unwrap();

    let f = File::open(path).expect("file not found");

    let reader = BufReader::new(f);
    
    let line = reader
        .lines()
        .take(1)
        .next()
        .expect("File has 0 lines.")
        .expect("Failed reading first line.");

    let splits = line
        .split(',')
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .sorted()
        .collect_vec();

    let median = median(&splits);

    let diffs = splits.iter().map(|x| x.abs_diff(median)).collect_vec();

    println!("{:?}", diffs.iter().sum::<usize>());
}

fn median(collection: &Vec<usize>) -> usize {
    let left_bound = collection.len() / 2;
    let right_bound = collection.len() / 2 - 1;
    return collection[(left_bound + right_bound) / 2];
}
