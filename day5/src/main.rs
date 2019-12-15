use std::{fs, io};


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

fn read_input() -> i32 {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    trimmed.parse().unwrap()
}

fn int_code(values: &mut Vec<i32>) {
    let len = values.len();
    let mut i = 0;
    loop {
        if i > len {
            break;
        }
        let abcde = format!("{:0>5}", values[i].to_string());
        let de: u32 = abcde[3..5].parse().unwrap();
        let c: u32 = abcde[2..3].parse().unwrap();
        let b: u32 = abcde[1..2].parse().unwrap();
        let a: u32 = abcde[0..1].parse().unwrap();
        let modes = [c, b, a];
        println!("{}", abcde);
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
                values[index_to_write] = read_input(); // prompt input
                i += 2;
            },
            4 => {
                let index_to_read = get_index(&values, &modes, 1, i);
                println!("--> {}", values[index_to_read]);
                i += 2;
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
                break;
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
    println!("{}", text);
    let original: Vec<i32> = text.split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    // part 2 
    let mut values = original.clone();
    println!("{:?}", values);
    int_code(&mut values); // --> 3419022
}
