extern crate regex;

use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use std::collections::HashMap;

fn load_input() -> String {
    let mut data = String::new();
    let mut fil = File::open("input.txt").expect("Open file");
    fil.read_to_string(&mut data).expect("Reading file");
    data
}

#[derive(Debug)]
struct Room (String, i32, String);

fn parse_input(s: &str) -> Vec<Room> {
    let mut out = Vec::new();
    let matcher = Regex::new(r"([a-z-]*)(\d*)\[([a-z]*)\]").unwrap();
    for l in s.lines() {
       if let Some(c) = matcher.captures(l) {
           out.push(Room(c[1].to_string(), c[2].parse().unwrap(), c[3].to_string()));
       } else {
           println!("Failed to match: {}", l);
       }
    }
    out
}

fn is_legit_room(room : &Room) -> bool {
    let mut histogram : HashMap<char, i32> = HashMap::new();
    for c in room.0.chars() {
        if c != '-' {
            let counter = histogram.entry(c).or_insert(0);
            *counter += 1;
        }
    }
    let mut pairs : Vec<(&char, &i32)> = histogram.iter().collect();
    pairs.as_mut_slice().sort_by_key(|&(c, n)| (-n, c));
    for (i, c) in room.2.chars().enumerate() {
        if *pairs[i].0 != c {
            return false;
        }
    }
    true
}

fn main() {
    let data = load_input();
    let rooms = parse_input(&data);

    let summation : i32 = rooms.iter().filter(|r| is_legit_room(&r)).map(|r| r.1).sum();
    println!("{:?}", summation);
}
