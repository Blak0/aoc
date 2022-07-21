use std::fmt::{self, Debug};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

use itertools::Itertools;

#[derive(Debug, Clone)]
struct LineParseError {
    line_number: usize,
    message: String,
}

impl fmt::Display for LineParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error parsing on line {}: {}",
            self.line_number, self.message
        )
    }
}

impl std::error::Error for LineParseError {}

fn extract_numbers_from_line(line_number: usize, line: String) -> Result<Coords, LineParseError> {
    let comma_separated_line = line
        .split(',')
        .map(|item| item.trim())
        .collect_tuple::<(_, _, _)>()
        .ok_or_else(|| LineParseError {
            line_number,
            message: "Error splitting by comma (check for commas).".to_string(),
        });

    let (first, middle, fourth) = comma_separated_line?;

    let arrow_separated_middle = middle
        .split("->")
        .map(|item| item.trim())
        .collect_tuple::<(_, _)>()
        .ok_or_else(|| LineParseError {
            line_number,
            message: "Error dividing by arrow (->) (check if there is exactly one arrow in line)."
                .to_string(),
        });

    let (second, third) = arrow_separated_middle?;

    let x1 = first.parse().map_err(|_| LineParseError {
        line_number,
        message: format!("\"{}\" is not a number.", &first),
    })?;

    let y1 = second.parse().map_err(|_| LineParseError {
        line_number,
        message: format!("\"{}\" is not a number.", &second),
    })?;

    let x2 = third.parse().map_err(|_| LineParseError {
        line_number,
        message: format!("\"{}\" is not a number.", &third),
    })?;

    let y2 = fourth.parse().map_err(|_| LineParseError {
        line_number,
        message: format!("\"{}\" is not a number.", &fourth),
    })?;

    Ok(Coords {
        x1,
        y1,
        x2,
        y2,
    })
}

#[derive(Debug)]
struct Coords {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Coords {
    fn sort(self) -> Coords {
        Coords {
            x1: usize::min(self.x1, self.x2),
            y1: usize::min(self.y1, self.y2),
            x2: usize::max(self.x1, self.x2),
            y2: usize::max(self.y1, self.y2),
        }
    }
}
struct World {
    grid: Vec<Vec<usize>>,
}

enum Direction {
    Horizontal(usize, usize),
    Vertical(usize, usize),
    Invalid,
}

impl World {
    fn new(len_x: usize, len_y: usize) -> World {
        World {
            grid: vec![vec![0; len_y as usize]; len_x as usize],
        }
    }

    fn apply_range(mut self, coords: Coords) -> World {
        let dir: Direction = coords_to_direction(&coords);

        match dir {
            Direction::Horizontal(x1, x2) => {
                let y = coords.y1;
                for x in x1..=x2 {
                    self.grid[x][y] += 1;
                }
            }
            Direction::Vertical(y1, y2) => {
                let x = coords.x1;
                for y in y1..=y2 {
                    self.grid[x][y] += 1;
                }
            }
            Direction::Invalid => {}
        };

        self
    }
}

fn coords_to_direction(coords: &Coords) -> Direction {
    if coords.x1 == coords.x2 && coords.y1 != coords.y2 {
        return Direction::Vertical(coords.y1, coords.y2);
    }
    if coords.y1 == coords.y2 && coords.x1 != coords.x2 {
        return Direction::Horizontal(coords.x1, coords.x2);
    }
    Direction::Invalid
}

pub fn solution() {
    let path = Path::new("files/5.txt").canonicalize().unwrap();

    let f = File::open(path).expect("file not found");

    let reader = BufReader::new(f);

    let nums = reader
        .lines()
        .filter_map(|x| x.ok())
        .enumerate()
        .map(|(line_idx, line)| extract_numbers_from_line(line_idx + 1, line))
        .map(Result::unwrap)
        .map(Coords::sort)
        .collect_vec();

    let max_x = nums.iter().fold(usize::MIN, |acc, coord| acc.max(coord.x2));

    let max_y = nums.iter().fold(usize::MIN, |acc, coord| acc.max(coord.y2));

    let world = nums
        .into_iter()
        .fold(World::new(max_x + 1, max_y + 1), |acc, coord| {
            acc.apply_range(coord)
        });

    let num_of_overlaps = world
        .grid
        .iter()
        .flatten()
        .filter(|cell| **cell > 1)
        .count();
    
    println!("{}", num_of_overlaps);
}
