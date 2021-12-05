extern crate common;


use itertools::Itertools;
use common::file_util;

fn main() {
    println!("Aoc Day 1");
    let data = file_util::read_to_casted_vec::<u64>("../inputs/day1.txt");
    part_1(&data);
    part_2(&data);
}

fn part_1(data: &Vec<u64>) {
    println!("Part 1");

    // calculate the number of times a depth measurement increases
    println!("Answer {:?}", count_increases(&data));
}

fn part_2(data: &Vec<u64>) {
    println!("Part 2");
    println!("Answer {:?}", count_windowed_increases(&data));
}

fn count_increases(depths: &Vec<u64>) -> u64 {
    depths.iter().tuple_windows().map(|(a,b)| { if a < b { 1 }  else { 0 } }).sum()
}

fn count_windowed_increases(depths: &Vec<u64>) -> u64 {
    depths.iter().tuple_windows::<(_,_,_)>().map(|(a,b,c)| a+b+c ).tuple_windows().map(|(a,b)| { if a < b { 1 }  else { 0 } }).sum()
}
