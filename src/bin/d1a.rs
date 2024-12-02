use std::fs::read_to_string;

fn main() {
    // parse the input
    let input = read_to_string("src/inputs/day1.txt").unwrap();

    let mut list1: Vec<i64> = Vec::new();
    let mut list2: Vec<i64> = Vec::new();

    let mut ans = 0;

    for line in input.lines() {
        let mut split = line.split_whitespace();
        list1.push(split.next().unwrap().parse().unwrap());
        list2.push(split.next().unwrap().parse().unwrap());
    }

    list1.sort();
    list2.sort();

    for i in 0..list1.len() {
        let sub = (list1[i] - list2[i]).abs();
        println!("{} - {} = {}", list1[i], list2[i], sub);
        ans += (list1[i] - list2[i]).abs();
    }

    println!("The total distance is: {}!", ans);
}
