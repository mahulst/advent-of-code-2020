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

    let count0 = count_trees(&tree_pattern,  1,  1);
    let count1 = count_trees(&tree_pattern,  3,  1);
    let count2 = count_trees(&tree_pattern,  5,  1);
    let count3 = count_trees(&tree_pattern,  7,  1);
    let count4 = count_trees(&tree_pattern,  1,  2);

    dbg!(count0 * count1 * count2 * count3 * count4);

}

fn count_trees(tree_pattern: &TreePattern, x_dist: usize, y_dist: usize) -> usize {
    let mut y: usize = 0;
    let mut x: usize = 0;
    let length = tree_pattern.lines.iter().count();
    let mut count = 0;

    while y < length {
        let line = &tree_pattern.lines[y];
        let place = line.get_x(x);
        if (*place == Tree) {
            count += 1;
        }
        y += y_dist;
        x += x_dist;
    }
    count
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
