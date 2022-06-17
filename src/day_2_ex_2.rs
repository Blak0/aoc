use std::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

use itertools::Itertools;

use crate::main;

#[derive(Debug)]
struct Direction {
    x: i32,
    y: i32,
    aim: i32,
}

struct DirectionError {
    msg: String,
}

impl fmt::Display for DirectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Direction cannot be parsed from command string: {}",
            self.msg
        )
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
        "down" => Ok(Direction {
            x: 0,
            y: 0,
            aim: value,
        }),
        "up" => Ok(Direction {
            x: 0,
            y: 0,
            aim: -value,
        }),
        "forward" => Ok(Direction {
            x: value,
            y: value,
            aim: 0,
        }),
        _ => Err(DirectionError {
            msg: format!(
                "Unrecognized command: {}, expected down|up|forward",
                &command
            ),
        }),
    }
}

pub fn solution() {
    let path = Path::new("files/2.txt").canonicalize().unwrap();

    let f = File::open(path).expect("file not found");

    let reader = BufReader::new(f);

    let lines = reader.lines();

    let directions = lines
        .map(|line| direction_from_command(&line.unwrap()))
        .collect_vec();
    let mut main_command = Direction { x: 0, y: 0, aim: 0 };

    for direction in &directions {
        match direction {
            Ok(dir) if dir.aim == 0 => {
                main_command.y += dir.y * main_command.aim;
                main_command.x += dir.x;
            }
            Ok(dir) => main_command.aim += dir.aim,
            Err(msg) => panic!("{}", msg),
        }
    }
    
    println!("{}", main_command.x * main_command.y);
}
