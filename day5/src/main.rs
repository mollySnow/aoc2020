fn part1(path: &str) -> Vec<u16> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| { 
            let row = line[..7].chars().enumerate().fold(0, |acc, (i, c)| if c == 'B' { acc + (1 << (6 - i)) } else { acc });
            let col = line[6..].chars().enumerate().fold(0, |acc, (i, c)| if c == 'R' { acc + (1 << (3 - i)) } else { acc });
            row * 8 + col
        })
        .collect::<Vec<_>>()
}

fn main() {
    let mut t = part1("input");
    t.sort();
    let mut s = 47;
    for c in t.iter() {
        if c - s > 1 {
            println!("{}", s);
        }
        s = *c;
    }
    println!("{:?}", t);
}
