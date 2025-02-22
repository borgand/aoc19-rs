pub mod template;

use std::fmt::Display;

// Use this file to add helper functions and additional modules.

// a function to prettyprint a vector of integers, highlighting a position in bold yellow
pub fn pp_vec<T: Display>(data: &Vec<T>, highlights: &[usize]) {
    use colored::*;
    // Define color styles in order of preference
    let styles = [
        |s: String| s.on_yellow().black().to_string(),
        |s: String| s.on_cyan().black().to_string(),
        |s: String| s.on_green().black().to_string(),
        |s: String| s.on_magenta().black().to_string(),
        |s: String| s.on_red().black().to_string(),
    ];
    for (i, x) in data.iter().enumerate() {
        if let Some(pos) = highlights.iter().position(|&r| r == i) {
            let style = &styles[pos % styles.len()];
            print!("{}", style(x.to_string()));
        } else {
            print!("{}", x);
        }
        if i < data.len() - 1 {
            print!(", ");
        }
    }
    println!();
}



// Intcode computer
pub fn intcomp(pc: usize, data: &mut Vec<usize>, debug:bool) -> usize {
    let mut pc = pc;
    let mut opcode = data[pc];
    while opcode != 99 {
        if debug {
            print!("d1: ");
            pp_vec(data, &[pc,pc + 1,pc + 2, pc + 3]);
        }
        let a = data[pc + 1];
        let b = data[pc + 2];
        let c = data[pc + 3];
        match opcode {
            1 => {
                data[c] = data[a] + data[b];
            }
            2 => {
                data[c] = data[a] * data[b];
            }
            _ => {
                panic!("Halt and Catch Fire: {}", opcode);
            }
        }
        if debug {
            print!("d2: ");
            pp_vec(data, &[pc,pc + 1,pc + 2, c]);
            println!();
        }
        pc += 4;
        opcode = data[pc];
    }
    data[0]
}
