use crate::Place::{Open, Tree};
use core::fmt;
use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let input = open_file("./src/input.txt");

    let tree_pattern = TreePattern {
        lines: input.lines().map(|line| {
            let tree_line: TreeLine = line.parse().unwrap();
            tree_line
        }).collect()
    };

    let mut y: usize = 0;
    let mut x: usize = 0;
    let length = tree_pattern.lines.iter().count();
    let mut count = 0;

    while y < length {
        let line = &tree_pattern.lines[y];
        let place = line.get_x(x);
        if(*place == Tree) {
            count += 1;
        }
        y += 1;
        x += 3;
    }
    dbg!(count);

}

#[derive(Debug, PartialEq, Clone)]
pub struct TreePattern {
    lines: Vec<TreeLine>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Place {
    Tree,
    Open,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TreeLine {
    row: Vec<Place>,
}

impl FromStr for TreeLine {
    type Err = ();
    fn from_str(input: &str) -> Result<TreeLine, ()> {
        let row: Vec<Place> = input
            .chars()
            .map(|c| if c == '#' { Tree } else { Open })
            .collect();

        return Ok(TreeLine { row: row });
        return Err(());
    }
}

impl TreeLine {
    fn get_x(&self, pos: usize) -> &Place {
        let count = self.row.iter().count();
        let x =  pos % count;
        &self.row[x]
    }
}

fn open_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Input not found");

    let mut string = String::new();
    file.read_to_string(&mut string)
        .expect("Error while reading file");

    string
}
