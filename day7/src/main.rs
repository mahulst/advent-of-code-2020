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

    let recursive_bags = amount_of_bags(&bags, needle_bag, 0);
    // shiny gold bag must not be counted, fold the bags it contains to a total
    let total = recursive_bags
        .contains
        .iter()
        .fold(0, |total, bag| calc_recursive(&bag, 1) + total);

    dbg!(total);
}

fn calc_recursive(bag: &RecursiveBag, amount: usize) -> usize {
    if bag.contains.is_empty() {
        bag.amount
    } else {
        let contains_total = bag
            .contains
            .iter()
            .fold(0, |total, amt| calc_recursive(amt, bag.amount) + total);

        // plus one time the amount of bags inside the parent
        contains_total * bag.amount + bag.amount
    }
}

fn amount_of_bags(
    all_bags: &HashMap<BagColor, Bag>,
    thread_bag: &BagColor,
    amount: usize,
) -> RecursiveBag {
    let bag = all_bags.get(thread_bag).unwrap();
    let contains: Vec<RecursiveBag> = bag
        .holds
        .iter()
        .map(|(bag, amt)| amount_of_bags(all_bags, bag, *amt))
        .collect();

    RecursiveBag { amount, contains }
}

#[derive(Debug, Clone)]
pub struct RecursiveBag {
    amount: usize,
    contains: Vec<RecursiveBag>,
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
