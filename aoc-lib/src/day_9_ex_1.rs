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

    let mut lowest_points = Vec::new();

    let n = grid.len();
    let m = grid[0].len();

    for (x, y) in (0..n).cartesian_product(0..m) {
        let top = if x != 0 {
            grid[x-1][y]
        } else {
            CellType::MAX
        };

        let bottom = if x != n-1 {
            grid[x+1][y]
        } else {
            CellType::MAX
        };

        let left = if y != 0 {
            grid[x][y-1]
        } else {
            CellType::MAX
        };
        
        let right = if y != m-1 {
            grid[x][y+1]
        } else {
            CellType::MAX
        };
 
        let minimum = vec![left, right, top, bottom]
            .into_iter()
            .min().unwrap();

        if grid[x][y] < minimum {
            lowest_points.push((x,y));
        }
    }

    let sum_of_lowest_points = lowest_points
        .iter()
        .map(|(x, y)| 1 + grid[*x][*y] as usize)
        .sum::<usize>();

    println!("{}", sum_of_lowest_points);
}
