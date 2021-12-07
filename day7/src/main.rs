extern crate common;

use common::file_util;

fn main() {
    println!("Aoc Day 7");
    
    println!("Part 1: {:?}", part_1("../inputs/day7.txt"));
    println!("Part 2: {:?}", part_2("../inputs/day7.txt"));
}

fn part_1(path: &str) -> usize {
    let data = file_util::read_file(path);
    let list = parse_input(data);
    find_min_fuel_rank(&list, false)
}

fn part_2(path: &str) -> usize {
    let data = file_util::read_file(path);
    let list = parse_input(data);
    find_min_fuel_rank(&list, true)
}

fn parse_input(input: Vec<String>) -> Vec<usize> {
    input.first().unwrap().split(',').map(|a| a.parse::<usize>().unwrap()).collect::<Vec<usize>>()   
}

fn find_min_fuel_rank(positions: &Vec<usize>, modifier: bool) -> usize {
    let &leftmost = positions.iter().min().unwrap();
    let &rightmost = positions.iter().max().unwrap();
    let mut current_min = usize::MAX;

    for rank in leftmost..rightmost {
        let cost = positions.iter().map(|&a| {

            let distance = ( a as i64 - rank as i64 ).abs() as usize;

            if modifier {
                distance * (distance + 1) / 2  
                // (1..=distance).sum::<usize>() 
            } else { 
                distance 
            }
        }).sum();
        current_min = if cost < current_min { cost } else { current_min };
    }

    current_min
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_p1() {
        assert_eq!(part_1("../inputs/day7_test.txt"), 37);
    }
    
    #[test]
    fn test_p2() {
        assert_eq!(part_2("../inputs/day7_test.txt"), 168);
    }
}
