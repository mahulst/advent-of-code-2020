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
    let possible_flips = vec![
        436, 108, 486, 461, 134, 284, 100, 6, 61, 376, 473, 40, 130, 137, 69, 642, 296, 582, 447,
        559, 322, 620, 492, 541, 459, 394, 1, 634, 256, 60, 326, 99, 393, 286, 15, 641, 0, 222,
        456, 449, 482, 187, 166, 377, 186, 553, 458, 373, 391, 76, 308, 283, 154, 91, 540, 483,
        435, 446, 402, 493, 129, 309, 264, 382, 161, 336, 285, 37, 109, 89, 388, 585, 261, 434,
        167, 484, 618, 452, 23, 539, 307, 575, 599, 43, 305, 297, 194, 21, 74, 401, 133, 505, 41,
        142, 433, 448, 576, 598, 392, 107, 563, 560, 277, 643, 467, 542, 58, 457, 49, 327, 369,
        278, 302, 491, 511, 295, 110, 318, 136, 471, 561, 399, 538, 462, 101, 372, 455, 504, 337,
        294, 380, 271, 59, 62, 279, 330, 253, 584, 475, 260, 14, 102, 111, 600, 503, 252, 75, 135,
        276, 574, 50, 125, 192, 597, 381, 331, 583, 494, 619, 632, 562, 432, 90, 636, 485, 22, 93,
        185, 513, 34, 568, 240, 141, 35, 36, 495, 335, 605, 319, 400, 466, 250, 577, 306, 239, 275,
        371, 160, 13, 502, 265, 472, 617, 370, 124, 251, 38, 621, 12, 73, 127, 128, 188, 42, 635,
        282, 474, 193, 184, 512, 506, 68, 153, 633, 155, 92, 323,
    ];

    possible_flips.iter().for_each(|flip| {
        let result = find_second_instruction(&boot_code, 0, &mut HashSet::new(), 0, *flip);
        if result != -1 {
            dbg!(result);
        }
    })
}

fn find_second_instruction(
    boot_code: &BootCode,
    next: i32,
    visited_instructions: &mut HashSet<i32>,
    mut acc: i32,
    flip: i32,
) -> i32 {
    if visited_instructions.contains(&next) {
        return -1;
    } else {
        visited_instructions.insert(next);
        match &boot_code.instructions.get(next as usize) {
            Some((ins, amount)) => {
                let ins = if flip == next {
                    match ins {
                        Jmp => &Nop,
                        Nop => &Jmp,
                        Acc => &Acc,
                    }
                } else {
                    ins
                };

                match ins {
                    Jmp => find_second_instruction(
                        boot_code,
                        next + amount,
                        visited_instructions,
                        acc,
                        flip,
                    ),
                    Acc => {
                        acc += amount;
                        find_second_instruction(
                            boot_code,
                            next + 1,
                            visited_instructions,
                            acc,
                            flip,
                        )
                    }
                    Nop => find_second_instruction(
                        boot_code,
                        next + 1,
                        visited_instructions,
                        acc,
                        flip,
                    ),
                }
            }
            None => acc,
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
