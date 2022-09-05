use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

use itertools::Itertools;
use std::fmt::Write;

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

    let mut count = parts
        .iter()
        .fold(0u32, |acc, part| acc + get_sum_of_digits_from_part(part));

    println!("{count}");
}

fn segment_to_u8(c: char) -> u8 {
    match c {
        'a' => 64,
        'b' => 32,
        'c' => 16,
        'd' => 8,
        'e' => 4,
        'f' => 2,
        'g' => 1,
        _ => panic!("Invalid char"),
    }
}

fn u8_to_segment(seg: u8) -> char {
    match seg {
        64 => 'a',
        32 => 'b',
        16 => 'c',
        8 => 'd',
        4 => 'e',
        2 => 'f',
        1 => 'g',
        _ => ' ',
    }
}

fn translated_segments_to_real_digit(segment: u8) -> u8 {
    match segment {
        0b0010010 => 1,
        0b1011101 => 2,
        0b1011011 => 3,
        0b0111010 => 4,
        0b1101011 => 5,
        0b1101111 => 6,
        0b1010010 => 7,
        0b1111111 => 8,
        0b1111011 => 9,
        0b1110111 => 0,
        _ => {
            panic!("Segment not recognized: {segment}")
        }
    }
}

#[derive(Debug)]
pub struct Segments {
    segments: Vec<u8>,
}

impl Default for Segments {
    fn default() -> Self {
        Self::new()
    }
}

impl Segments {
    pub fn new() -> Self {
        let segments = vec![127; 7];

        Self { segments }
    }

    fn repr(&self) -> String {
        let mut s = String::from("\n");

        for segment_num in 0..=6 {
            let mut i = 64;

            while i >= 1 {
                write!(&mut s, "{}", u8_to_segment(self.segments[segment_num] & i))
                    .expect("Failed to write segment to buffer.");
                i >>= 1;
            }
            writeln!(&mut s).expect("Failed to write whitespace to buffer.");
        }
        write!(&mut s, "\n").expect("Failed to write last whitespace to buffer.");

        s
    }
    fn segment_name_to_index(&self, segment_name: char) -> usize {
        match segment_name {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            _ => panic!("Wrong segment name."),
        }
    }

    pub fn get(&self, segment_name: char) -> &u8 {
        let idx = self.segment_name_to_index(segment_name);

        &self.segments[idx]
    }
    pub fn get_mut(&mut self, segment_name: char) -> &mut u8 {
        let idx = self.segment_name_to_index(segment_name);

        &mut self.segments[idx]
    }

    pub fn make_segment_exclusive(&mut self, segment_name: char) {
        let seg = *self.get_mut(segment_name);
        if seg & (seg - 1) != 0 {
            // if segment is not a power of 2 (not a single bit)
            panic!("Segment {seg} is not a single bit!");
        }
        for c in "abcdefg".chars().filter(|c| *c != segment_name) {
            *self.get_mut(c) &= !seg;
        }
    }

    //     self
    // }
    /// Apply closure f over given segment names a-g.
    /// Closure takes two parameters: segment as u8 and provided mask.
    ///
    /// #Example
    /// ```
    /// use aoc_lib::day_8_ex_2::Segments;
    ///
    ///
    /// let mut segments = Segments::new();
    /// let mask = 1u8;
    /// segments.apply_over_segments("acdefg".to_string(), mask, |seg, mask| seg & mask);
    /// assert!(*segments.get('a') == 1);
    /// assert!(*segments.get('b') == 127);
    /// assert!(*segments.get('c') == 1);
    /// assert!(*segments.get('d') == 1);
    /// assert!(*segments.get('e') == 1);
    /// assert!(*segments.get('f') == 1);
    /// assert!(*segments.get('g') == 1);
    ///
    /// ```
    ///
    pub fn apply_over_segments(&mut self, segment_names: String, mask: u8, f: fn(u8, u8) -> u8) {
        for c in segment_names.chars() {
            let seg = self.get_mut(c);
            *seg = f(*seg, mask);
        }
    }
}

fn pattern_as_summed_value(pattern: &str) -> u8 {
    pattern.chars().map(segment_to_u8).sum::<u8>()
}

fn get_sum_of_digits_from_part((patterns, digits): &(String, String)) -> u32 {
    let mut patterns_iter = patterns.split(' ').sorted_by(|first, second| {
        let a = first.len();
        let b = second.len();
        a.cmp(&b)
    });

    let one = patterns_iter.next().unwrap();
    let seven = patterns_iter.next().unwrap();
    let four = patterns_iter.next().unwrap();

    let fives_bytes = patterns_iter
        .take(3)
        .map(pattern_as_summed_value)
        .collect_vec();

    let one_bytes = pattern_as_summed_value(one);
    let seven_bytes = pattern_as_summed_value(seven);
    let four_bytes = pattern_as_summed_value(four);

    let mut segments = Segments::new();

    *segments.get_mut('c') &= one_bytes;
    *segments.get_mut('f') &= one_bytes;

    segments.apply_over_segments("abdeg".to_string(), one_bytes, |seg, mask| seg & !mask);

    *segments.get_mut('a') &= seven_bytes;

    segments.apply_over_segments("bdeg".to_string(), seven_bytes, |seg, mask| seg & !mask);

    segments.apply_over_segments("bd".to_string(), four_bytes, |seg, mask| seg & mask);

    segments.apply_over_segments("bd".to_string(), one_bytes, |seg, mask| seg & !mask);

    let horizontals = fives_bytes.iter().fold(127u8, |acc, x| acc & *x);

    *segments.get_mut('g') = horizontals & !*segments.get('a') & !*segments.get('d');
    *segments.get_mut('d') = horizontals & !*segments.get('a') & !*segments.get('g');

    *segments.get_mut('b') &= !*segments.get('d');

    *segments.get_mut('e') &= !*segments.get('g');

    segments.make_segment_exclusive('g');
    segments.make_segment_exclusive('d');
    segments.make_segment_exclusive('b');

    let filter_segment = *segments.get('e');

    let two = fives_bytes
        .iter()
        .filter(|five| **five & filter_segment != 0)
        .exactly_one()
        .expect("Filter segment should get us 2.");

    *segments.get_mut('c') &= two;
    *segments.get_mut('f') &= !*segments.get('c');

    let digits_iter = digits.split(' ');
    let digits_u8 = digits_iter.map(pattern_as_summed_value).collect_vec();

    let mut value = String::new();
    for digit in digits_u8 {
        let mut d = digit;

        for (idx, segment) in segments.segments.iter().enumerate() {
            let should_swap = segment & digit != 0;
            if should_swap {
                d = d - segment + (1 << (6 - idx));
            }
        }
        let x = translated_segments_to_real_digit(d);
        value.push(char::from_digit(x as u32, 10).unwrap());
    }
    value.parse().unwrap()
}
