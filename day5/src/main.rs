extern crate common;

use common::file_util;
use std::collections::HashMap;

fn main() {
    println!("Aoc Day 5");
    
    println!("Part 1: {:?}", part_1("../inputs/day5.txt"));
    println!("Part 2: {:?}", part_2("../inputs/day5.txt"));
}

fn part_1(path: &str) -> usize {

    let data = file_util::read_file(path);
    let line_map = parse_line_inputs(data, false);
    calculate_intersections(&line_map, 1)
}

fn part_2(path: &str) -> usize {

    let data = file_util::read_file(path);
    let line_map = parse_line_inputs(data, true);
    calculate_intersections(&line_map, 1)
}


fn calculate_intersections(line_map: &HashMap<(usize,usize), usize>, min_intersections: usize) -> usize {
    line_map.values().filter(|&a| *a > min_intersections).map(|_| 1).sum::<usize>() 
}
// return a map of line freq
fn parse_line_inputs(inputs: Vec<String>, consider_diagonal: bool) -> HashMap<(usize,usize), usize> {
    let mut line_map = HashMap::new();


    for line in inputs {
        let mut segments = line.split(" -> ");
        let first_segment = segments.next().unwrap().split(',').map(|a| a.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let second_segment = segments.next().unwrap().split(',').map(|a| a.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        
        let point_1 = (*first_segment.first().unwrap(), *first_segment.last().unwrap());
        let point_2 = (*second_segment.first().unwrap(), *second_segment.last().unwrap());
        

        if consider_diagonal {
            populate_line_map(&mut line_map, point_1, point_2);
        } else if point_1.0 == point_2.0 || point_1.1 == point_2.1 {
            populate_line_map(&mut line_map, point_1, point_2);
        }
    }

    line_map
}

struct LineIterator {
    end: (usize, usize),
    current_pos: (usize,usize), 
    done: bool
}

impl Iterator for LineIterator {
    type Item = (usize, usize);
    // let last_x = 0;

    fn next(&mut self) -> Option<(usize, usize)> {
        if self.done { 
            None            
        } else if self.current_pos.0 == self.end.0 && self.current_pos.1 == self.end.1 {
            self.done = true;
            Some(self.current_pos)
        } else {
            let prev_position = self.current_pos;
            if self.end.0 != self.current_pos.0 {
                if self.end.0 > self.current_pos.0 { self.current_pos.0+=1 } else {self.current_pos.0-=1};
            }
            if self.end.1 != self.current_pos.1 {
                if self.end.1 > self.current_pos.1 { self.current_pos.1+=1 } else {self.current_pos.1-=1};
            }
            Some(prev_position)
        }
    }
}

fn populate_line_map(line_map: &mut HashMap<(usize,usize), usize>, point_1: (usize,usize), point_2: (usize, usize)) {

    let mut path = LineIterator{ end: point_2, current_pos: point_1, done: false};
    
    loop {
        // works because they have to be at 45 degrees 
        match path.next() {
            Some((x,y)) => {
                update_map(line_map, x, y);
            },
            None => { break}
        };        
    };

}

fn update_map(line_map: &mut HashMap<(usize,usize), usize>, x: usize, y:usize) {
    match line_map.get_mut(&(x,y)) {
        Some(item) => {
            *item+=1;
        },
        None => { 
            line_map.insert((x,y), 1);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_p1() {
        assert_eq!(part_1("../inputs/day5_test.txt"), 5);
    }
    
    #[test]
    fn test_p2() {
        assert_eq!(part_2("../inputs/day5_test.txt"), 12);
    }

}
