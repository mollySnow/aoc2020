use std::fs::read_to_string;

fn main() {
    let s: Vec<_> = read_to_string("./src/input")
        .expect("hi")
        .lines()
        .flat_map(|line| line.parse::<i32>())
        .collect();

    for i in s.iter() {
        for j in s.iter() {
            for k in s.iter() {
                if j + i + k == 2020 {
                    println!("{:?}", j * i * k);
                    return;
                }
            }
        }
    }
}
