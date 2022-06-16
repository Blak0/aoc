use std::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

use itertools::Itertools;


struct DirectionError {
    msg: String,
}

impl fmt::Display for DirectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Direction cannot be parsed from command string: {}", self.msg)
    }
}

fn direction_from_command(command: &str) -> Result<Direction, DirectionError> {
    let words = command.split_whitespace().collect_tuple::<(&str, &str)>();

    let (command, value) = words.expect(
        format!(
        "Commands string should have two words: down|up|forward and integer value, got {} instead",
        &command
    )
        .as_str(),
    );

    let value = value.parse::<i32>().unwrap();

    match command {
        "down" => Ok(Direction(0, value)),
        "up" => Ok(Direction(0, -value)),
        "forward" => Ok(Direction(value, 0)),
        _ => Err(DirectionError {
            msg: format!(
                "Unrecognized command: {}, expected down|up|forward",
                &command
            ),
        }),
    }
}

#[derive(Debug)]
struct Direction(i32, i32);

pub fn solution() {
    let path = Path::new("files/2.txt").canonicalize().unwrap();

    let f = File::open(path).expect("file not found");

    let reader = BufReader::new(f);

    let lines = reader.lines();

    let directions = lines.map(|line| direction_from_command(&line.unwrap()));

    let result = directions.fold((0, 0), |acc, dir| {
        let dir = dir.map(|direction| (acc.0 + direction.0, acc.1 + direction.1));

        match dir {
            Ok(dir) => dir,
            Err(msg) => panic!("{}", msg),
        }
    });
    let result = result.0 * result.1;

    println!("{}", result);
}
