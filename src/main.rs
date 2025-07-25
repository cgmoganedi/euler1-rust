//! Euler Rust Runner
//!
//! Prints solutions to the first 5 Project Euler-style problems.

mod problems;

use problems::{
    problem01::solve_01,
    problem02::solve_02,
    problem03::solve_03,
    problem04::solve_04,
    problem05::solve_05,
};

fn main() {
    println!("Problem 01: {}", solve_01());
    println!("Problem 02: {}", solve_02());
    println!("Problem 03: {}", solve_03());
    println!("Problem 04: {}", solve_04());
    println!("Problem 05: {}", solve_05());
}
