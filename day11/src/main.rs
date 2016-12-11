use std::collections::HashSet;

#[derive(Debug, Eq, Hash)]
struct Config {
    chips : Vec<u32>,
    generators : Vec<u32>,
    elevator : u32,
}

impl PartialEq for Config {
     fn eq(&self, other:&Config) -> bool {
         for i in 0..self.chips.len() {
             if self.chips[i] != other.chips[i] ||
                self.generators[i] != other.generators[i] {
                    return false
                }
         }
         self.elevator == other.elevator
     }
}

fn is_legal(c: &Config) -> bool {
    for (i, floor) in c.chips.iter().enumerate() {
        if c.generators[i] != *floor {
            // unprotected chip
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
    for i in 0..c.chips.len() {
        if c.chips[i] == c.elevator {
            // Carry just that
            if c.elevator < 3 {
                let mut up = copy_config(c);
                up.chips[i] += 1;
                up.elevator += 1;
                let up = up;
                if c.generators[i] == c.elevator {
                    let mut with_gen = copy_config(&up);
                    with_gen.generators[i] += 1;
                    out.push(with_gen);
                }
                for j in i+1..c.chips.len() {
                    if c.chips[j] == c.elevator {
                        let mut with_chip = copy_config(&up);
                        with_chip.chips[j] += 1;
                        out.push(with_chip);
                    }
                }
                out.push(up);
            }

            if c.elevator > 0 {
                let mut down = copy_config(c);
                down.chips[i] -= 1;
                down.elevator -= 1;
                let down = down;
                if c.generators[i] == c.elevator {
                    let mut with_gen = copy_config(&down);
                    with_gen.generators[i] -= 1;
                    out.push(with_gen);
                }
                for j in i+1..c.chips.len() {
                    if c.chips[j] == c.elevator {
                        let mut with_chip = copy_config(&down);
                        with_chip.chips[j] -= 1;
                        out.push(with_chip);
                    }
                }
                out.push(down);
            }
        }
        if c.generators[i] == c.elevator {
              // Carry just that
            if c.elevator < 3 {
                let mut up = copy_config(c);
                up.generators[i] += 1;
                up.elevator += 1;
                let up = up;
                for j in i+1..c.generators.len() {
                    if c.generators[j] == c.elevator {
                        let mut with_gen = copy_config(&up);
                        with_gen.generators[j] += 1;
                        out.push(with_gen);
                    }
                }
                out.push(up);
            }

            if c.elevator > 0 {
                let mut down = copy_config(c);
                down.generators[i] -= 1;
                down.elevator -= 1;
                let down = down;
                for j in i+1..c.generators.len() {
                    if c.generators[j] == c.elevator {
                        let mut with_gen = copy_config(&down);
                        with_gen.generators[j] -= 1;
                        out.push(with_gen);
                    }
                }
                out.push(down);
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
                if is_legal(&s) {
                    if visited.contains(&s) {
                        continue;
                    }
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
