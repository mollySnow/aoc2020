fn part1() -> usize {
    std::fs::read_to_string("./src/input").unwrap()
        .lines()
        .enumerate()
        .filter(|(i, line)| line.chars().nth(i * 3 % 31).unwrap() == '#')
        .count()
}

fn part2(right: usize, down: usize) -> usize {
    std::fs::read_to_string("./src/input").unwrap()
        .lines()
        .enumerate()
        .filter_map(|( i, line )| if i % down == 0 { Some(line) } else { None })
        .enumerate()
        .filter(|(i, line)| line.chars().nth(i * right % 31).unwrap() == '#')
        .count()
}

fn main() {
    let s = [(1usize,1usize),(3usize,1usize),(5usize,1usize),(7usize,1usize),(1usize,2usize)]
        .iter()
        .fold(1, |acc, (right, down)| acc * part2(*right, *down));

    println!("{:?}",part1());
    println!("{:?}",part2(1,2));
    println!("{:?}",s);
}
