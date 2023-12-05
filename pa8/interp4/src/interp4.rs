use std::{
    env, error, fs,
    io::{self, Read, Write},
};
enum Ops {
    Left,
    Right,
    Add,
    Sub,
    LBrack(usize),
    RBrack(usize),
    Output,
    Input,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut prog = vec![];
    for b in fs::read(env::args().nth(1).unwrap())? {
        match b as char {
            '<' => prog.push(Ops::Left),
            '>' => prog.push(Ops::Right),
            '+' => prog.push(Ops::Add),
            '-' => prog.push(Ops::Sub),
            '[' => prog.push(Ops::LBrack(usize::max_value())),
            ']' => prog.push(Ops::RBrack(usize::max_value())),
            '.' => prog.push(Ops::Output),
            ',' => prog.push(Ops::Input),
            _ => (),
        }
    }

    // Notice: we drop bmap here, since it isn't needed.
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
    let mut cc = 0;
    let mut pc = 0;
    while pc < prog.len() {
        match prog[pc] {
            Ops::Left => cc -= 1,
            Ops::Right => cc += 1,
            Ops::Add => cells[cc] += 1,
            Ops::Sub => cells[cc] -= 1,
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
