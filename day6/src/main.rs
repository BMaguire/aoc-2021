extern crate common;

use common::file_util;


fn main() {
    println!("Aoc Day 6");
    
    println!("Part 1: {:?}", part_1("../inputs/day6.txt"));
    println!("Part 2: {:?}", part_2("../inputs/day6.txt"));
}

fn part_1(path: &str) -> usize {
    let data = file_util::read_file(path);
    let list = parse_input(data);
    calculate_spawns(&list, 80)
}

fn part_2(path: &str) -> usize {
    let data = file_util::read_file(path);
    let list = parse_input(data);
    calculate_spawns(&list, 256)
}

fn parse_input(input: Vec<String>) -> Vec<usize> {
    input.first().unwrap().split(',').map(|a| a.parse::<usize>().unwrap()).collect::<Vec<usize>>()
    
}

// initial linear approach
fn simulate_days(fish_list: &mut Vec<usize>, days: usize) -> usize {
    for _ in 0..days {
        simulate_day(fish_list);
    }

    fish_list.iter().count()
}

fn simulate_day(fish_list: &mut Vec<usize>) {
    let mut new_fish: Vec<usize> = vec![];
    for fish in fish_list.iter_mut() {
        if *fish == 0 {
            new_fish.push(8);
            *fish = 6;
        } else {
            *fish -= 1;
        }
    }

    // append new fish
    while let Some(fish) = new_fish.pop() {
        fish_list.push(fish)
    }
}


// faster frequency count approach
fn calculate_spawns(initial_fish: &Vec<usize>, days: usize) -> usize {

    let mut fish_per_day: [usize; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];

    for fish in initial_fish {
        fish_per_day[*fish] += 1;
    }

    for _ in 1..=days {
        // shift counts towards index 0
        let respawn = fish_per_day[0];
        fish_per_day[0] = fish_per_day[1];
        fish_per_day[1] = fish_per_day[2];
        fish_per_day[2] = fish_per_day[3];
        fish_per_day[3] = fish_per_day[4];
        fish_per_day[4] = fish_per_day[5];
        fish_per_day[5] = fish_per_day[6];
        fish_per_day[6] = fish_per_day[7];
        fish_per_day[7] = fish_per_day[8];
        
        fish_per_day[6] += respawn;
        fish_per_day[8] = respawn;
    }

    fish_per_day.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_p1() {
        assert_eq!(part_1("../inputs/day6_test.txt"), 5934);
    }
    
    #[test]
    fn test_p2() {
        assert_eq!(part_2("../inputs/day6_test.txt"), 26984457539);
    }

}
