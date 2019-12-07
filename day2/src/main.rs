use std::fs;

fn int_code(values: &mut Vec<i32>) {
    let len = values.len();
    let mut i = 0;
    loop {
        if i > len {
            break;
        }
        match values[i] {
            1 => {
                let index_to_write = values[i+3] as usize;
                values[index_to_write] = values[values[i+1] as usize] + values[values[i+2] as usize];
                i += 4;
            },
            2 => {
                let index_to_write = values[i+3] as usize;
                values[index_to_write] = values[values[i+1] as usize] * values[values[i+2] as usize];
                i += 4;
            },
            99 => {
                break;
            },
            _ => {
                panic!("Uknow OP code {}", i);
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
    // part 1
    let mut values = original.clone();
    println!("{:?}", values);
    values[1] = 12;
    values[2] = 2;
    int_code(&mut values);
    println!("{:?}", values);
    assert_eq!(values[0], 5290681);
    // part 2
    let mut result: Option<i32> = None;
    'main: for i in 0..99 {
        for j in 0..99 {
            let mut values = original.clone();
            values[1] = i;
            values[2] = j;
            int_code(&mut values);
            if values[0] == 19690720 {
                result = Some(100 * i + j);
                println!("Noun={} Verb={} Result={}", i, j, result.unwrap());
                break 'main;
            }
        }
    }
    assert_eq!(result.unwrap(), 5741);
}
