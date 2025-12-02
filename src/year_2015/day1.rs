use crate::common::parsing::read_one_string;

#[derive(Debug)]
enum Ud {
    Up,
    Down,
}

fn read_input(source: Option<String>) -> Vec<Ud> {
    let tokens = read_one_string(source);
    let mut result = vec!();
    for token in tokens.chars() {
        match token {
            ')' => result.push(Ud::Down),
            '(' => result.push(Ud::Up),
            _ => { panic!("parser error"); }
        }
    }
    result
}

pub fn day1(source: Option<String>) -> (i64, i64) {
    let mut final_floor: i64 = 0;
    let mut basement_time: Option<i64> = None;
    for (index, ud) in read_input(source).iter().enumerate() {
        match ud {
            Ud::Up => { final_floor += 1; }
            Ud::Down => { final_floor -= 1; }
        }
        if final_floor < 0 && basement_time.is_none() {
            basement_time = Some(index as i64 + 1);
        }
    }
    (final_floor, basement_time.unwrap())
}

pub fn solution_a(source: Option<String>) -> i64 {
    day1(source).0 as i64
}

pub fn solution_b(source: Option<String>) -> i64 {
    day1(source).1 as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    use const_format::concatcp;
    use log::info;
    use std::fs::File;

    const DAY: &str = "1";
    const EXAMPLE_A_DATA: &str = concatcp!("data/2015/day", DAY, "a_example.txt");
    const EXAMPLE_B_DATA: &str = concatcp!("data/2015/day", DAY, "a_example.txt");
    const INPUT_A_DATA: &str = concatcp!("inputs/2015/day", DAY, "_test.txt");
    const INPUT_B_DATA: &str = concatcp!("inputs/2015/day", DAY, "_test.txt");

    #[test]
    fn test_example_1() {
        assert_eq!(solution_a(Some(EXAMPLE_A_DATA.to_string())), -3);
    }

    #[test]
    fn test_test_1() {
        if File::open(INPUT_A_DATA).is_err() {
            info!("Skipping test that requires input not in repository");
            return;
        }
        assert_eq!(solution_a(Some(INPUT_A_DATA.to_string())), 232);
    }

    #[test]
    fn test_example_1b() {
        assert_eq!(solution_b(Some(EXAMPLE_B_DATA.to_string())), 1);
    }

    #[test]
    fn test_test_1b() {
        if File::open(INPUT_B_DATA).is_err() {
            info!("Skipping test that requires input not in repository");
            return;
        }
        assert_eq!(solution_b(Some(INPUT_B_DATA.to_string())), 1783);
    }
}
