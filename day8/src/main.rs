#[macro_use] extern crate lazy_static;
extern crate regex;

use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use std::option::Option::*;

#[derive(Debug)]
enum Cmd {
    Rect(usize, usize),
    RotRow(usize, usize),
    RotCol(usize, usize)
}

fn load_input() -> String {
    let mut data = String::new();
    let mut fil = File::open("input.txt").expect("Open file");
    fil.read_to_string(&mut data).expect("Reading file");
    data
}

fn parse_command(s : &str) -> Option<Cmd> {
    lazy_static!{
        static ref rect_matcher : Regex = Regex::new(r"rect (\d*)x(\d*)").unwrap();
        static ref rotrow_matcher : Regex = Regex::new(r"rotate row y=(\d*) by (\d*)").unwrap();
        static ref rotcol_matcher : Regex = Regex::new(r"rotate column x=(\d*) by (\d*)").unwrap();
    }
    if let Some(c) = rect_matcher.captures(s) {
        return Some(Cmd::Rect(c[1].parse().unwrap(), c[2].parse().unwrap()));
    }
    if let Some(c) = rotrow_matcher.captures(s) {
        return Some(Cmd::RotRow(c[1].parse().unwrap(), c[2].parse().unwrap()));
    } 
    if let Some(c) = rotcol_matcher.captures(s) {
        return Some(Cmd::RotCol(c[1].parse().unwrap(), c[2].parse().unwrap()));
    }
    println!("Parse fail: {:?}", s);
    None
}

fn main() {
    let commands = load_input();
    let mut display : [[u8; 50]; 6] = [[0; 50]; 6];

    for l in commands.lines() {
        let c = parse_command(l).unwrap();
        match c {
            Cmd::Rect(x, y) => {
                for i in 0..x {
                    for j in 0..y {
                        display[j][i] = 1;
                    }
                }
            },
            Cmd::RotRow(r, s) => {
                let row = display[r];
                let mut new_row : [u8; 50] = [0; 50];
                for (i, v) in row.iter().enumerate() {
                    let index = (i + s) % 50;
                    new_row[index] = *v;
                }
                display[r] = new_row; 
            },
            Cmd::RotCol(c, s) => {
                let mut new_col : [u8; 6] = [0; 6];
                for (i, r) in display.iter().enumerate() {
                    let index = (i + s) % 6;
                    new_col[index] = r[c];
                }
                for (i, v) in new_col.iter().enumerate() {
                    display[i][c] = *v;
                }
            }
        }
    }

    let mut count = 0;
    for r in display.iter() {
        for v in r.iter() {
            count += *v;
            print!("{}", if *v == 0 {' '} else { 'x' }); 
        }
        println!("");
    }
    println!("{}", count);
}
