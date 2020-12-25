use std::{alloc::Layout, collections::btree_map::Keys};

fn part1() {
    let file = std::fs::read_to_string("./src/input").unwrap();
    let s = file
        .lines()
        .filter(|line| {
            let split = line.split(' ').collect::<Vec<&str>>();
            let a = split.first().unwrap().split('-').collect::<Vec<_>>();
            let b = a.first().unwrap().parse::<u8>().unwrap();
            let c = a.last().unwrap().parse::<u8>().unwrap();
            let x = split[1].chars().next().unwrap();
            let u = split.last().unwrap().chars().filter(|e| *e == x).count() as u8;
            b <= u && c >= u
        })
        .count();
    println!("{:?}", s);
} 

fn part2() {
    let file = std::fs::read_to_string("./src/input").unwrap();
    let s = file
        .lines()
        .filter(|line| {
            let split = line.split(' ').collect::<Vec<&str>>();
            let a = split.first().unwrap().split('-').collect::<Vec<_>>();
            let pos1 = a.first().unwrap().parse::<u8>().unwrap();
            let pos2 = a.last().unwrap().parse::<u8>().unwrap();
            let x = split[1].chars().next().unwrap();
            let u = split.last().unwrap().chars().collect::<Vec<char>>();
            let ca = u[( pos1 - 1 ) as usize];
            let cb = u[( pos2 - 1 ) as usize];

            ca == x && cb != x || ca != x && cb == x
        })
        .count();
    println!("{:?}", s);
} 

fn main() {
    part2();
}
