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

