use std::fs::read_to_string;

use regex::Regex;

fn main() {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let input = read_to_string("src/inputs/day3.txt").expect("Failed to load input file!");

    let mut ans = 0usize;

    for (_, [number_1, number_2]) in re.captures_iter(&input).map(|c| c.extract()) {
        let number_1: usize = number_1.parse().unwrap();
        let number_2: usize = number_2.parse().unwrap();

        ans += number_1 * number_2;
    }

    println!("The answer is {}", ans);
}
