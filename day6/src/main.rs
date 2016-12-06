use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn load_input() -> String {
    let mut data = String::new();
    let mut fil = File::open("input.txt").expect("Open file");
    fil.read_to_string(&mut data).expect("Reading file");
    data
}

fn parse_input(s : &str) -> Vec<String> {
    let mut out = Vec::new();
    for l in s.lines() {
        for (i, c) in l.chars().enumerate() {
            if i == out.len() {
                out.push(String::new())
            }
            out[i].push(c);
        }
    }
    return out;
}

fn find_most_least_common(s : & str) -> (char, char) {
    let mut histogram : HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        let counter = histogram.entry(c).or_insert(0);
        *counter += 1;
    }

    let v = histogram.iter().min_by_key(|&(_, n)| n).unwrap();
    let u = histogram.iter().max_by_key(|&(_, n)| n).unwrap();
    (*v.0, *u.0)
}

fn main() {
    let data = load_input();
    let v = parse_input(&data);
    let mut part1 = String::new();
    let mut part2 = String::new();

    for s in v.iter() {
        let (a, b) = find_most_least_common(s);
        part1.push(b);
        part2.push(a);
    }

    println!("{} {}", part1, part2);
}
