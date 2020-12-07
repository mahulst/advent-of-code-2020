use core::fmt;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::ops::{Add, BitAnd};
use std::str::FromStr;

fn main() {
    let input = open_file("./src/input.txt");

    let bags = input.lines().fold(HashMap::new(), |mut bags, line| {
        let bag: Bag = line.parse().unwrap();
        bags.insert(bag.color.clone() as BagColor, bag);
        bags
    });
    let needle_bag: &BagColor = &"shiny gold".to_string();

   let a = bags.iter().fold(0 as usize, |amount, (bc, bb)| {
        if needle_bag != bc {
            let a = amount_of_bags(&bags, needle_bag, bc);
            if a > 0 {
                amount + 1
            } else {
                amount
            }
        } else {
            amount
        }
    });

    dbg!(a);
}

fn amount_of_bags(
    all_bags: &HashMap<BagColor, Bag>,
    needle_bag: &BagColor,
    thread_bag: &BagColor,
) -> usize {
    let bag = all_bags.get(thread_bag).unwrap();
    bag.holds.iter().fold(0, |total, (bag, amount)| {
        if bag == needle_bag {
            total + amount
        } else {
            amount_of_bags(all_bags, needle_bag, bag) + total
        }
    })
}

type BagColor = String;
type Bags = HashMap<BagColor, usize>;
#[derive(Debug, Clone)]
pub struct Bag {
    color: BagColor,
    pub holds: Bags,
}
impl FromStr for Bag {
    type Err = ();
    fn from_str(input: &str) -> Result<Bag, ()> {
        let re = Regex::new(r"(?P<bag_name>[a-z ]*) bags contain (?P<contains>.*)$").unwrap();

        match re.captures(input) {
            Some(caps) => {
                let color: BagColor = caps["bag_name"].to_string();
                let contains: &str = &caps["contains"];
                let holds =
                    contains
                        .split(",")
                        .fold(HashMap::new(), |mut holds, bag_description| {
                            let re =
                                Regex::new(r" *(?P<amount>\d+) (?P<bag_name>[a-z ]*) bags?.*$")
                                    .unwrap();
                            match re.captures(bag_description) {
                                Some(caps) => {
                                    let test: BagColor = caps["bag_name"].to_string();
                                    let amount: usize = caps["amount"].parse().unwrap();
                                    holds.insert(test, amount);
                                }
                                None => (),
                            };
                            holds
                        });
                Ok(Bag { color, holds })
            }
            None => Err(()),
        }
    }
}

fn open_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Input not found");

    let mut string = String::new();
    file.read_to_string(&mut string)
        .expect("Error while reading file");

    string
}
