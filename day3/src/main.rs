extern crate common;

use common::file_util;
use std::char;

fn main() {
    println!("Aoc Day 3");
    let data = file_util::read_file("../inputs/day3.txt");
    
    println!("Part 1: {:?}", part_1(&data));
    println!("Part 2: {:?}", part_2(&data));
}

fn part_1(diagnostics: &Vec<String>) -> u64 {
    parse_diagnostic(diagnostics)
}

fn part_2(diagnostics: &Vec<String>) -> usize {
    verify_life_support(diagnostics)
}

fn verify_life_support(diagnostics: &Vec<String>) -> usize {
    // for each rank in the values get the filterable bit
    let rank_size = diagnostics.first().unwrap().len();
    let bin_diagnostics = diagnostics.iter().map(|a| string_to_bin(a) ).collect();
    determine_oxygen_generator_rating(&bin_diagnostics, rank_size) * determine_co2_scrubber_rating(&bin_diagnostics, rank_size)
}


fn determine_co2_scrubber_rating(diagnostics: &Vec<usize>, rank_size: usize) -> usize {
    
    let diagnostics = filter_rank_by_bit( rank_size, diagnostics, false);
    
    *diagnostics.first().unwrap()
}

fn determine_oxygen_generator_rating(diagnostics: &Vec<usize>, rank_size: usize) -> usize {
    
    let diagnostics = filter_rank_by_bit( rank_size, diagnostics, true);
    *diagnostics.first().unwrap()
}

fn filter_rank_by_bit(rank_size: usize, diagnostics: &Vec<usize>, pick_most: bool) -> Vec<usize> {

    let mut filtered_diagnostic = diagnostics.clone();

    for rank in (0..rank_size).rev() {
    
        let filter_bit = get_rank_common_value(rank, &filtered_diagnostic, pick_most);
        // filter by diagnostic codes with the filter bit at that position
        filtered_diagnostic = filtered_diagnostic.iter().filter(|&diag| {
            let out = match filter_bit {
                a if a > 0 => (diag & (1 << rank)),
                0 => (!diag & (1 << rank)),
                _ => panic!('?')
            };
            out > 0
        }).cloned().collect();

        if filtered_diagnostic.len() == 1 {return filtered_diagnostic};
    }

    filtered_diagnostic
    
}


fn get_rank_common_value(rank: usize, diagnostics: &Vec<usize>, pick_most: bool) -> usize {

    let mut one_count = 0;
    let mut zero_count = 0;

    for diagnostic in diagnostics {            
        match diagnostic & 1 << rank {
            a if a > 0  => one_count += 1,
            0  => zero_count += 1,
            _ => {}
        }
    }
    
    match pick_most {
        true => if one_count >= zero_count { 1 } else { 0 }
        false => if one_count < zero_count  { 1 } else { 0 }
    }
}


fn string_to_bin(input: &String) -> usize {
    
    input.split("")
         .filter(|a| !a.is_empty())
         .map(|a| a.parse::<usize>().unwrap())
         .enumerate()
         .map(|(index, value)| {value << (input.len()-1 - index) })
         .fold(0, |acc, val| { val | acc})
   
}


fn parse_diagnostic(diagnostics: &Vec<String>) -> u64 {

    let mut rankwise_diagnostic: Vec<Vec<u64>> = vec![];
    // get size 
    let rank_length = diagnostics.first().unwrap().len();

    for _ in 0..rank_length {
        rankwise_diagnostic.push(vec![])
    }

    // stack rank  values
    for diagnostic in diagnostics {
        
        let chars : Vec<&str> = diagnostic.split("").filter(|a| !a.is_empty()).collect();
        
        for (index, value) in chars.iter().enumerate() {
           
            let digit = value.parse::<u64>().unwrap();

            rankwise_diagnostic[index].push(digit);
        }
    }
    
    // sum ranks and determine gamma / 
    let mut gamma_bin = vec![];
    
    for rank in rankwise_diagnostic {
        let gamma_result: u64 = if rank.iter().sum::<u64>() > (diagnostics.len()/2) as u64 { 1 } else { 0 };
        gamma_bin.push(gamma_result);
    }

    let epsilon_bin: Vec<u64> = gamma_bin.iter().map(|&a| if a == 0 { 1 } else { 0 } ).collect();
    

    let gamma = gamma_bin.iter().rev().enumerate()
                         .map(|(index, value)| { value << index })
                         .fold(0, |acc, val| { val | acc});

    let epsilon = epsilon_bin.iter().rev().enumerate()
                             .map(|(index, value)| { value << index })
                             .fold(0, |acc, val| { acc | val});

    gamma as u64 * epsilon as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_p1() {
        let input = vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010",
        ].iter().map(|a| a.to_string()).collect::<Vec<String>>();
        
        assert_eq!(parse_diagnostic(&input), 198);
    }
    
    #[test]
    fn test_p2() {
        let input = vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010",
        ].iter().map(|a| a.to_string()).collect::<Vec<String>>();
        
        assert_eq!(part_2(&input), 230);
    }

}
