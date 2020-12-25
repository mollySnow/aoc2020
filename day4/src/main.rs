use std::collections::HashMap;

fn part1(path: &str) -> usize {
    let f = std::fs::read_to_string(path).unwrap();
    let s = f
        .split("\r\n\r\n")
        .map(|unparsed| {
            unparsed
                .split_whitespace()
                .fold(HashMap::new(), |mut acc, kv| {
                    let mut it = kv.split(':');
                    acc.insert(it.next().unwrap(), it.next().unwrap());
                    acc
                })
        })
        .collect::<Vec<_>>();

    s.iter()
        .filter(|map| {
            if let Some(v) = map.get("byr") {
                let year = v.parse::<i32>().unwrap();
                return year >= 1920 && year <= 2002;
            } else {
                false
            }
        })
        .filter(|map| {
            if let Some(v) = map.get("iyr") {
                let year = v.parse::<i32>().unwrap();
                return year >= 2010 && year <= 2020;
            } else {
                false
            }
        })
        .filter(|map| {
            if let Some(v) = map.get("eyr") {
                let year = v.parse::<i32>().unwrap();
                return year >= 2020 && year <= 2030;
            } else {
                false
            }
        })
        .filter(|map| {
            if let Some(v) = map.get("hgt") {
                let u = v
                    .chars()
                    .filter(|d| d.is_digit(10))
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap();
                if v.ends_with("cm") {
                    u >= 150 && u <= 193
                } else if v.ends_with("in") {
                    u >= 59 && u <= 76
                } else {
                    false
                }
            } else {
                false
            }
        })
        .filter(|map| {
            if let Some(v) = map.get("hcl") {
                let mut it = v.chars();
                if let Some(h) = it.next() {
                    if h != '#' {
                        return false;
                    }
                }
                v.len() == 7 && it.fold(true, |acc, d| acc && d.is_digit(16))
            } else {
                false
            }
        })
        .filter(|map| if let Some(v) = map.get("ecl") {
            v == &"amb" ||
            v == &"blu" ||
            v == &"brn" ||
            v == &"gry" ||
            v == &"grn" ||
            v == &"hzl" ||
            v == &"oth"
        } else { false })
        .filter(|map| if let Some(v) = map.get("pid") {
            match v.parse::<i32>() {
                Ok(_) => v.len() == 9,
                Err(_) => false,
            }
        } else { false })
        .count()
}

fn main() {
    println!("{:?}", part1("input"))
}
