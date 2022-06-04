use itertools::Itertools;
use std::cmp::Ordering;
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

    let values: Vec<i32> = lines_vec.iter().map(|x| x.parse().unwrap()).collect_vec();

    let sums = values
        .iter()
        .tuple_windows::<(_, _, _)>()
        .map(|(a, b, c)| a + b + c)
        .collect_vec()
        .into_iter();

    let sum_measurments = sums
        .tuple_windows::<(_, _)>()
        .map(|(a, b)| match a.cmp(&b) {
            Ordering::Less => Measurement::Increased,
            Ordering::Greater => Measurement::Decreased,
            Ordering::Equal => Measurement::NoChange,
        })
        .collect_vec();

    let times_increased = sum_measurments
        .into_iter()
        .filter(|x| *x == Measurement::Increased)
        .count();

    println!("{}", times_increased);
}
