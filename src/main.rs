use std::env;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
//mod day16;
mod day17;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod parsing;
//mod grid_board;
mod astar;

fn main() {
    if env::args().len() == 1 {
        println!("{}", day1::day1b(None));
    } else {
        for arg in env::args().skip(1) {
            let result: i64 = match arg.as_str() {
                "day1" => day1::day1(None).into(),
                "day1b" => day1::day1b(None).into(),
                "day2" => day2::day2(None).into(),
                "day2b" => day2::day2b(None).into(),
                "day3" => day3::day3(None).into(),
                "day3b" => day3::day3b(None).into(),
                "day4" => day4::day4(None).into(),
                "day4b" => day4::day4b(None).into(),
                "day5" => day5::day5(None).into(),
                "day5b" => day5::day5b(None).into(),
                "day6" => day6::day6(None).into(),
                "day6b" => day6::day6b(None).into(),
                "day7" => day7::day7(None),
                "day7b" => day7::day7b(None),
                "day8" => day8::day8(None),
                "day8b" => day8::day8b(None),
                "day9" => day9::day9(None),
                "day9b" => day9::day9b(None),
                "day10" => day10::day10(None),
                "day10b" => day10::day10b(None),
                "day11" => day11::day11(None),
                "day11b" => day11::day11b(None),
                "day12" => day12::day12(None),
                "day12b" => day12::day12b(None),
                "day13" => day13::day13(None),
                "day13b" => day13::day13b(None),
                "day14" => day14::day14(None),
                "day14_small" => day14::day14_generic(None, 11, 7, 100),
                "day14b" => day14::day14b(None),
                "day15" => day15::day15(None),
                "day15b" => day15::day15b(None),
                //"day16" => day16::day16(None),
                //"day16b" => day16::day16b(None),
                "day17" => { println!("{}", day17::day17(None)); 0 },
                "day17b" => { println!("{}", day17::day17b(None)); 0 },
                _ => panic!("Skipping unknown test {}", arg),
            };
            println!("{} output: {}", arg, result)
        }
    }
}
