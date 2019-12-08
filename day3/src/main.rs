use std::fs::File;
use std::io::{BufReader, BufRead};

fn create_path(line: &Vec<String>) -> Vec<[i32; 2]> {
    let mut x = 0;
    let mut y = 0;
    let mut path: Vec<[i32; 2]> = Vec::new();
    for mvt in line {
        let dir = mvt.chars().next().unwrap();
        let value: u32 = mvt[1..].parse().unwrap();
        match dir {
            'L' | 'R' | 'U' | 'D' => {
                for _ in 0..value {
                    if dir == 'L' { x -= 1 }
                    else if dir == 'R' { x += 1 }
                    else if dir == 'U' { y += 1 }
                    else { y -= 1 };
                    path.push([x, y]);
                }
            },
            _ => panic!("Got unexpected character!")
        }
    }
    path
}

fn best_cross(path1: Vec<[i32; 2]>, path2: Vec<[i32; 2]>) -> u32 {
    // not efficient, takes a couple of seconds, needs to be optimized
    // (e.g. sort on second key)
    let mut path1 = path1.clone();
    let mut path2 = path2.clone();
    path1.sort_by(|a, b| a[0].cmp(&b[0]));
    path2.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut common: Vec<[i32; 2]> = Vec::new();
    'outer: for c1 in &path1 {
        for c2 in &path2 {
            if c1 == c2 {
                common.push(c1.clone());
            }
            if c2[0] > c1[0] {
                break;
            }
        }
    }
    let mut distances: Vec<i32> = common.into_iter().map(|x| x[0].abs() + x[1].abs()).collect();
    distances.sort_by(|a, b| a.partial_cmp(b).unwrap());
    distances[0] as u32
}

fn main() {
    let file = File::open("inputs.txt").expect("got an error opening the file");
    let buffer = BufReader::new(file);
    let lines: Vec<Vec<String>> = buffer
        .lines()
        .map(|x| x.unwrap().parse().unwrap())
        .map(|x: String| x.trim().to_string())
        .map(|x| x.split(",").map(|s| s.to_string()).collect())
        .collect();
    let mut maps: Vec<Vec<[i32; 2]>> = Vec::new();
    for line in lines {
        maps.push(create_path(&line));
    }
    let best = best_cross(maps[0].clone(), maps[1].clone());
    println!("{}", best);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_create_map() {
        let line = vec!["R8", "U5", "L5", "D3"];
        let line: Vec<String> = line.into_iter().map(|x| x.to_string()).collect();
        let map = create_path(&line);
        assert_eq!(map[map.len() - 1], [3, 2]);
        let line = vec!["U7", "R6", "D4", "L4"];
        let line: Vec<String> = line.into_iter().map(|x| x.to_string()).collect();
        let map = create_path(&line);
        assert_eq!(map[map.len() - 1], [2, 3]);
    }

    #[test]
    fn test_best_cross() {
        let map1 = vec!( 
            [1, 0], [2, 0], [3, 0], [4, 0], [5, 0], [6, 0], [7, 0], [8, 0], [8, 1], [8, 2],
            [8, 3], [8, 4], [8, 5], [7, 5], [6, 5], [5, 5], [4, 5], [3, 5], [3, 4], [3, 3], [3, 2]
        );
        let map2 = vec!(
            [0, 1], [0, 2], [0, 3], [0, 4], [0, 5], [0, 6], [0, 7], [1, 7], [2, 7], [3, 7],
            [4, 7], [5, 7], [6, 7], [6, 6], [6, 5], [6, 4], [6, 3], [5, 3], [4, 3], [3, 3], [2, 3]
        );
        let best = best_cross(map1, map2);
        assert_eq!(best, 6);
    }
}
