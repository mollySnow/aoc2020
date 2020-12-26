
use std::collections;

use collections::{HashMap, binary_heap::PeekMut};

fn part1(path: &str) -> usize {
    std::fs::read_to_string(path).unwrap()
        .split("\r\n\r\n")
        .map(|a| a.split_whitespace().collect::<String>())
        .map(|s| s.chars()
            .fold(HashMap::new(), |mut acc, c| {
                acc.insert(c, 0);
                acc
            }).keys().count()
        )
        .sum()
}

fn part2(path: &str) -> usize {
    std::fs::read_to_string(path).unwrap()
        .split("\r\n\r\n")
        .map(|group| {
            let mut it = group.split("\r\n");
            let map = it.next().unwrap().chars().fold(HashMap::new(), |mut acc,c| { acc.insert(c, c); acc }); 
            it.fold(map, |acc, user| {
                user.chars()
                    .filter_map(|c| acc.get(&c))
                    .fold(HashMap::new(), |mut acc, c| {acc.insert(*c, *c); acc })
            }).keys().count()
        })
        .sum()
}

fn main() {
    println!("{:?}", part2("input"));
}
