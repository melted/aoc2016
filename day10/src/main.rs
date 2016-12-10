#[macro_use] extern crate lazy_static;
extern crate regex;

use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashMap;

#[derive(Debug)]
enum Receiver {
    Output(u32),
    Bot(u32)
}

#[derive(Debug)]
struct Bot {
    val : Option<u32>,
    cmd : (Receiver, Receiver)
}

#[derive(Debug)]
struct World {
    bots :  HashMap<u32, Bot>,
    outputs : HashMap<u32, u32>
}

impl Bot {
    fn create(cmd : (Receiver, Receiver)) -> Bot {
        Bot {
            val: None,
            cmd: cmd
        }
    }
}

impl World {
    fn put_values(&mut self, values : Vec<(u32, u32)>) {
        let mut bot_vals = values.clone();
        while let Some((i, val)) = bot_vals.pop() {
            let mut bot = self.bots.get_mut(&i).unwrap();
            match bot.val {
                None => bot.val = Some(val),
                Some(v) => {
                    let low = min(v, val);
                    let high = max(v, val);
                    if low == 17 && high == 61 {
                        println!("Part 1: {}", i);
                    }
                    match bot.cmd.0 {
                        Receiver::Output(n) => { self.outputs.insert(n, low); },
                        Receiver::Bot(n) => bot_vals.push((n, low))
                    };
                    match bot.cmd.1 {
                        Receiver::Output(n) => { self.outputs.insert(n, high); },
                        Receiver::Bot(n) => bot_vals.push((n, high))
                    };
                }
            }
        }
    }
}

fn load_input() -> String {
    let mut data = String::new();
    let mut fil = File::open("input.txt").expect("Open file");
    fil.read_to_string(&mut data).expect("Reading file");
    data
}

fn parse_input(s : &str, w : &mut World) -> Vec<(u32, u32)> {
    let mut inits = Vec::new();
    lazy_static!{
        static ref init_matcher : Regex = Regex::new(r"^value (\d*) goes to bot (\d*)").unwrap();
        static ref command_matcher : Regex = 
            Regex::new(r"^bot (\d*) gives low to (bot|output) (\d*) and high to (bot|output) (\d*)").unwrap();
    }
    for l in s.lines() {
        if let Some(c) = init_matcher.captures(l) {
            inits.push((c[2].parse().unwrap(), c[1].parse().unwrap()))
        }
        if let Some(c) = command_matcher.captures(l) {
            let id = c[1].parse().unwrap();
            let n1 = c[3].parse().unwrap();
            let n2 = c[5].parse().unwrap();
            let get_receiver = move |c, n| match c {
                                    "bot" => Receiver::Bot(n),
                                    _ => Receiver::Output(n)
                                };
            let r1 = get_receiver(&c[2], n1);
            let r2 = get_receiver(&c[4], n2);
            w.bots.insert(id, Bot::create((r1, r2)));
        }
    }
    inits
}

fn main() {
    let mut world = World { bots: HashMap::new(), outputs: HashMap::new() };
    let data = load_input();
    let inits = parse_input(&data, &mut world);
    world.put_values(inits);
    let mut prod = 1;
    for i in 0..3 {
        prod *=  *world.outputs.get(&i).unwrap();
    }
    println!("Part 2: {}", prod);
}
