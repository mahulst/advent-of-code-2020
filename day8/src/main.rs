use crate::Operation::{Acc, Jmp, Nop};
use core::fmt;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::ops::{Add, BitAnd};
use std::str::FromStr;

fn main() {
    let input = open_file("./src/input.txt");
    let boot_code: BootCode = input.parse().unwrap();

    let result = find_second_instruction(&boot_code, 0, &mut HashSet::new(), 0);
    dbg!(result);
}

fn find_second_instruction(
    boot_code: &BootCode,
    next: i32,
    visited_instructions: &mut HashSet<i32>,
    mut acc: i32,
) -> i32 {
    if visited_instructions.contains(&next) {
        return acc;
    } else {
        visited_instructions.insert(next);
        let (ins, amount) = &boot_code.instructions[next as usize];

        match ins {
            Jmp => find_second_instruction(boot_code, next + amount, visited_instructions, acc),
            Acc => {
                acc += amount;
                find_second_instruction(boot_code, next + 1, visited_instructions, acc)
            }
            Nop => find_second_instruction(boot_code, next + 1, visited_instructions, acc),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operation {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, Clone)]
pub struct BootCode {
    pub instructions: Vec<(Operation, i32)>,
}

impl FromStr for BootCode {
    type Err = ();
    fn from_str(input: &str) -> Result<BootCode, ()> {
        let instructions = input
            .lines()
            .map(|line| {
                let re = Regex::new(r"(?P<instruction>.*) (?P<amount>[-+]\d+)").unwrap();

                match re.captures(line) {
                    Some(caps) => {
                        let ins = match &caps["instruction"] {
                            "acc" => Acc,
                            "jmp" => Jmp,
                            _ => Nop,
                        };
                        let amount: i32 = caps["amount"].parse().unwrap();

                        (ins, amount)
                    }
                    None => (Nop, 1),
                }
            })
            .collect();

        Ok(BootCode { instructions })
    }
}

fn open_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Input not found");

    let mut string = String::new();
    file.read_to_string(&mut string)
        .expect("Error while reading file");

    string
}
