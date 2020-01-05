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
    // part1
    let mut orbits = 0;
    for key in map.keys() {
        orbits += calculate_path(0, key, &map, Vec::new()).last().unwrap().1 + 1;
    }
    println!("{:?}", orbits);
    // part2
    let my_path = calculate_path(0, "YOU", &map, Vec::new());
    let santa_path = calculate_path(0, "SAN", &map, Vec::new());
    'outer: for (key1, steps1) in &my_path {
        for (key2, steps2) in &santa_path {
            if key1 == key2 {
                println!("{:?}", key1);
                println!("{:?}", steps1 + steps2 - 2);
                break 'outer;
            }
        }
    }
}


fn calculate_path(steps: u32, name: &str, map: &HashMap<String, String>, mut path: Vec<(String, u32)>) -> Vec<(String, u32)> {
    if name == "COM" {
        return path;
    }
    path.push((name.to_string(), steps));
    let new_steps = steps + 1;
    let new_name = map.get(name).unwrap();
    calculate_path(new_steps, new_name, map, path)
}

