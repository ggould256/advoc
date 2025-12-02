use regex::Regex;

use log::debug;

use crate::common::parsing::read_lines;

type Id = usize;
#[derive(Debug)]
struct IdRange {
    start: Id,
    end: Id,
}

fn read_input(source: Option<String>) -> Vec<IdRange> {
    let mut result: Vec<IdRange> = vec!();
    let id_re: Regex = Regex::new(r"(\d+)-(\d+)").unwrap();
    debug!("Starting read");
    for line in read_lines(source) {
        debug!("Parsing line {}", line);
        for range in line.split(",") {
            let re_match = id_re.captures(range);
            let re_match = match re_match {
                Some(m) => m,
                _ => { debug!("discarding: {}", range); continue; }
            };
            let range = IdRange{
                start: re_match[1].parse::<Id>().unwrap(),
                end: re_match[2].parse::<Id>().unwrap()};
            debug!("Parsed {:?}", range);
            result.push(range);
        }
    }
    result
}

pub fn day2(source: Option<String>) -> (i64, i64) {
    let records = read_input(source);
    let mut repeat_accumulator: i64 = 0;
    let mut exactly_two_accumulator: i64 = 0;
    for record in records {
        for id in record.start..=record.end {
            let id_str = id.to_string();
            let mut accept = false;
            for substr_len in 1..=id_str.len() / 2 {
                let num_repeats = id_str.len() / substr_len;
                if id_str.len() != num_repeats * substr_len { continue; }
                let pattern = &id_str[0..substr_len];
                let mut ok = true;
                for i in 0..num_repeats {
                    if id_str[substr_len*i..substr_len*(i+1)] != *pattern { ok=false; }
                }
                if ok { 
                    accept = true;
                    debug!("Accepted: {} as {} x {}", id, num_repeats, pattern);
                    if num_repeats==2 { exactly_two_accumulator += id as i64; }
                }
            }
            if accept {
                debug!("Accepted: {}", id); 
                repeat_accumulator += id as i64;
            }
        }
    }
    (exactly_two_accumulator, repeat_accumulator)
}

pub fn solution_a(source: Option<String>) -> i64 {
    day2(source).0 as i64
}

pub fn solution_b(source: Option<String>) -> i64 {
    day2(source).1 as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    use const_format::concatcp;
    use log::warn;
    use std::fs::File;

    const DAY: &str = "2";
    const EXAMPLE_A_DATA: &str = concatcp!("data/2025/day", DAY, "a_example.txt");
    const EXAMPLE_B_DATA: &str = concatcp!("data/2025/day", DAY, "a_example.txt");
    const INPUT_A_DATA: &str = concatcp!("inputs/2025/day", DAY, "_test.txt");
    const INPUT_B_DATA: &str = concatcp!("inputs/2025/day", DAY, "_test.txt");

    #[test]
    fn test_example_1() {
        assert_eq!(solution_a(Some(EXAMPLE_A_DATA.to_string())), 1227775554);
    }

    #[test]
    fn test_test_1() {
        if File::open(INPUT_A_DATA).is_err() {
            warn!("Skipping test that requires input not in repository");
            return;
        }
        assert_eq!(solution_a(Some(INPUT_A_DATA.to_string())), 38158151648);
    }

    #[test]
    fn test_example_1b() {
        assert_eq!(solution_b(Some(EXAMPLE_B_DATA.to_string())), 4174379265);
    }

    #[test]
    fn test_test_1b() {
        if File::open(INPUT_B_DATA).is_err() {
            warn!("Skipping test that requires input not in repository");
            return;
        }
        assert_eq!(solution_b(Some(INPUT_B_DATA.to_string())), 45283684555);
    }
}
