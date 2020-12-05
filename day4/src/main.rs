use core::fmt;
use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::ops::BitAnd;
use std::str::FromStr;

fn main() {
    let input = open_file("./src/input.txt");

    let necessary_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let valid_passports: Vec<String> = input
        .split("\n\n")
        .filter_map(|passport| {
            let single_line_passport = passport.replace("\n", " ");

            let has_all_fields = necessary_fields
                .iter()
                .all(|field| single_line_passport.contains(field));

            if has_all_fields {
                return Some(single_line_passport);
            }
            None
        })
        .collect();

    valid_passports[0].parse::<Passport>().unwrap();
    let validated_passwords = valid_passports
        .iter()
        .filter(|pass| { pass.parse::<Passport>().map_or(false, |p| { p.valid }) })
        .count();

    dbg!(validated_passwords);
}

#[derive(Debug, PartialEq, Clone)]
pub struct Passport {
    pub valid: bool,
}

impl FromStr for Passport {
    type Err = ();
    fn from_str(input: &str) -> Result<Passport, ()> {
        return Ok(Passport {
            valid: parse_byr(input)
                && parse_iyr(input)
                && parse_eyr(input)
                && parse_hgt(input)
                && parse_hcl(input)
                && parse_ecl(input)
                && parse_pid(input)
        });
    }
}

pub fn parse_iyr(pass: &str) -> bool {
    let re = Regex::new(r"iyr:(?P<iyr>\d*)($| )").unwrap();
    match re.captures(pass) {
        Some(caps) => {
            let iyr = caps["iyr"].parse::<i32>().unwrap();

            iyr >= 2010 && iyr <= 2020
        }
        None => false,
    }
}

pub fn parse_byr(pass: &str) -> bool {
    let re = Regex::new(r"byr:(?P<byr>\d*)($| )").unwrap();
    match re.captures(pass) {
        Some(caps) => {
            let byr = caps["byr"].parse::<i32>().unwrap();

            byr >= 1920 && byr <= 2002
        }
        None => false,
    }
}

pub fn parse_eyr(pass: &str) -> bool {
    let re = Regex::new(r"eyr:(?P<eyr>\d*)($| )").unwrap();
    match re.captures(pass) {
        Some(caps) => {
            let eyr = caps["eyr"].parse::<i32>().unwrap();

            eyr >= 2020 && eyr <= 2030
        }
        None => false,
    }
}

pub fn parse_hgt(pass: &str) -> bool {
    let re_in = Regex::new(r"hgt:(?P<hgt>\d*)in($| )").unwrap();
    let re_cm = Regex::new(r"hgt:(?P<hgt>\d*)cm($| )").unwrap();

    match re_cm.captures(pass) {
        Some(caps) => {
            let hgt = caps["hgt"].parse::<i32>().unwrap();
            hgt >= 150 && hgt <= 193
        }
        None => match re_in.captures(pass) {
            Some(caps) => {
                let hgt = caps["hgt"].parse::<i32>().unwrap();

                hgt >= 59 && hgt <= 76
            }
            None => false,
        },
    }
}

pub fn parse_hcl(pass: &str) -> bool {
    let re = Regex::new(r"hcl:#(?P<hcl>[a-f0-9]{6})($| )").unwrap();

    match re.captures(pass) {
        Some(_) => true,
        None => false,
    }
}

pub fn parse_ecl(pass: &str) -> bool {
    let re = Regex::new(r"ecl:(?P<ecl>)(amb|blu|brn|gry|grn|hzl|oth)($| )").unwrap();

    match re.captures(pass) {
        Some(_) => true,
        None => false,
    }
}

pub fn parse_pid(pass: &str) -> bool {
    let re = Regex::new(r"pid:(?P<pid>\d{9})($| )").unwrap();

    match re.captures(pass) {
        Some(_) => true,
        None => false,
    }
}

fn open_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Input not found");

    let mut string = String::new();
    file.read_to_string(&mut string)
        .expect("Error while reading file");

    string
}
