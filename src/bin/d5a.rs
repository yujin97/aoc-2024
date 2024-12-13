use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/inputs/day5.txt").expect("Failed to read input");
    let line_iter = input
        .lines()
        .take_while(|line| !line.is_empty())
        .filter(|line| !line.is_empty());

    let mut cannot_place_after_map = HashMap::new();
    let mut updates = Vec::new();

    let mut ans = 0usize;

    for line in line_iter {
        let mut split = line.split('|');

        let left: usize = split.next().unwrap().parse().unwrap();
        let right: usize = split.next().unwrap().parse().unwrap();

        if cannot_place_after_map.contains_key(&right) {
            let list: &mut Vec<usize> = cannot_place_after_map.get_mut(&right).unwrap();

            list.push(left);
        } else {
            let list = vec![left];

            cannot_place_after_map.insert(right, list);
        }
    }

    let line_iter = input.lines().skip_while(|line| !line.is_empty()).skip(1);

    for line in line_iter {
        let mut pages = Vec::new();
        for page in line.split(',') {
            let page: usize = page.parse().unwrap();
            pages.push(page);
        }

        updates.push(pages);
    }

    for pages in updates {
        let mut forbidden = HashSet::new();
        let mut is_correct = true;

        for page in &pages {
            if !forbidden.contains(&page) {
                let forbidden_pages = cannot_place_after_map.get(page);

                if forbidden_pages.is_some() {
                    for forbidden_page in forbidden_pages.unwrap() {
                        forbidden.insert(forbidden_page);
                    }
                }
            } else {
                is_correct = false;
                break;
            }
        }

        if is_correct {
            let middle_page_index = pages.len() / 2;

            ans += pages[middle_page_index];
        }
    }

    println!("The answer is: {}", ans);
}
