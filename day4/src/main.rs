extern crate common;

use common::file_util;
use std::char;

fn main() {
    println!("Aoc Day 4");
    let data = file_util::read_file("../inputs/day4.txt");
    
    println!("Part 1: {:?}", part_1(&data));
    println!("Part 2: {:?}", part_2(&data));
}

fn part_1(diagnostics: &Vec<String>) -> u64 {
    // parse_diagnostic(diagnostics)
    0
}

fn part_2(diagnostics: &Vec<String>) -> usize {
    // verify_life_support(diagnostics)
    0
}
