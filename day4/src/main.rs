fn main() {
    let mut nb = 0;
    'outer: for i in 245182..(790572+1) {
        let s = i.to_string();
        let mut array: [u32; 6] = [0; 6];
        for j in 0..6 {
            array[j] = s[j..(j+1)].parse::<u32>().unwrap();
        }
        let mut equal = false;
        for j in 0..5 {
            if array[j+1] < array[j] {
                continue 'outer;
            }
            else if array[j+1] == array[j] {
                equal = true;
            }
        }
        if equal {
            nb += 1;
        }
    }
    println!("{}", nb);
}
