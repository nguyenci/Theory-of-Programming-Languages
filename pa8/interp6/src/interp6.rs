use std::{
    env, error, fs,
    io::{self, Read, Write},
};
enum Ops {
    Left(usize),
    Right(usize),
    Add(u8),
    Sub(u8),
    LBrack(usize),
    RBrack(usize),
    Output,
    Input,
}
fn main() -> Result<(), Box<dyn error::Error>> {
    let mut prog = vec![];
    let bytes = fs::read(env::args().nth(1).unwrap())?;
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] as char {
            '<' => {
                let count = bytes[i..].iter().take_while(|&&b| b as char == '<').count() as usize;
                prog.push(Ops::Left(count));
                i += count as usize - 1;
            }
            '>' => {
                let count = bytes[i..].iter().take_while(|&&b| b as char == '>').count() as usize;
                prog.push(Ops::Right(count));
                i += count as usize - 1;
            }
            '+' => {
                let count = bytes[i..].iter().take_while(|&&b| b as char == '+').count() as u8;
                prog.push(Ops::Add(count));
                i += count as usize - 1;
            }
            '-' => {
                let count = bytes[i..].iter().take_while(|&&b| b as char == '-').count() as u8;
                prog.push(Ops::Sub(count));
                i += count as usize - 1;
            }
            '[' => prog.push(Ops::LBrack(usize::max_value())),
            ']' => prog.push(Ops::RBrack(usize::max_value())),
            '.' => prog.push(Ops::Output),
            ',' => prog.push(Ops::Input),
            _ => (),
        }
        i += 1;
    }

    let mut bstack = vec![];
    let mut i = 0;
    while i < prog.len() {
        match prog[i] {
            Ops::LBrack(_) => {
                bstack.push(i);
            }
            Ops::RBrack(_) => {
                let j = bstack.pop().unwrap();
                prog[i] = Ops::RBrack(j);
                prog[j] = Ops::LBrack(i);
            }
            _ => (),
        }
        i += 1;
    }

    let mut cells = vec![0u8; 10000];
    let mut cc = 0usize;
    let mut pc = 0;
    while pc < prog.len() {
        match prog[pc] {
            Ops::Left(count) => cc -= count,
            Ops::Right(count) => cc += count,
            Ops::Add(count) => cells[cc] += count,
            Ops::Sub(count) => cells[cc] -= count,
            Ops::LBrack(jmp_pos) if cells[cc] == 0 => 
                pc = jmp_pos,
            Ops::RBrack(jmp_pos) if cells[cc] != 0 => 
                pc = jmp_pos,
            Ops::Output => io::stdout().write_all(&cells[cc..cc + 1])?,
            Ops::Input => io::stdin().read_exact(&mut cells[cc..cc + 1])?,
            _ => (),
        }
        pc += 1;
    }
    Ok(())
}
