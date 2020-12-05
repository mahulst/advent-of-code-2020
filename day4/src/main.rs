use core::fmt;
use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let input = open_file("./src/input.txt");

    let necessary_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let valid_passports: Vec<&str> = input
        .split("\n\n")
        .filter_map(|passport| {
            let single_line_passport = passport.replace("\n", " ");

            let has_all_fields = necessary_fields
                .iter()
                .all(|field| single_line_passport.contains(field));

            if has_all_fields {
                return Some(passport);
            }
            None
        })
        .collect();

    dbg!(valid_passports.iter().count());
}

fn open_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Input not found");

    let mut string = String::new();
    file.read_to_string(&mut string)
        .expect("Error while reading file");

    string
}
