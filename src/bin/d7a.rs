use std::{fs::read_to_string, usize};

fn main() {
    let input = read_to_string("src/inputs/day7.txt").expect("Failed to load input.");

    let mut answers = Vec::new();
    let mut operands_list = Vec::new();

    for line in input.lines() {
        let answer: usize = line.split(':').next().unwrap().parse().unwrap();
        answers.push(answer);

        let operands: Vec<usize> = line
            .split(':')
            .skip(1)
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .map(|operand| operand.parse::<usize>().unwrap())
            .collect();

        operands_list.push(operands);
    }
}
