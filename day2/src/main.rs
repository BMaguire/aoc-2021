extern crate common;


use common::file_util;

#[derive(Debug)]
enum Direction {
    FORWARD,
    DOWN,
    UP
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    magnitude: i64
}

#[derive(Debug)]
struct Submarine {
    horiz_pos: i64,
    depth: i64,
    aim: i64
}


fn main() {
    println!("Aoc Day 2");
    let data = file_util::read_file("../inputs/day2.txt");
    let instruction_list = parse_instructions(data);
    println!("Part 1: {:?}", part_1(&instruction_list));
    println!("Part 2: {:?}", part_2(&instruction_list));
}

fn part_1(instructions: &Vec<Instruction>) -> i64 {
    
    let mut sub = Submarine {
        horiz_pos: 0,
        depth: 0,
        aim: 0
    };
    
    simulate_instructions(instructions, &mut sub);

    sub.depth * sub.horiz_pos
    
}

fn part_2(instructions: &Vec<Instruction>) -> i64 {
    
    let mut sub = Submarine {
        horiz_pos: 0,
        depth: 0,
        aim: 0
    };
    
    advanced_simulate_instructions(instructions, &mut sub);

    sub.depth * sub.horiz_pos
}

fn advanced_simulate_instructions(instructions: &Vec<Instruction>, sub: &mut Submarine) {
    for instruction in instructions {
        match instruction.direction {
            Direction::DOWN     => sub.aim += instruction.magnitude,
            Direction::UP       => sub.aim -= instruction.magnitude,
            Direction::FORWARD  => {
                sub.horiz_pos += instruction.magnitude;
                sub.depth += sub.aim * instruction.magnitude
            }
        } 
    }
}

fn simulate_instructions(instructions: &Vec<Instruction>, sub: &mut Submarine) {
    for instruction in instructions {
        match instruction.direction {
            Direction::DOWN     => sub.depth += instruction.magnitude,
            Direction::UP       => sub.depth -= instruction.magnitude,
            Direction::FORWARD  => sub.horiz_pos += instruction.magnitude
        } 
    }
}

fn parse_instructions(input: Vec<String>) -> Vec<Instruction> {

    input.iter().map(|a| a.split(" ") ).map(|mut a|  {
        let direction = match a.next() {
            Some("forward") => Direction::FORWARD,
            Some("up") => Direction::UP,
            Some("down") => Direction:: DOWN,
            _ => panic!("Error occured while parsing")
        };
        let magnitude = a.next().unwrap().parse::<i64>().unwrap();

        Instruction{
            direction,
            magnitude
        }
    } 
    ).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_p1() {
        let input = parse_instructions(vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ].iter().map(|a| a.to_string()).collect());
        
        assert_eq!(part_1(&input), 150);
    }
    
    #[test]
    fn test_p2() {

        let input = parse_instructions(vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ].iter().map(|a| a.to_string()).collect());

        
        assert_eq!(part_2(&input), 900);
    }
}
