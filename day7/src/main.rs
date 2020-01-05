use std::fs;
use std::collections::{BTreeMap, HashSet};
use std::iter::FromIterator;


fn get_index(values: &Vec<i32>, modes: &[u32; 3], param: u32, index: usize) -> usize {
    if modes[param as usize - 1] == 0 {
        values[index + param as usize] as usize
    }
    else if modes[param as usize -1] == 1 {
        index + param as usize
    } else {
        panic!("Mode not possible")
    }
}

fn int_code(values: &mut Vec<i32>, mut inputs: Vec<i32>) -> Option<i32> {
    let len = values.len();
    let mut i = 0;
    loop {
        if i > len {
            return None;
        }
        let abcde = format!("{:0>5}", values[i].to_string());
        let de: u32 = abcde[3..5].parse().unwrap();
        let c: u32 = abcde[2..3].parse().unwrap();
        let b: u32 = abcde[1..2].parse().unwrap();
        let a: u32 = abcde[0..1].parse().unwrap();
        let modes = [c, b, a];
        //println!("{}", abcde);
        match de {
            1 => {
                let index_to_write = get_index(&values, &modes, 3, i);
                values[index_to_write] = values[get_index(&values, &modes, 1, i)] + values[get_index(&values, &modes, 2, i)];
                i += 4;
            },
            2 => {
                let index_to_write = get_index(&values, &modes, 3, i);
                values[index_to_write] = values[get_index(&values, &modes, 1, i)] * values[get_index(&values, &modes, 2, i)];
                i += 4;
            },
            3 => {
                let index_to_write = get_index(&values, &modes, 1, i);
                values[index_to_write] = inputs.remove(0);
                i += 2;
            },
            4 => {
                let index_to_read = get_index(&values, &modes, 1, i);
                //println!("--> {}", values[index_to_read]);
                return Some(values[index_to_read]);
            },
            5 => {
                if values[get_index(&values, &modes, 1, i)] != 0 {
                    i = values[get_index(&values, &modes, 2, i)] as usize;
                } else {
                    i += 3;
                }
            },
            6 => {
                if values[get_index(&values, &modes, 1, i)] == 0 {
                    i = values[get_index(&values, &modes, 2, i)] as usize;
                } else {
                    i += 3;
                }
            },
            7 => {
                let index_to_write = get_index(&values, &modes, 3, i);
                if values[get_index(&values, &modes, 1, i)] < values[get_index(&values, &modes, 2, i)] {
                    values[index_to_write] = 1;
                } else {
                    values[index_to_write] = 0;
                }
                i += 4;
            },
            8 => {
                let index_to_write = get_index(&values, &modes, 3, i);
                if values[get_index(&values, &modes, 1, i)] == values[get_index(&values, &modes, 2, i)] {
                    values[index_to_write] = 1;
                } else {
                    values[index_to_write] = 0;
                }
                i += 4;
            }
            99 => {
                return None;
            },
            _ => {
                panic!("Uknow OP code {}", de);
            }
        }
    }
}

fn main() {
    let text = fs::read_to_string("inputs.txt").expect("got an error opening the file");
    let text = text.trim();
    let original: Vec<i32> = text.split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    let mut values = original.clone();

    let mut results = BTreeMap::new();
    let mut settings = vec![0, 0, 0, 0, 0];
    loop {
        let mut signal = 0;
        let set: HashSet<i32> = HashSet::from_iter(settings.clone());
        if set.len() == 5 {
            for i in 0..5 {
                signal = int_code(&mut values, vec![settings[i], signal]).unwrap();
            }
            results.insert(signal, settings.clone());
        }
        if settings == vec![4, 4, 4, 4, 4] {
            break;
        }
        settings[4] += 1;
        for i in (0..5).rev() {
            if settings[i] == 5 {
                settings[i] = 0;
                settings[i-1] += 1;
            }
        }
    }
    println!("{:?}", results.iter().next_back().unwrap());
}
