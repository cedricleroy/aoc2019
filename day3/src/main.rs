use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashSet, HashMap};

fn create_path(line: &Vec<String>) -> Vec<[i32; 3]> {
    let mut x = 0;
    let mut y = 0;
    let mut d = 0;
    let mut path: Vec<[i32; 3]> = Vec::new();
    for mvt in line {
        let dir = mvt.chars().next().unwrap();
        let value: u32 = mvt[1..].parse().unwrap();
        match dir {
            'L' | 'R' | 'U' | 'D' => {
                for _ in 0..value {
                    d += 1;
                    if dir == 'L' { x -= 1 }
                    else if dir == 'R' { x += 1 }
                    else if dir == 'U' { y += 1 }
                    else { y -= 1 };
                    path.push([x, y, d]);
                }
            },
            _ => panic!("Got unexpected character!")
        }
    }
    path
}

fn find_cross(path1: Vec<[i32; 3]>, path2: Vec<[i32; 3]>) -> Vec<[i32; 3]> {
    let mut set1: HashSet<[i32; 2]> = HashSet::new();
    let mut set2: HashSet<[i32; 2]> = HashSet::new();
    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();
    for x in &path1 {
        set1.insert([x[0], x[1]]);
        map1.insert([x[0], x[1]], x.clone());
    }
    for x in &path2 {
        set2.insert([x[0], x[1]]);
        map2.insert([x[0], x[1]], x.clone());
    }
    set1.intersection(&set2)
        .map(|x| {
            let x1 = map1.get(x).unwrap();
            let x2 = map2.get(x).unwrap();
            [x1[0], x1[1], x1[2] + x2[2]]
        })
        .collect()
}

fn nearest_cross(points: Vec<[i32; 3]>) -> u32 {
    let mut distances: Vec<i32> = points.iter().map(|x| x[0].abs() + x[1].abs()).collect();
    distances.sort_by(|a, b| a.partial_cmp(b).unwrap());
    distances[0] as u32
}

fn best_cross(points: Vec<[i32; 3]>) -> u32 {
    let mut distances: Vec<i32> = points.iter().map(|x| x[2]).collect();
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
    let mut maps: Vec<Vec<[i32; 3]>> = Vec::new();
    for line in lines {
        maps.push(create_path(&line));
    }
    let points = find_cross(maps[0].clone(), maps[1].clone());
    let nearest = nearest_cross(points.clone());
    println!("{}", nearest);
    // part 2
    let best = best_cross(points);
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
        assert_eq!(map[map.len() - 1], [3, 2, 21]);
        let line = vec!["U7", "R6", "D4", "L4"];
        let line: Vec<String> = line.into_iter().map(|x| x.to_string()).collect();
        let map = create_path(&line);
        assert_eq!(map[map.len() - 1], [2, 3, 21]);
    }

    #[test]
    fn test_find_nearest_cross() {
        let map1 = vec!( 
            [1, 0, 1], [2, 0, 2], [3, 0, 3], [4, 0, 4], [5, 0, 5], [6, 0, 6], [7, 0, 7], [8, 0, 8], [8, 1, 9], [8, 2, 10],
            [8, 3, 11], [8, 4, 12], [8, 5, 13], [7, 5, 14], [6, 5, 15], [5, 5, 16], [4, 5, 17], [3, 5, 18], [3, 4, 19], [3, 3, 20], [3, 2, 21]
        );
        let map2 = vec!(
            [0, 1, 1], [0, 2, 2], [0, 3, 3], [0, 4, 4], [0, 5, 5], [0, 6, 6], [0, 7, 7], [1, 7, 8], [2, 7, 9], [3, 7, 10],
            [4, 7, 11], [5, 7, 12], [6, 7, 13], [6, 6, 14], [6, 5, 15], [6, 4, 16], [6, 3, 17], [5, 3, 18], [4, 3, 19], [3, 3, 20], [2, 3, 21]
        );
        let points = find_cross(map1, map2);
        let nearest = nearest_cross(points);
        assert_eq!(nearest, 6);
    }

    #[test]
    fn test_find_best_cross() {
        let line1 = vec!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"];
        let line1: Vec<String> = line1.into_iter().map(|x| x.to_string()).collect();
        let line2 = vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"];
        let line2: Vec<String> = line2.into_iter().map(|x| x.to_string()).collect();
        let map1 = create_path(&line1);
        let map2 = create_path(&line2);
        let points = find_cross(map1, map2);
        let best = best_cross(points);
        assert_eq!(best, 610);
    }
}
