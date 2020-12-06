use core::fmt;
use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::ops::{BitAnd, Add};
use std::str::FromStr;
use std::collections::HashSet;

fn main() {
    let input = open_file("./src/input.txt");

    let groups = input.split("\n\n");
    let i = groups.fold(0 as usize, |count, group| {
        group.lines().fold(HashSet::new() as HashSet<char>, |mut set, line| {
            line.chars().for_each( |c| {
                set.insert(c);
            });
            set
        }).len()  + count
    });

    dbg!(i);
}

fn open_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Input not found");

    let mut string = String::new();
    file.read_to_string(&mut string)
        .expect("Error while reading file");

    string
}
