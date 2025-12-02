mod day1;

type SolutionFn = fn(Option<String>) -> i64;
const NAME_TO_FN : &[(&str, SolutionFn)] = &[
    ("day1a", day1::solution_a),
    ("day1b", day1::solution_b),
];

pub fn run_solution(name: &str, input: Option<String>) -> i64 {
    let solutions: std::collections::HashMap<_, _> = NAME_TO_FN.iter().cloned().collect();
    solutions[name](input)
}
