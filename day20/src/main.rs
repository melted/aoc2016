use std::io::prelude::*;
use std::fs::File;
use std::cmp::max;
use std::u32;

fn load_input() -> String {
    let mut data = String::new();
    let mut fil = File::open("input.txt").expect("Open file");
    fil.read_to_string(&mut data).expect("Reading file");
    data
}

fn parse_input(s : &str) -> Vec<(u32, u32)> {
    let mut out = Vec::new();
    for l in s.lines() {
        let v : Vec<_> = l.split('-').collect();
        let p = (v[0].parse().unwrap(), v[1].parse().unwrap());
        out.push(p);
    }
    coalesce(&mut out);
    out
}

fn coalesce(v : &mut Vec<(u32, u32)>) {
    let mut i = 0;
    v.sort();
    while i+1 < v.len() {
        let (lo, hi) = v[i];
        let (lo2, hi2) = v[i+1];

        if lo2 - 1 <= hi {
            v[i] = (lo, max(hi, hi2));
            v.remove(i+1);
        } else {
            i += 1;
        }
    } 
}

fn count_ips(v : &Vec<(u32, u32)>) -> u32 {
    let mut count = 0;
    for i in 0..v.len()-1 {
        count += v[i+1].0 - v[i].1 - 1;
    }
    count
}

fn main() {
    let input = load_input();
    let ranges = parse_input(&input);
    println!("{} {}", ranges[0].1 + 1, count_ips(&ranges));
}
