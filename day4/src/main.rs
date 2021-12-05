extern crate common;

use common::file_util;

#[derive(Debug, Clone)]
struct Board {
    marked_numbers: Vec<(usize, (usize, usize))>,
    array: [[ usize; 5]; 5]
}

#[derive(Debug, Clone)]
struct Bingo {
    draws: Vec<usize>,
    boards: Vec<Board>
}

fn main() {
    println!("Aoc Day 4");
    
    println!("Part 1: {:?}", part_1("../inputs/day4.txt"));
    println!("Part 2: {:?}", part_2("../inputs/day4.txt"));
}

fn part_1(path: &str) -> usize {

    let data = file_util::read_file(path);
    let mut game = parse_board_input(&data);
    play_game(&mut game)
}

fn part_2(path: &str) -> usize {

    let data = file_util::read_file(path);
    let mut game = parse_board_input(&data);
    let_the_squid_win(&mut game)
}

fn let_the_squid_win(game: &mut Bingo) -> usize {
    // keep playing till the last board returns a win
        let board = loop {
        match get_winning_board(game) {
            Some(board) => {
                if game.boards.len() == 0 {
                    break Some(board);
                };
            }
            None => {
                simulate_round(game);
            }
        }

    }.unwrap();

    calculate_score(&board)
}

fn play_game(game: &mut Bingo) -> usize { 
    let board = loop {
        match get_winning_board(game) {
            Some(board) => {
                break Some(board); 
            }
            None => {
                simulate_round(game);
            }
        }

    }.unwrap();

    calculate_score(&board)
}


fn simulate_round(game: &mut Bingo) {
    // pop the next number 
    let number = game.draws.pop().unwrap();

    for board in &mut game.boards {
        for row in 0..board.array.len() {
            for col in 0..board.array[0].len() {
                if board.array[row][col] == number {
                    board.marked_numbers.push((number, (row,col)));
                }
            }
        }    
    }
}

fn calculate_score(board: &Board) -> usize {
    // build a list of all non selected values
    let picked_numbers: Vec<usize> = board.marked_numbers.iter().map(|(val, (_,_))| *val).collect();
    let mut remainder_numbers: Vec<usize> = vec![];
    for row in &board.array {
        for col in row {
            if !picked_numbers.contains(col) {
                remainder_numbers.push(*col);
            }
        }
    }

    remainder_numbers.iter().sum::<usize>() * picked_numbers.last().unwrap()
}

fn get_winning_board(game: &mut Bingo) -> Option<Board> {
    // let winning_board: &Board;
    let reference_game = game.clone();
    let board_iterator = &mut reference_game.boards.iter().enumerate();

    let winning_board: Option<usize> = 'outer:loop {
        match board_iterator.next() {
            Some((board_index, board)) => { 

                for index in 0..5 {
                        if board.marked_numbers.iter().filter(|(_, (row, _))| *row == index).collect::<Vec<_>>().len() == 5 {
                            break 'outer Some(board_index)
                        };
            
                        if board.marked_numbers.iter().filter(|(_, (_, col))| *col == index).collect::<Vec<_>>().len() == 5 {
                            break 'outer Some(board_index)
                        };
                    }
            },
            None => break None
        }
    };

    match winning_board {
        Some(index) => {
            let returner = game.boards.get(index).unwrap().clone();
            game.boards.remove(index);
            return Some(returner);
        }
        None => None
    }

}


fn parse_board_input<'a>(input: &'a Vec<String>) -> Bingo {
    let mut iter = input.iter();
    let mut boards: Vec<Board> = vec![];
    let draws = iter.next().unwrap().split(',').map(|a| a.parse::<usize>().unwrap()).rev().collect::<Vec<usize>>();
    
    // skip the first empty line 
    iter.next();

    let mut temp_board: Board = Board{ array: [[0; 5 ]; 5], marked_numbers: vec![]};
    let mut row = 0;
    while let Some(line) = iter.next() {
        let trimmed_line = line.trim();
        match trimmed_line {
            "" => {
                // push the new board onto the outbout
                boards.push(temp_board.clone());
                row = 0;
            },
            _ => {

                for (col, val) in trimmed_line.split_ascii_whitespace().map(|a| a.parse::<usize>().unwrap()).enumerate() {
                    temp_board.array[row][col] = val;
                }
                row+=1;
            }
        }
    }
    
    boards.push(temp_board.clone());

    Bingo {
        boards,
        draws
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_p1() {
        assert_eq!(part_1("../inputs/day4_test.txt"), 4512);
    }
    
    #[test]
    fn test_p2() {
        assert_eq!(part_2("../inputs/day4_test.txt"), 1924);
    }

}
