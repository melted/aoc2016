use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Config {
    chips : Vec<i32>,
    generators : Vec<i32>,
    elevator : i32,
}

fn is_legal(c: &Config) -> bool {
    for (i, floor) in c.chips.iter().enumerate() {
        if c.generators[i] != *floor {
            if c.generators.iter().any(|f| f == floor) {
                return false;
            }
        }
    }
    true
}

fn is_finished(c : &Config) -> bool {
    c.chips.iter().all(|n| *n == 3) && c.generators.iter().all(|n| *n == 3)
}

fn copy_config(c : &Config) -> Config {
    Config { chips: c.chips.to_vec(), generators: c.generators.to_vec(), elevator: c.elevator }
}

fn possible_moves(c : &Config) -> Vec<Config> {
    let mut out = Vec::new();
    let dirs = match c.elevator {
        0 => vec![1],
        3 => vec![-1],
        _ => vec![-1, 1]
    };
    for i in 0..c.chips.len() {
        if c.chips[i] == c.elevator {
            for d in dirs.iter() {
                let mut up = copy_config(c);
                up.chips[i] += *d;
                up.elevator += *d;
                let up = up;
                if c.generators[i] == c.elevator {
                    let mut with_gen = copy_config(&up);
                    with_gen.generators[i] += *d;
                    out.push(with_gen);
                }
                for j in i+1..c.chips.len() {
                    if c.chips[j] == c.elevator {
                        let mut with_chip = copy_config(&up);
                        with_chip.chips[j] += *d;
                        out.push(with_chip);
                    }
                }
                out.push(up);
            }
        }
        if c.generators[i] == c.elevator {
            for d in dirs.iter() {
                let mut up = copy_config(c);
                up.generators[i] += *d;
                up.elevator += *d;
                let up = up;
                for j in i+1..c.generators.len() {
                    if c.generators[j] == c.elevator {
                        let mut with_gen = copy_config(&up);
                        with_gen.generators[j] += *d;
                        out.push(with_gen);
                    }
                }
                out.push(up);
            }
        }
    }
    out
}

fn normalize(c : &Config) -> Config {
    let mut out = copy_config(c);
    for i in 0..out.chips.len() {
        for j in i..out.chips.len() {
            if out.chips[i] > out.chips[j] {
                out.chips.swap(i, j);
                out.generators.swap(i, j)
            } else if out.chips[i] == out.chips[j] && out.generators[i] > out.generators[j] {
                out.generators.swap(i, j);
            }
        }
    }
    out
}

fn search(init : Config) -> u32 {
    let mut candidates = HashSet::new();
    let mut visited = HashSet::new();
    let mut rounds = 0;
    candidates.insert(init);
    loop {
        rounds += 1;
        let mut next_round = HashSet::new();
        for conf in candidates.iter() {
            let moves = possible_moves(&conf);
            visited.insert(normalize(&conf));
            for m in moves {
                let s = normalize(&m);
                if is_legal(&s) && !visited.contains(&s) {
                    if is_finished(&s) {
                        return rounds;
                    }
                    next_round.insert(s);        
                }
            }
        }
        candidates = next_round;
    }
}

fn main() {
    let start = Config { chips: vec![0,2,2,2,2], generators: vec![0,1,1,1,1], elevator: 0 };
    let start2 = Config { chips: vec![0,2,2,2,2,0,0], generators: vec![0,1,1,1,1,0,0], elevator: 0 };
    let moves = search(start);
    let moves2 = search(start2);
    println!("{} {}", moves, moves2);
}
