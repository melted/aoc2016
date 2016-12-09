#[macro_use] extern crate lazy_static;
extern crate regex;

use std::io::prelude::*;
use std::fs::File;
use regex::Regex;

fn load_input() -> String {
    let mut data = String::new();
    let mut fil = File::open("input.txt").expect("Open file");
    fil.read_to_string(&mut data).expect("Reading file");
    data
}

fn decompressed_size(s : &str, shallow : bool) -> u64 {
    let mut count = 0;
    let mut slice = &s[..];

    lazy_static!{
        static ref command_matcher : Regex = Regex::new(r"^\((\d*)x(\d*)\)").unwrap();
    }
    loop {
        let next_cmd = slice.find('(');
        match next_cmd {
            Some(n) => {
                count += n;
                if let Some(c) = command_matcher.captures(&slice[n..]) {
                    let size : usize = c[1].parse().unwrap();
                    let reps : usize = c[2].parse().unwrap();
                    
                    let pos = c.pos(0).unwrap().1 + n;
                    let frag = &slice[pos..pos+size];
                    if shallow {
                        count += reps * frag.len();
                    } else {
                        count += reps * decompressed_size(&frag, shallow) as usize;
                    }
                
                    slice = &slice[pos+size..];
                } else {
                    println!("Match failed {:?} {}", slice, n);
                }
            },
            None => {
                count += slice.trim().len();
                break;
            } 
        }
    }
    count as u64
}

fn main() {
    let data = load_input();
    println!("{} {}", decompressed_size(&data, true), decompressed_size(&data, false));
}
