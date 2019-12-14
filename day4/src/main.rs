use std::collections::HashMap;

fn calculate(from: u32, to: u32, part2: bool) -> u32 {
    let mut nb = 0;
    'outer: for i in from..(to+1) {
        let s = i.to_string();
        let mut array: [u32; 6] = [0; 6];
        let mut map = HashMap::new();
        for j in 0..6 {
            array[j] = s[j..(j+1)].parse::<u32>().unwrap();
        }
        for j in 0..5 {
            if array[j+1] < array[j] {
                continue 'outer
            }
            else if array[j+1] == array[j] {
                *map.entry(&array[j]).or_insert(0) += 1;
            }
        }
        if !map.is_empty() {
            if part2 {
                let mut ok = false;
                for val in map.values() {
                    if val == &1 {
                        ok = true;
                        break;
                    }
                }
                if !ok {
                    continue
                }
            }
            nb += 1;
        }
    }
    nb
}

fn main() {
    // part1
    let nb = calculate(245182, 790572, false);
    println!("{}", nb);
    // part2
    let nb = calculate(245182, 790572, true);
    println!("{}", nb);
}
