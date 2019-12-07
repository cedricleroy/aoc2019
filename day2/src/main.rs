use std::fs;

fn int_code(values: &mut Vec<i32>) {
    let len = values.len();
    println!("Length -> {}", len);
    let mut i = 0;
    loop {
        println!("Index -> {}", i);
        if i > len {
            println!("index is out of range: {}", i);
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
                println!("Got 99, finishing");
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
    let mut values: Vec<i32> = text.split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    println!("{:?}", values);
    values[1] = 12;
    values[2] = 2;
    int_code(&mut values);
    println!("{:?}", values);
    assert_eq!(values[0], 5290681);
}
