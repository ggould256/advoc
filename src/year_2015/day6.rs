use std::fmt::Display;

use log::debug;

use crate::common::grid_board::{Board, Xy};
use crate::common::parsing::read_regex_records;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct LightState {
    on: bool,
    brightness: u32,
}

impl LightState {
    fn turn_on(&self) -> LightState {
        LightState {
            on: true,
            brightness: self.brightness + 1,
        }
    }

    fn turn_off(&self) -> LightState {
        LightState {
            on: false,
            brightness: self.brightness.saturating_sub(1),
        }
    }

    fn toggle(&self) -> LightState {
        LightState {
            on: !self.on,
            brightness: self.brightness + 2,
        }
    }
}

impl From<char> for LightState {
    fn from(c: char) -> Self {
        match c {
            '1' => LightState { on: true, brightness: 1 },
            '0' => LightState { on: false, brightness: 0 },
            _ => panic!("Cannot convert {} to LightState", c),
        }
    }
}
impl From<LightState> for char {
    fn from(val: LightState) -> Self {
        match val.on {
            true => '1',
            false => '0',
        }
    }
}
impl Display for LightState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c: char = (*self).into();
        write!(f, "{}", c)
    }
}

type Lights = Board<LightState>;

enum CommandAction {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Command {
    action: CommandAction,
    from: Xy,
    to: Xy,
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let action_str = match self.action {
            CommandAction::TurnOn => "turn on",
            CommandAction::TurnOff => "turn off",
            CommandAction::Toggle => "toggle",
        };
        write!(f,"{} {},{} through {},{}",
            action_str,
            self.from.x,
            self.from.y,
            self.to.x,
            self.to.y)
    }
}

fn read_input(source: Option<String>) -> Vec<Command> {
    let command_re = regex::Regex::new(
        r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let records = read_regex_records(source, command_re);
    let mut result: Vec<Command> = Vec::new();
    for record in records {
        let action = match record[1].as_str() {
            "turn on" => CommandAction::TurnOn,
            "turn off" => CommandAction::TurnOff,
            "toggle" => CommandAction::Toggle,
            _ => panic!("Unknown action {}", record[1]),
        };
        let from = Xy::new(record[2].parse().unwrap(), record[3].parse().unwrap());
        let to = Xy::new(record[4].parse().unwrap(), record[5].parse().unwrap());
        result.push(Command { action, from, to });  
    }
    result
}

pub fn solution(source: Option<String>) -> (i64, i64) {
    let commands = read_input(source);
    let mut lights = Lights{board: vec![vec![LightState { on: false, brightness: 0 }; 1000]; 1000]};
    for command in &commands {
        debug!("Executing command: {}", command.to_string());
        match command.action {
            CommandAction::Toggle => {
                lights.update_rect(
                    command.from,
                    command.to + Xy::new(1, 1),
                    |v: LightState| v.toggle(),
                );
            }
            CommandAction::TurnOff => {
                lights.update_rect(
                    command.from,
                    command.to + Xy::new(1, 1),
                    |v: LightState| v.turn_off(),
                );

            }
            CommandAction::TurnOn => {
                lights.update_rect(
                    command.from,
                    command.to + Xy::new(1, 1),
                    |v: LightState| v.turn_on(),
                );

            }
        }
    }
    let count = lights.iter().filter(|&(_, light)| light.on).count() as i64;
    let brightness: i64 = lights.iter().map(|(_, light)| light.brightness as i64).sum();
    (count, brightness)
}

pub fn solution_a(source: Option<String>) -> i64 {
    solution(source).0
}

pub fn solution_b(source: Option<String>) -> i64 {
    solution(source).1
}

#[cfg(test)]
mod tests {
    use super::*;

    use const_format::concatcp;
    use log::info;
    use std::fs::File;

    const DAY: &str = "6";
    const EXAMPLE_A_DATA: &str = concatcp!("data/2015/day", DAY, "a_example.txt");
    const EXAMPLE_B_DATA: &str = concatcp!("data/2015/day", DAY, "a_example.txt");
    const INPUT_A_DATA: &str = concatcp!("inputs/2015/day", DAY, "_test.txt");
    const INPUT_B_DATA: &str = concatcp!("inputs/2015/day", DAY, "_test.txt");

    #[test]
    fn test_example_a() {
        assert_eq!(solution_a(Some(EXAMPLE_A_DATA.to_string())), 998996);
    }

    #[test]
    fn test_test_a() {
        if File::open(INPUT_A_DATA).is_err() {
            info!("Skipping test that requires input not in repository");
            return;
        }
        assert_eq!(solution_a(Some(INPUT_A_DATA.to_string())), 400410);
    }

    #[test]
    fn test_example_b() {
        assert_eq!(solution_b(Some(EXAMPLE_B_DATA.to_string())), 1001996);
    }

    #[test]
    fn test_test_b() {
        if File::open(INPUT_B_DATA).is_err() {
            info!("Skipping test that requires input not in repository");
            return;
        }
        assert_eq!(solution_b(Some(INPUT_B_DATA.to_string())), 15343601);
    }
}
