pub mod template;

use std::fmt::Display;

pub fn is_debug() -> bool {
    std::env::var("DEBUG").is_ok()
}

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


//////////////////////
// Intcode Computer //
//////////////////////

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
//////////////////////////
// End Intcode Computer //
//////////////////////////

//////////////////////////
// Points and Lines     //
//////////////////////////

pub type WorldCoord = i64;
pub type MapCoord = usize;

// define a struct for a point
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

pub const ORIGIN: Point = Point { x: 0, y: 0 };

impl Point {
    // return -1, 0 or 1 depending on the orientation of the points
    pub fn orient(&self, a: Point, b: Point) -> i64 {
       (a.x - self.x) * (b.y - self.y) - (a.y - self.y) * (b.x - self.x).signum()
    }

    // find the Manhattan distance between two points
    pub fn manhattan_distance(&self, other: Point) -> u64 {
        ((self.x - other.x).abs() as u64) + ((self.y - other.y).abs() as u64)
    }

    pub fn manhattan_distance_origin(&self) -> u64 {
        self.manhattan_distance(ORIGIN)
    }

    pub fn to_map_coords(&self, shift: WorldCoord) -> (MapCoord, MapCoord) {
        ((self.x + shift) as MapCoord, (self.y + shift) as MapCoord)
    }
}

// define a struct for a line
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn intersect(&self, other: &Line) -> Option<Point> {
        if self.start == ORIGIN || other.start == ORIGIN {
            return None;
        }
        let a = self.start;
        let b = self.end;
        let c = other.start;
        let d = other.end;

        // if the lines have different orientations towards other points, they intersect
        // assume the lines are not parallel (i.e. there is maximum 1 intersection)
        if (a.orient(b,c) != a.orient(b,d) && c.orient(d,a) != c.orient(d,b)) {
            // calculate the intersection point
            let det = |a: i64, b: i64, c: i64, d: i64| a * d - b * c;
            let x = det(det(a.x, a.y, b.x, b.y), a.x - b.x, det(c.x, c.y, d.x, d.y), c.x - d.x) / det(a.x - b.x, a.y - b.y, c.x - d.x, c.y - d.y);
            let y = det(det(a.x, a.y, b.x, b.y), a.y - b.y, det(c.x, c.y, d.x, d.y), c.y - d.y) / det(a.x - b.x, a.y - b.y, c.x - d.x, c.y - d.y);
            let p = Point { x, y };
            // Check if intersection point lies within both line segments
            if x >= a.x.min(b.x) && x <= a.x.max(b.x) &&
                x >= c.x.min(d.x) && x <= c.x.max(d.x) &&
                y >= a.y.min(b.y) && y <= a.y.max(b.y) &&
                y >= c.y.min(d.y) && y <= c.y.max(d.y) {
                Some(p)
            } else {
                None
            }
        } else {
            None
        }
    }
}



