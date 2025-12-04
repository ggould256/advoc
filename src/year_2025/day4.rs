use log::debug;

use crate::common::parsing::read_lines;
use crate::common::grid_board::{Board, Xy};

fn read_input(source: Option<String>) -> Board<char> {
    Board::from_strings(&read_lines(source))
}

fn solutions(source: Option<String>) -> (i64, i64) {
    let mut board = read_input(source);
    let mut first_step_removals: i64 = 0;
    let mut total_removals: i64 = 0;
    let mut removals: Vec<Xy> = Vec::new();
    loop {
        for coord in board.all_coords() {
            if board.at(coord) != '@' {continue;}
            let neighbors = board.neighbors8(coord);
            let occupied_count = neighbors.iter().filter(|c| **c == '@').count();
            if occupied_count < 4 {
                debug!("Location at {:?} is free with {} occupied neighbors", coord, occupied_count);
                total_removals += 1; 
                removals.push(coord);
            }
        }
        if first_step_removals == 0 {
            first_step_removals = total_removals;
        }
        if removals.is_empty() {
            break;
        }
        for coord in removals.drain(..) {
            board.set_at(coord, '_');
        }
    }
    (first_step_removals, total_removals)
}

pub fn solution_a(source: Option<String>) -> i64 {
    solutions(source).0 as i64
}

pub fn solution_b(source: Option<String>) -> i64 {
    solutions(source).1 as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    use const_format::concatcp;
    use log::info;
    use std::fs::File;

    const DAY: &str = "4";
    const EXAMPLE_A_DATA: &str = concatcp!("data/2025/day", DAY, "a_example.txt");
    const EXAMPLE_B_DATA: &str = concatcp!("data/2025/day", DAY, "a_example.txt");
    const INPUT_A_DATA: &str = concatcp!("inputs/2025/day", DAY, "_test.txt");
    const INPUT_B_DATA: &str = concatcp!("inputs/2025/day", DAY, "_test.txt");

    #[test]
    fn test_example() {
        assert_eq!(solution_a(Some(EXAMPLE_A_DATA.to_string())), 13);
    }

    #[test]
    fn test_test_a() {
        if File::open(INPUT_A_DATA).is_err() {
            info!("Skipping test that requires input not in repository");
            return;
        }
        assert_eq!(solution_a(Some(INPUT_A_DATA.to_string())), 1344);
    }

    #[test]
    fn test_example_b() {
        assert_eq!(solution_b(Some(EXAMPLE_B_DATA.to_string())), 43);
    }

    #[test]
    fn test_test_b() {
        if File::open(INPUT_B_DATA).is_err() {
            info!("Skipping test that requires input not in repository");
            return;
        }
        assert_eq!(solution_b(Some(INPUT_B_DATA.to_string())), 8112);
    }
}
