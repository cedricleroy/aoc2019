use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("inputs.txt").expect("got an error opening the file");
    let buffer = BufReader::new(file);
    let result: u32 = buffer
        .lines()
        .map(|x| {
            let number: f32 = x.unwrap().parse().unwrap();
            (number / 3.).floor() as u32 - 2
        })
        .sum();
    println!("{}", result);
    assert_eq!(result, 3423511);
}
