use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use regex::Regex;

fn main() {
    let input = open_file("./src/input.txt");

    let mut valid_passwords = 0;

    input.lines().for_each(|line| {
       let p: PasswordPolicy = line.parse().unwrap();

        let count: i32 = p.password.chars().filter(|c| c == &p.char).count() as i32;

        if(count >= p.min && count <= p.max) {
            valid_passwords += 1;
        }
    });

    dbg!(valid_passwords);
}

#[derive(Debug, PartialEq, Clone)]
pub struct PasswordPolicy {
    pub min: i32,
    pub max: i32,
    pub char: char,
    pub password: String
}


impl FromStr for PasswordPolicy {
    type Err = ();
    fn from_str(input: &str) -> Result<PasswordPolicy, ()> {
        let re =
            Regex::new(r"(?P<min>\d*)-(?P<max>\d*) (?P<character>[a-z]): (?P<password>[a-z]*)").unwrap();

        match re.captures(input) {
            Some(caps) => {
                let result = PasswordPolicy {
                    min: caps["min"].parse().unwrap(),
                    max: caps["max"].parse().unwrap(),
                    char: caps["character"].parse().unwrap(),
                    password: caps["password"].parse().unwrap(),
                };

                Ok(result)
            }
            None => Err(())
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
