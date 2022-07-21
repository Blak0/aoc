use std::fs::File;
use std::io::{prelude::*, BufReader};

use std::path::Path;

use itertools::Itertools;

#[derive(Default, Debug)]
struct BingoTable {
    size: usize,
    number_table: Vec<Vec<u8>>,
    checked_table: Vec<Vec<bool>>,
}

impl BingoTable {
    fn new(id: u8, table: Vec<Vec<u8>>) -> BingoTable {
        BingoTable {
            size: table
                .get(0)
                .unwrap_or_else(|| panic!("Error when parsing {id} table"))
                .len(),
            number_table: table,
            checked_table: vec![vec![false; 5]; 5],
        }
    }

    fn make_a_move(&mut self, m: u8) {
        for (row_idx, row) in self.number_table.iter().enumerate() {
            for (col_idx, number) in row.iter().enumerate() {
                if m == *number {
                    self.checked_table[row_idx][col_idx] = true;
                }
            }
        }
    }

    fn is_won(&self) -> bool {
        for i in 0..self.size {
            if self.checked_table[i][0..self.size].iter().all(|x| *x)
                || (0..self.size)
                    .map(|idx| self.checked_table[idx][i])
                    .all(|cell| cell)
            {
                return true;
            }
        }
        false
    }

    fn calculate_score(&self, last_number: u8) -> u32 {
        let mut sum = 0u32;
        for row_idx in 0..self.size {
            for cell_idx in 0..self.size {
                if !self.checked_table[row_idx][cell_idx] {
                    sum += self.number_table[row_idx][cell_idx] as u32;
                }
            }
        }
        last_number as u32 * sum
    }
}

pub fn solution() {
    let path = Path::new("files/4.txt").canonicalize().unwrap();

    let f = File::open(path).expect("file not found");

    let reader = BufReader::new(f);

    let mut lines = reader.lines().filter_map(|x| x.ok());

    let moves = lines
        .next()
        .expect("File empty")
        .split(',')
        .map(str::parse::<u8>)
        .map(Result::unwrap)
        .collect_vec();

    let tables = lines
        .chunks(6)
        .into_iter()
        .map(|chunk| {
            chunk
                .skip(1)
                .take(5)
                .map(|line| {
                    line.split(' ')
                        .filter(|x| !x.is_empty())
                        .map(str::to_owned)
                        .collect_vec()
                })
                .map(|x| {
                    x.into_iter()
                        .map(|x| x.parse::<u8>())
                        .map(Result::unwrap)
                        .collect_vec()
                })
                .collect_vec()
        })
        .collect_vec();

    let mut tables = tables
        .into_iter()
        .enumerate()
        .filter(|(_, table)| !table.is_empty())
        .map(move |(idx, table)| BingoTable::new(idx as u8, table))
        .collect_vec();

    for m in moves {
        tables.iter_mut().for_each(|table| table.make_a_move(m));

        let winner_tables = tables.iter().filter(|table| table.is_won()).collect_vec();

        match winner_tables.len() {
            0 => continue,
            1 => {
                println!("{:?}", winner_tables[0].calculate_score(m));
                break;
            }
            _ => panic!("What now? :D"),
        }
    }
}
