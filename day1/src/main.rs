use std::fs::File;
use std::io::Read;

fn main() {
    let input = open_file("./src/input.txt");

    input.lines().for_each(|line| {
        input.lines().for_each(|line2| {
            input.lines().for_each(|line3| {
                if (line.parse::<i32>().unwrap()
                    + line2.parse::<i32>().unwrap()
                    + line3.parse::<i32>().unwrap()
                    == 2020)
                {
                    dbg!(
                        line.parse::<i32>().unwrap()
                            * line2.parse::<i32>().unwrap()
                            * line3.parse::<i32>().unwrap()
                    );
                }
            });
        });
    });
}

fn open_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Input not found");

    let mut string = String::new();
    file.read_to_string(&mut string)
        .expect("Error while reading file");

    string
}
