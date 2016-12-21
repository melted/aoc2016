extern crate regex;

use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
enum Instr {
    SwapPos(usize, usize),
    SwapLetter(char, char),
    RotL(usize),
    RotR(usize),
    RotPos(char),
    Rev(usize, usize),
    Move(usize, usize),
    RevRotPos(char)
}

impl Instr {
    fn execute(&self, s : &str) -> String {
        let mut v : Vec<char> = s.chars().collect();
        match *self {
            Instr::SwapPos(a, b) => {
                v.swap(a, b);
            },
            Instr::SwapLetter(a, b) => {
                let ai = v.iter().position(|&c| c == a).unwrap();
                let bi = v.iter().position(|&c| c == b).unwrap();
                v.swap(ai, bi);
            },
            Instr::RotL(n) => {
                let m = n % v.len();
                let mut new_v = v[m..].to_vec();
                new_v.extend_from_slice(&v[0..m]);
                v = new_v;
            },
            Instr::RotR(n) => {
                let m = (v.len() - n) % v.len();
                let mut new_v = v[m..].to_vec();
                new_v.extend_from_slice(&v[0..m]);
                v = new_v;
            },
            Instr::RotPos(a) => {
                let i = v.iter().position(|&c| c == a).unwrap();
                let rot = (i + if i > 3 { 2 } else { 1 }) % v.len();
                let m = v.len() - rot;
                let mut new_v = v[m..].to_vec();
                new_v.extend_from_slice(&v[0..m]);
                v = new_v;
            },
            Instr::Rev(a, b) => {
                let mut sub = &mut v[a..b+1];
                sub.reverse();
            },
            Instr::Move(a, b) => {
               let c = v.remove(a);
               v.insert(b, c);
            },
            Instr::RevRotPos(a) => {
                let i = v.iter().position(|&c| c == a).unwrap();
                let m = match i {
                    1 => 1,
                    2 => 6,
                    3 => 2,
                    4 => 7,
                    5 => 3,
                    6 => 0,
                    7 => 4,
                    0 => 1,
                    _ => 0
                };
                let mut new_v = v[m..].to_vec();
                new_v.extend_from_slice(&v[0..m]);
                v = new_v;
            }
        }
        let mut out = String::new();
        for c in v.iter() {
            out.push(*c);
        }
        out
    }
}

fn load_input() -> String {
    let mut data = String::new();
    let mut fil = File::open("input.txt").expect("Open file");
    fil.read_to_string(&mut data).expect("Reading file");
    data
}

fn parse(s : &str) -> Vec<Instr> {
    let swap_pos_re = Regex::new(r"swap position (\d) with position (\d)").unwrap();
    let swap_let_re = Regex::new(r"swap letter (.) with letter (.)").unwrap();
    let rot_re = Regex::new(r"rotate (left|right) (\d*) steps*").unwrap();
    let rot_pos_re = Regex::new(r"rotate based on position of letter (.)").unwrap();
    let rev_re = Regex::new(r"reverse positions (\d) through (\d)").unwrap();
    let move_re = Regex::new(r"move position (\d) to position (\d)").unwrap();

    let mut out = Vec::new();
    for l in s.lines() {
        if let Some(c) = swap_pos_re.captures(l) {
            out.push(Instr::SwapPos(c[1].parse().unwrap(), c[2].parse().unwrap()));
        } else if let Some(c) = swap_let_re.captures(l) {
            out.push(Instr::SwapLetter(c[1].chars().next().unwrap(), c[2].chars().next().unwrap()));
        } else if let Some(c) = rot_re.captures(l) {
            if &c[1] == "left" {
                out.push(Instr::RotL(c[2].parse().unwrap()));
            } else {
                out.push(Instr::RotR(c[2].parse().unwrap()));
            }
        } else if let Some(c) = rot_pos_re.captures(l) {
            out.push(Instr::RotPos(c[1].chars().next().unwrap()));
        } else if let Some(c) = rev_re.captures(l) {
            out.push(Instr::Rev(c[1].parse().unwrap(), c[2].parse().unwrap()));
        } else if let Some(c) = move_re.captures(l) {
            out.push(Instr::Move(c[1].parse().unwrap(), c[2].parse().unwrap()));
        } else {
            println!("Failed match {}", l);
        }
    }
    out
}

fn reverser(v : &Vec<Instr>) -> Vec<Instr> {
    let mut out = Vec::new();
    for i in v.iter().rev() {
        let new_ins = match *i {
            Instr::SwapPos(a, b) => Instr::SwapPos(a, b),
            Instr::SwapLetter(a, b) => Instr::SwapLetter(a, b),
            Instr::RotL(n) => Instr::RotR(n),
            Instr::RotR(n) => Instr::RotL(n),
            Instr::RotPos(c) => Instr::RevRotPos(c),
            Instr::Rev(a, b) => Instr::Rev(a, b),
            Instr::Move(a, b) => Instr::Move(b, a),
            Instr::RevRotPos(c) => Instr::RotPos(c)
        };
        out.push(new_ins);
    }
    out
}


fn main() {
    let input = "abcdefgh";
    let src = load_input();
    let instr = parse(&src);
    let mut s = input.to_string();
    for ins in instr {
        println!("{:?}", ins);
        println!("Before: {}", s);
        s = ins.execute(&s);
        println!("After: {}", s);
    }

    let scrambled = "fbgdceah";
    s = scrambled.to_string();
    let again = parse(&src);
    let unscrambler = reverser(&again);
    for ins in unscrambler {
        println!("{:?}", ins);
        println!("Before: {}", s);
        s = ins.execute(&s);
        println!("After: {}", s);
    }
}
