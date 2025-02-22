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


// define a struct for an instruction
pub struct Instruction {
    pub opcode: usize,
    pub args: usize,
    pub operation: fn(&mut Vec<usize>, usize, usize, usize) -> (),
}

// define a list of intcodes and their count of arguments
pub fn get_intcodes() -> Vec<Instruction> {
    vec![
        // Add
        Instruction { opcode: 1, args: 3,
            operation: |data: &mut Vec<usize>, a: usize, b: usize, c: usize| { data[c] = data[a] + data[b]; },
        },

        // Multiply
        Instruction { opcode: 2, args: 3,
            operation: |data: &mut Vec<usize>, a: usize, b: usize, c: usize| { data[c] = data[a] * data[b]; },
        },

        // Halt
        Instruction { opcode: 99, args: 0,
            operation: |_, _, _, _| (),
        },
    ]
}

// Intcode computer
pub fn intcomp(pc: usize, data: &mut Vec<usize>, debug:bool) -> usize {
    let intcodes = get_intcodes();
    let mut pc = pc;
    let mut opcode = data[pc];
    while opcode != 99 {
        if debug {
            print!("d1: ");
            pp_vec(data, &[pc,pc + 1,pc + 2, pc + 3]);
        }
        if let Some(inst) = intcodes.iter().find(|i| i.opcode == opcode) {
            let a = data[pc + 1];
            let b = data[pc + 2];
            let c = data[pc + 3];
            (inst.operation)(data, a, b, c);

            if debug {
                print!("d2: ");
                pp_vec(data, &[pc, pc+1, pc+2, c]);
                println!();
            }
            pc += inst.args + 1;
        } else {
            panic!("Halt and Catch Fire: {}", opcode);
        }
        opcode = data[pc];
    }
    data[0]
}
