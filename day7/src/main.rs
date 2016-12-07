use std::io::prelude::*;
use std::fs::File;

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
        let has_abba = p.as_bytes().windows(4).any(is_abba);
        if has_abba {
            if i%2 == 0 {
                out = true;
            }
            if i%2 == 1 {
                return false;
            }
        }
    }
    out
}

fn supports_ssl(s : &str) -> bool {
    true
}

fn main() {
    let data = load_input();
    let part1 = data.lines().filter(|s| supports_tls(s)).count();
    let part2 = data.lines().filter(|s| supports_ssl(s)).count();
    println!("{} {}", part1, part2);
}
