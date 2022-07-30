use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

use itertools::Itertools;

// 0 - 6 digits
// 1 - 2
// 2 - 5
// 3 - 5
// 4 - 4
// 5 - 5
// 6 - 6
// 7 - 3
// 8 - 7
// 9 - 6

pub fn solution() {
    let path = Path::new("files/8.txt").canonicalize().unwrap();

    let f = File::open(path).expect("file not found");

    let reader = BufReader::new(f);

    let parts = reader
        .lines()
        .map(Result::unwrap)
        .into_iter()
        .flat_map(|s| {
            s.split('|')
                .map(str::trim)
                .map(str::to_string)
                .collect_vec()
        })
        .chunks(2)
        .into_iter()
        .map(|x| x.into_iter().collect_tuple::<(_, _)>())
        .map(Option::unwrap)
        .collect_vec();

    let counts = parts
        .iter()
        .map(|(part1, part2)| count_occurences_in_line(part1, part2))
        .sum::<usize>();

    println!("{counts}");
}

fn count_occurences_in_line(part1: &str, part2: &str) -> usize {
    let nums1 = part1.split_ascii_whitespace().collect_vec();
    let nums2 = part2.split_ascii_whitespace().collect_vec();
    let mut count_occurences = 0;
    for num in &nums2 {
        for other_num in &nums1 {
            match (num.len(), other_num.len()) {
                (2, 2) | (3, 3) | (4, 4) | (7, 7) => {
                    count_occurences += 1;
                }
                (_, _) => continue,
            }
        }
    }
    count_occurences
}
