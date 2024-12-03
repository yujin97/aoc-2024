use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    // parse the input
    let input = read_to_string("src/inputs/day1.txt").unwrap();

    let mut list1: Vec<u64> = Vec::new();
    let mut list2: Vec<u64> = Vec::new();

    let mut count_map: HashMap<u64, u64> = HashMap::new();

    let mut ans = 0;

    for line in input.lines() {
        let mut split = line.split_whitespace();
        list1.push(split.next().unwrap().parse().unwrap());
        list2.push(split.next().unwrap().parse().unwrap());
    }

    for key in list1.iter() {
        if !count_map.contains_key(key) {
            count_map.insert(*key, 0);
        }
    }

    for key in list2.iter() {
        if count_map.contains_key(key) {
            let current_count = count_map.get(key);
            if let Some(&count) = current_count {
                count_map.insert(*key, count + 1);
            }
        }
    }

    for key in list1.iter() {
        let count = count_map
            .get(key)
            .expect("count_map is not initiated probably!");

        ans += *key * *count;
    }

    println!("The similarity score: {}!", ans);
}
