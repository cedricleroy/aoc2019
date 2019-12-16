use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main() {
    let file = File::open("inputs.txt").expect("got an error opening the file");
    let buffer = BufReader::new(file);
    let results: Vec<Vec<String>> = buffer
        .lines()
        .map(|x| x.unwrap().trim().to_string())
        .map(|x| x.split(")").map(|s| s.to_string()).collect())
        .collect();

    let mut map = HashMap::new();
    for line in results {
        map.insert(line[1].clone(), line[0].clone());
    }
    let mut orbits = 0;
    for key in map.keys() {
        orbits += calculate(0, key, &map);
    }
    println!("{:?}", orbits);
}


fn calculate(steps: u32, name: &str, map: &HashMap<String, String>) -> u32 {
    if name == "COM" {
        return steps;
    }
    let new_steps = steps + 1;
    let new_name = map.get(name).unwrap();
    calculate(new_steps, new_name, map)
}
