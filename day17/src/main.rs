extern crate md5;

type Pos = (i32, i32);

#[derive(Debug)]
struct State {
    path : String,
    pos : Pos
}

fn moves(passcode : &str, s : &State) -> Vec<State> { 
    let hash = md5::compute(format!("{}{}", passcode, s.path));
    let mut new_states = Vec::new();
    if s.pos.1 > 0 && hash[0] & 0xf0 > 0xa0 {
        new_states.push(State { path: String::from(s.path.clone() + "U"), pos: (s.pos.0, s.pos.1 - 1) });
    }
    if s.pos.1 < 3 && hash[0] & 0x0f > 0x0a {
        new_states.push(State { path: String::from(s.path.clone() + "D"), pos: (s.pos.0, s.pos.1 + 1) });
    }
    if s.pos.0 > 0 && hash[1] & 0xf0 > 0xa0 {
        new_states.push(State { path: String::from(s.path.clone() + "L"), pos: (s.pos.0 - 1, s.pos.1) });
    }
    if s.pos.0 < 3 && hash[1] & 0x0f > 0x0a {
        new_states.push(State { path: String::from(s.path.clone() + "R"), pos: (s.pos.0 + 1, s.pos.1) });
    }
    new_states
}

fn find_path(passcode : &str, longest : bool) -> String {
    let mut states = Vec::new();
    let mut paths = Vec::new();
    states.push(State { path: "".to_string(), pos: (0,0) }); 
    loop {
        let mut next_states = Vec::new();
        for st in states {
            let steps = moves(passcode, &st);
            for step in steps {
                if step.pos == (3,3) {
                    if !longest {
                        return step.path;
                    } else {
                        paths.push(step.path);
                    }
                } else {
                    next_states.push(step);
                }
            }
        }
        states = next_states;
        if states.is_empty() {
            return paths.iter().max_by_key(|p| p.len()).unwrap().clone();
        }
    }
}

fn main() {
    let input = "lpvhkcbi";
    println!("{}", find_path(input, false));
    println!("{}", find_path(input, true).len());
}
