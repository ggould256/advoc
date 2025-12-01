use std::env;

mod day1;

mod parsing;
//mod grid_board;
mod astar;

fn main() {
    for arg in env::args().skip(1) {
        let result: i64 = match arg.as_str() {
            "day1" => day1::day1(None).into(),
            "day1b" => day1::day1b(None).into(),
            _ => panic!("Skipping unknown test {}", arg),
        };
        println!("{} output: {}", arg, result)
    }
}
