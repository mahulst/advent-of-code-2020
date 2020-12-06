use core::fmt;
use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::ops::{BitAnd, Add};
use std::str::FromStr;
use std::collections::{HashSet, HashMap};

fn main() {
    let input = open_file("./src/input.txt");

    let groups = input.split("\n\n");
    let i = groups.fold(0 as usize, |count, group| {
        let answers = group.lines().fold(HashMap::new() as HashMap<char, usize>, |mut answers_per_group, person| {
            person.chars().for_each( |answer| {
                *answers_per_group.entry(answer).or_insert(0) += 1;
            });
            answers_per_group
        });
        let people_count_in_group = group.lines().count();

        let group_yes = answers.iter().fold(0 as usize, |counted_answers, (_, amount)| {
            if *amount == people_count_in_group {
                counted_answers + 1
            } else {
                counted_answers
            }
        });

        count + group_yes
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
