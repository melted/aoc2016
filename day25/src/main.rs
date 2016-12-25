use std::io::prelude::*;
use std::fs::File;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Arg {
    Imm(i32),
    Reg(usize)
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Instruction {
    Cpy(Arg, Arg),
    Inc(Arg),
    Dec(Arg),
    Jnz(Arg, Arg),
    Out(Arg)
}

#[derive(Debug, Clone)]
struct Machine {
    program : Vec<Instruction>,
    regs : Vec<i32>,
    pc : i32,
}

fn load_input() -> String {
    let mut data = String::new();
    let mut fil = File::open("input.txt").expect("Open file");
    fil.read_to_string(&mut data).expect("Reading file");
    data
}

fn parse_arg(s :&str) -> Option<Arg> {
    match s {
        "a" => Some(Arg::Reg(0)),
        "b" => Some(Arg::Reg(1)),
        "c" => Some(Arg::Reg(2)),
        "d" => Some(Arg::Reg(3)),
        x => x.parse().ok().map(|n| Arg::Imm(n))
    }
}

fn parse_instruction(s : &str) -> Instruction {
    let parts :Vec<&str> = s.split_whitespace().collect();
    if !parts.is_empty() {
        match parts[0] {
            "cpy" => return Instruction::Cpy(parse_arg(parts[1]).unwrap(), parse_arg(parts[2]).unwrap()),
            "inc" => return Instruction::Inc(parse_arg(parts[1]).unwrap()),
            "dec" => return Instruction::Dec(parse_arg(parts[1]).unwrap()),
            "jnz" => return Instruction::Jnz(parse_arg(parts[1]).unwrap(), parse_arg(parts[2]).unwrap()),
            "out" => return Instruction::Out(parse_arg(parts[1]).unwrap()),
            _ => panic!("Parse fail: {}", s)
        }
    }
    panic!("Parse fail: {}", s);
}

fn init_machine(src : &str) -> Machine {
    let mut program = Vec::new();
    for l in src.lines() {
        program.push(parse_instruction(l));
    }
    Machine { program: program, regs: vec![0,0,0,0], pc: 0 }
}

fn execute(m : &mut Machine, pattern : &[i32]) -> bool {
    let val = |a, regs : &Vec<i32>| match a {
        Arg::Imm(i) => i,
        Arg::Reg(r) => regs[r]
    };
    let mut current_out = 0;
    let mut saved_state : Option<Machine> = None;
    while m.pc < m.program.len() as i32 {
        match m.program[m.pc as usize] {
            Instruction::Cpy(v, Arg::Reg(d)) => m.regs[d] = val(v, &m.regs),
            Instruction::Inc(Arg::Reg(r)) => m.regs[r] += 1,
            Instruction::Dec(Arg::Reg(r)) => m.regs[r] -= 1,
            Instruction::Jnz(a, b) => {
                let v = val(a, &m.regs);
                if v != 0 {
                    m.pc += val(b, &m.regs);
                    continue;
                }
            },
            Instruction::Out(Arg::Reg(r)) => {
                let p = current_out % pattern.len();
                if m.regs[r] != pattern[p] {
                    return false;
                }
                if p == 0 {
                    if let Some(s) = saved_state {
                        if s.regs == m.regs && s.pc == m.pc {
                            return true;
                        } else {
                            saved_state = Some(s);
                        }
                    } else {
                        saved_state = Some(m.clone());
                    }
                }
                current_out += 1;
            }
            _ => {
                // Nop
            }
        }
        m.pc += 1;
    }
    false
}

fn main() {
    let code = load_input();
    let mut counter = 0;
    loop {
        let mut machine = init_machine(&code);
        machine.regs[0] = counter;
        if execute(&mut machine, &[0, 1]) {
            break;
        }
        counter += 1;
    }
    println!("{}", counter);
}
