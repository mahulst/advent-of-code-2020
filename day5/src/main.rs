use core::fmt;
use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::ops::BitAnd;
use std::str::FromStr;

fn main() {
    let input = open_file("./src/input.txt");

    let mut seating_ids: Vec<usize> = input
        .lines()
        .map(|boarding_pass| {
            let chars: Vec<char> = boarding_pass.chars().collect();
            let (row, seat) = chars.split_at(7);

            let row_nr: usize = row.iter().enumerate().fold(0, |sum, (i, char)| {
                let value = f64::powf(0.5, i as f64) * 64.0;
                if *char == 'B' {
                    sum + f64::floor(value) as usize
                } else {
                    sum
                }
            });

            let seat_nr: usize = seat.iter().enumerate().fold(0, |sum, (i, char)| {
                let value = f64::powf(0.5, i as f64) * 4.0;
                if *char == 'R' {
                    sum + f64::floor(value) as usize
                } else {
                    sum
                }
            });

            row_nr * 8 + seat_nr
        })
        .collect();

    seating_ids.sort();

    dbg!(seating_ids.last());
}

fn open_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Input not found");

    let mut string = String::new();
    file.read_to_string(&mut string)
        .expect("Error while reading file");

    string
}
