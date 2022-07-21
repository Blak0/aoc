use std::collections::VecDeque;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

use itertools::Itertools;

#[derive(Debug)]
struct Generation {
    fish_queue: VecDeque<u64>,
}

impl Generation {
    fn next_generation(mut self) -> Self {
        let swap = self.fish_queue.pop_front().unwrap();
        self.fish_queue.push_back(0);
        self.fish_queue[6] += swap;
        self.fish_queue[8] += swap;
        self
    }
}

impl TryFrom<Vec<usize>> for Generation {
    type Error = String;

    fn try_from(initial_state: Vec<usize>) -> Result<Self, Self::Error> {
        let mut gen = Generation {
            fish_queue: VecDeque::from_iter((0..).take(9).map(|_| 0)),
        };

        for i in initial_state {
            if i > 8 {
                return Err(format! {"Number {} not in range <0; 8>", i});
            }
            gen.fish_queue[i] += 1;
        }

        Ok(gen)
    }
}

pub fn solution() {
    let path = Path::new("files/6.txt").canonicalize().unwrap();

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
        .collect_vec();

    let start_gen = Generation::try_from(splits).unwrap();

    let n_times = 256;

    let last_gen = (1..=n_times).fold(start_gen, |acc, gen_idx| {
        println!("Gen {gen_idx}");
        acc.next_generation()
    });

    println!("{:?}", last_gen.fish_queue.into_iter().sum::<u64>());
}
