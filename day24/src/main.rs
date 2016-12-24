use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::{min, max};

type Grid = Vec<Vec<u32>>;
type Point = (usize, usize);
type Routes = HashMap<(u32, u32), u32>;

#[derive(Debug)]
struct Map {
    layout : Grid,
    goals : HashMap<u32, Point>
}

fn load_input() -> String {
    let mut data = String::new();
    let mut fil = File::open("input.txt").expect("Open file");
    fil.read_to_string(&mut data).expect("Reading file");
    data
}

fn parse_map(s : &str) -> Map {
    let mut layout = Vec::new();
    let mut goals = HashMap::new();

    for (y, l) in s.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in l.chars().enumerate() {
            let wall = if c == '#' { 1 } else { 0 };
            row.push(wall);
            if c.is_digit(10) {
                goals.insert(c as u32 - 48, (x,y));
            }
        }
        layout.push(row);
    }
    Map { layout: layout, goals: goals }
}

fn neighbours((x,y) : Point) -> Vec<Point> {
    let mut out = Vec::new();
    out.push((x-1,y));
    out.push((x+1,y));
    out.push((x,y-1));
    out.push((x,y+1));
    out
}

fn distance(m : &Map, a : Point, b : Point) -> u32 {
    let mut round = 0;
    let mut visited = HashSet::new();
    let mut to_explore = HashSet::new();
    to_explore.insert(a);

    loop {
        let mut next_round = HashSet::new();
        for p in to_explore {
            if p == b {
                return round;
            }
            visited.insert(p);
            for n in neighbours(p) {
                if m.layout[n.1][n.0] == 0 && !visited.contains(&n) {
                    next_round.insert(n);
                }
            }
        }
        round += 1;
        to_explore = next_round;
        if to_explore.is_empty() {
            println!("In a dead end {}", round);
            return 10000;
        }
    }
}

fn find_routes(m : &Map) -> Routes {
    let mut routes = HashMap::new();
    for i in 0..m.goals.len() as u32 {
        for j in i+1..m.goals.len() as u32 {
            let from = m.goals.get(&i).unwrap();
            let to = m.goals.get(&j).unwrap();
            let dist = distance(m, *from, *to);
            routes.insert((i,j), dist);
        }
    }
    routes
}

fn calculate_route(routes : &Routes, path : &Vec<u32>) -> u32 {
    let mut out : u32 = 0;
    for i in 0..path.len()-1 {
        let a = path[i];
        let b = path[i+1];
        let dist = routes.get(&(min(a,b),max(a,b))).unwrap();
        out += *dist;
    }
    out
}

fn next_permutation(v : &mut [u32]) -> bool {
    let len = v.len();
    let mut k = 0;
    let mut has_k = false;
    for i in 0..len-1 {
        if v[i] < v[i+1] {
            k = i;
            has_k = true;
        }
    }
    if !has_k {
        return false;
    }
    let mut l = k+1;
    for j in k+1..len {
        if v[j] > v[k] {
            l = j;
        }
    }
    v.swap(k,l);
    v[k+1..].reverse();
    true
}

fn shortest_route(routes : &Routes, ret : bool) -> u32 {
    let mut v = vec![1,2,3,4,5,6,7];
    let mut shortest = 100000;
    loop {
        let mut w = v.to_vec();
        if ret {
            w.push(0);
        }
        w.insert(0,0);
        let dist = calculate_route(routes, &w);
        shortest = min(shortest, dist);
        if !next_permutation(&mut v) {
            return shortest;
        }
    }
}

fn main() {
    let input = load_input();
    let m = parse_map(&input);
    let r = find_routes(&m);
    let s = shortest_route(&r, false);
    let t = shortest_route(&r, true);
    println!("{} {}", s, t);
}
