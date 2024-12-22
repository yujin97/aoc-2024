use std::fs::read_to_string;

fn generate_combination(length: usize) -> Vec<Vec<char>> {
    let operators = vec!['+', '*', '|'];
    if length == 1 {
        return operators
            .into_iter()
            .map(|operator| return vec![operator])
            .collect();
    } else {
        let recursed_combinations = generate_combination(length - 1);
        return recursed_combinations
            .clone()
            .into_iter()
            .map(|recursed_combination| {
                let mut new_combinations = Vec::new();
                for operator in operators.clone() {
                    new_combinations
                        .push(vec![vec![operator], recursed_combination.clone()].concat())
                }
                return new_combinations;
            })
            .collect::<Vec<_>>()
            .concat();
    }
}

fn main() {
    let input = read_to_string("src/inputs/day7.txt").expect("Failed to load input.");

    let mut ans = 0usize;

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

    for (idx, answer) in answers.iter().enumerate() {
        let operands = operands_list[idx].clone();
        let operator_combinations = generate_combination(operands.len() - 1);

        for operator_combination in operator_combinations {
            let mut result = operands[0];

            for i in 1..operands.len() {
                let operator = operator_combination[i - 1];

                match operator {
                    '+' => {
                        result += operands[i];
                    }
                    '*' => {
                        result *= operands[i];
                    }
                    '|' => {
                        let operand_length = operands[i].to_string().len();
                        let result_multiplier = 10usize.pow(
                            operand_length
                                .try_into()
                                .expect("Failed to convert usize to u32"),
                        );

                        result = result * result_multiplier + operands[i];
                    }
                    _ => (),
                }
            }

            if result == *answer {
                ans += answer;
                break;
            }
        }
    }
    println!("The answer is {}", ans);
}
