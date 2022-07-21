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

    let crabs = line
        .split(',')
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect_vec();

    let n = find_optimal_fuel_cost_by_coordinates(&crabs);

    let most_optimal_fuel = compute_fuel_cost_for_point(n, &crabs);

    println!("{most_optimal_fuel}");
}

fn compute_fuel_cost_for_point(x: usize, crabs: &[usize]) -> usize {
    crabs.iter().fold(0, |acc, crab| {
        let n = crab.abs_diff(x);
        acc + (n * (n + 1)) / 2
    })
}

fn find_optimal_fuel_cost_by_coordinates(splits: &[usize]) -> usize {
    let f = compute_fuel_cost_for_point;
    let mut step = splits.len() / 5;

    let mut x = splits.len() / 2;

    while step != 0 {
        let fx = f(x, splits);
        let fx_left = f(x - step, splits);
        let fx_right = f(x + step, splits);

        if fx_left < fx {
            x = x - step;
        } else if fx_right < fx {
            x = x + step;
        } else {
            step /= 2;
        }
    }
    x
}
