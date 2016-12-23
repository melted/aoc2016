use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
enum Arg {
    Imm(i32),
    Reg(usize)
}

#[derive(Debug)]
enum Instruction {
    Cpy(Arg, Arg),
    Inc(Arg),
    Dec(Arg),
    Jnz(Arg, Arg),
    Tgl(Arg)
}

#[derive(Debug)]
struct Machine {
    program : Vec<Instruction>,
    regs : Vec<i32>,
    pc : i32
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
            "cpy" => {
                return Instruction::Cpy(parse_arg(parts[1]).unwrap(), parse_arg(parts[2]).unwrap())
            },
            "inc" => {
                if let Some(reg) = parse_arg(parts[1]) {
                    return Instruction::Inc(reg)
                }
            },
            "dec" => {
                if let Some(reg) = parse_arg(parts[1]) {
                    return Instruction::Dec(reg);
                }
            },
            "jnz" => {
                return Instruction::Jnz(parse_arg(parts[1]).unwrap(), parse_arg(parts[2]).unwrap())
            }
            "tgl" => {
                if let Some(reg) = parse_arg(parts[1]) {
                    return Instruction::Tgl(reg);
                }
            },
            _ => {
                panic!("Parse fail: {}", s);
            }
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

fn execute(m : &mut Machine) {
    while m.pc < m.program.len() as i32 {
        match m.program[m.pc as usize] {
            Instruction::Cpy(Arg::Imm(v), Arg::Reg(d)) => { 
                m.regs[d] = v
            },
            Instruction::Cpy(Arg::Reg(r), Arg::Reg(d)) => { 
                m.regs[d] = m.regs[r]
            },
            Instruction::Inc(Arg::Reg(r)) => m.regs[r] += 1,
            Instruction::Dec(Arg::Reg(r)) => m.regs[r] -= 1,
            Instruction::Jnz(Arg::Reg(c), Arg::Imm(j)) => if m.regs[c] != 0 {
                m.pc += j;
                continue;
            },
            Instruction::Jnz(Arg::Imm(c), Arg::Imm(j)) => if c != 0 {
                m.pc += j;
                continue;
            },
            Instruction::Tgl(Arg::Reg(r)) => {
                let target = (m.pc + m.regs[r]) as usize;
                if target < m.program.len() {
                    match m.program[target] {
                        _ => {}
                    }
                }
            }
            _ => {
                // Nop
            }
        }
        m.pc += 1;
    }
}

fn main() {
    let code = load_input();
    let mut machine = init_machine(&code);
    execute(&mut machine);
    println!("{:?}", machine.regs[0]);

    let mut machine2 = init_machine(&code);
    machine2.regs[2] = 1;
    execute(&mut machine2);
    println!("{:?}", machine2.regs[0]);
}
