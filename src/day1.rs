use regex::Regex;

use crate::parsing::{read_regex_records};


enum Lr {
    LEFT,
    RIGHT
}

struct Action {
    direction: Lr,
    distance: i32
}

fn read_input(source: Option<String>) -> Vec<Action> {
    let regex = Regex::new(r"([LR])(\d+)");
    let records = read_regex_records(source, regex.unwrap());
    let mut result: Vec<Action> = vec![];
    for record in records {
        let action = Action{
            direction: match record[1].as_str() {
                "L" => Lr::LEFT,
                "R" => Lr::RIGHT,
                _ => { panic!("Parsing failure")}
            },
            distance: record[1].parse::<i32>().unwrap()
        };
        result.push(action);
    }
    result
}

pub fn day1(source: Option<String>) -> i32 {
    let records = read_input(source);
    0
}

pub fn day1b(source: Option<String>) -> i32 {
    let records = read_input(source);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(day1(Some("data/day1_example.txt".to_string())), 11);
    }

    #[test]
    #[ignore = "requires input not in repository"]
    fn test_test_1() {
        assert_eq!(day1(Some("inputs/day1_test.txt".to_string())), 1319616);
    }

    #[test]
    fn test_example_1b() {
        assert_eq!(day1b(Some("data/day1_example.txt".to_string())), 31);
    }

    #[test]
    #[ignore = "requires input not in repository"]
    fn test_test_1b() {
        assert_eq!(day1b(Some("inputs/day1_test.txt".to_string())), 27267728);
    }
}
