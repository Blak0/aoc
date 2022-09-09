use std::collections::HashSet;

use std::fs::File;
use std::io::{prelude::*, BufReader};

use std::path::Path;

use itertools::Itertools;

type CellType = u8;

pub fn solution() {
    let path = Path::new("files/9.txt").canonicalize().unwrap();

    let f = File::open(path).expect("file not found");

    let reader = BufReader::new(f);

    let grid = reader
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().as_str().parse::<CellType>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut coordinates_hashmap = Vec::new();

    let n = grid.len();
    let m = grid[0].len();

    for pair in (0..n).cartesian_product(0..m) {
        coordinates_hashmap.push(pair);
    }

    let mut lowest_points = HashSet::new();

    while let Some((x, y)) = coordinates_hashmap.pop() {
        let (left_x, left_y) = (x.checked_sub(1).or(Some(0)).unwrap(), y);
        let (right_x, right_y) = (usize::min(x + 1, n - 1), y);
        let (top_x, top_y) = (x, y.checked_sub(1).or(Some(0)).unwrap());
        let (bottom_x, bottom_y) = (x, usize::min(y + 1, m - 1));

        let minimum_coords = vec![
            (left_x, left_y),
            (right_x, right_y),
            (top_x, top_y),
            (bottom_x, bottom_y),
            (x, y),
        ]
        .into_iter()
        .min_by(|(x1, y1), (x2, y2)| grid[*x1][*y1].cmp(&grid[*x2][*y2]))
        .unwrap();

        if minimum_coords == (x, y) {
            lowest_points.insert(minimum_coords);
        } else {
            coordinates_hashmap.push(minimum_coords);
        }
    }

    let sum_of_lowest_points = lowest_points
        .iter()
        .sorted()
        .inspect(|x| println!("{:?}", x))
        .map(|(x, y)| 1 + grid[*x][*y] as usize)
        .inspect(|x| println!("{:?}", &x))
        .sum::<usize>();

    println!("{}", sum_of_lowest_points);
}
