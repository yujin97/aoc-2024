use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/inputs/day3.txt").unwrap();

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(do)(\(\))|(don't)(\(\))").unwrap();

    let mut skip_next = false;
    let mut ans = 0usize;

    for (_, [match_1, match_2]) in re.captures_iter(&input).map(|c| c.extract()) {
        match match_1 {
            "do" => {
                skip_next = false;
            }
            "don't" => {
                skip_next = true;
            }
            _ => {
                let number_1: usize = match_1.parse().unwrap();
                let number_2: usize = match_2.parse().unwrap();

                if !skip_next {
                    ans += number_1 * number_2;
                }
            }
        }
    }

    println!("The ansswer is {}", ans);
}
