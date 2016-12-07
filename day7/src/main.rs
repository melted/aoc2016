use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn load_input() -> String {
    let mut data = String::new();
    let mut fil = File::open("input.txt").expect("Open file");
    fil.read_to_string(&mut data).expect("Reading file");
    data
}

fn is_abba(v : &[u8]) -> bool {
     v[0] == v[3] && v[1] == v[2] && v[0] != v[1]
}

fn supports_tls(s : &str) -> bool {
    let mut out = false;
    for (i, p) in s.split(['[', ']'].as_ref()).enumerate() {
        if p.as_bytes().windows(4).any(is_abba) {
            if i%2 == 1 {
                return false
            }
            out = true;
        }
    }
    out
}

fn supports_ssl(s : &str) -> bool {
    let mut abas = HashMap::new();

    for (i, p) in s.split(['[', ']'].as_ref()).enumerate() {
        for v in p.as_bytes().windows(3) {
            if v[0] == v[2] && v[0] != v[1] {
                let val = if i % 2 == 1 { (v[0], v[1]) } else { (v[1], v[0]) };
                let e = abas.entry(val).or_insert(0);
                *e |= 1 << (i % 2);    
            }
        }
    }
    abas.values().any(|n| *n == 3)
}

fn main() {
    let data = load_input();
    let part1 = data.lines().filter(|s| supports_tls(s)).count();
    let part2 = data.lines().filter(|s| supports_ssl(s)).count();
    println!("{} {}", part1, part2);
}
