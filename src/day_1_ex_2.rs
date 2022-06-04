use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;
use std::path::Path;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Measurement {
    Increased,
    Decreased,
    NoChange,
}

pub fn solution() {
    let path = Path::new("files/1.txt").canonicalize().unwrap();

    let f = File::open(path).expect("file not found");

    let reader = BufReader::new(f);

    let lines_vec: Vec<_> = reader.lines().map(|x| x.unwrap()).collect();

    let values = lines_vec
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect_vec();

    let sums = values
        .into_iter()
        .tuple_windows::<(i32, i32, i32)>()
        .map(|(a, b, c)| a + b + c)
        .collect_vec()
        .into_iter();

    let sum_measurments = sums
        .tuple_windows::<(i32, i32)>()
        .map(|(a, b)| {
            if a < b {
                Measurement::Increased
            } else if a > b {
                Measurement::Decreased
            } else {
                Measurement::NoChange
            }
        })
        .collect_vec();

    let times_increased = sum_measurments
        .iter()
        .filter(|x| **x == Measurement::Increased)
        .count();

    println!("{}", times_increased);
}
