use std::io::prelude::*;
use std::fs::File;

type Reg = usize;

#[derive(Debug)]
enum Instruction {
    CpyImm(i32, Reg),
    Cpy(Reg, Reg),
    Inc(Reg),
    Dec(Reg),
    JnzImm(i32, i32),
    Jnz(Reg, i32)
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

fn parse_reg(s :&str) -> Option<Reg> {
    match s {
        "a" => Some(0),
        "b" => Some(1),
        "c" => Some(2),
        "d" => Some(3),
        _ => None
    }
}

fn parse_instruction(s : &str) -> Instruction {
    let parts :Vec<&str> = s.split_whitespace().collect();
    if !parts.is_empty() {
        match parts[0] {
            "cpy" => {
                let dest = parse_reg(parts[2]).unwrap();
                match parse_reg(parts[1]) {
                    Some(reg) => {
                        return Instruction::Cpy(reg, dest);
                    },
                    None => {
                        let imm : i32 = parts[1].parse().unwrap();
                        return Instruction::CpyImm(imm, dest);
                    }
                }
            },
            "inc" => {
                if let Some(reg) = parse_reg(parts[1]) {
                    return Instruction::Inc(reg)
                }
            },
            "dec" => {
                if let Some(reg) = parse_reg(parts[1]) {
                    return Instruction::Dec(reg);
                }
            },
            "jnz" => {
                let jmp : i32 = parts[2].parse().unwrap();
                match parse_reg(parts[1]) {
                    Some(reg) => { return Instruction::Jnz(reg, jmp); }
                    None => {
                        let imm : i32 = parts[1].parse().unwrap();
                        return Instruction::JnzImm(imm, jmp);
                    }
                }
            }
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
            Instruction::CpyImm(v, d) => m.regs[d] = v,
            Instruction::Cpy(f, d) => m.regs[d] = m.regs[f],
            Instruction::Inc(r) => m.regs[r] += 1,
            Instruction::Dec(r) => m.regs[r] -= 1,
            Instruction::Jnz(c, j) => if m.regs[c] != 0 {
                m.pc += j;
                continue;
            },
            Instruction::JnzImm(c, j) => if c != 0 {
                m.pc += j;
                continue;
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
