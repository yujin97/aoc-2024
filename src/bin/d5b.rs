use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/inputs/day5.txt").expect("Failed to read input");
    let line_iter = input
        .lines()
        .take_while(|line| !line.is_empty())
        .filter(|line| !line.is_empty());

    let mut cannot_place_before_map = HashMap::new();
    let mut cannot_place_after_map = HashMap::new();
    let mut updates = Vec::new();

    let mut ans = 0usize;

    for line in line_iter {
        let mut split = line.split('|');

        let left: usize = split.next().unwrap().parse().unwrap();
        let right: usize = split.next().unwrap().parse().unwrap();

        if cannot_place_after_map.contains_key(&right) {
            let set: &mut HashSet<usize> = cannot_place_after_map.get_mut(&right).unwrap();

            set.insert(left);
        } else {
            let mut set = HashSet::new();
            set.insert(left);

            cannot_place_after_map.insert(right, set);
        }

        if cannot_place_before_map.contains_key(&left) {
            let set: &mut HashSet<usize> = cannot_place_before_map.get_mut(&left).unwrap();

            set.insert(right);
        } else {
            let mut set = HashSet::new();
            set.insert(right);

            cannot_place_before_map.insert(left, set);
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

        let mut reordered = Vec::new();

        for page in pages.iter() {
            if !forbidden.contains(&page) {
                reordered.push(*page);
                let forbidden_pages = cannot_place_after_map.get(page);

                if forbidden_pages.is_some() {
                    for forbidden_page in forbidden_pages.unwrap() {
                        forbidden.insert(forbidden_page);
                    }
                }
            } else {
                is_correct = false;

                let mut right_of_insertion = reordered.len() - 1;

                loop {
                    if right_of_insertion == 0 {
                        reordered.insert(0, *page);
                        break;
                    }

                    let left_of_insertion = right_of_insertion - 1;

                    let right_forbidden_pages =
                        cannot_place_before_map.get(&reordered[right_of_insertion]);

                    let left_forbidden_pages =
                        cannot_place_after_map.get(&reordered[left_of_insertion]);

                    let is_right_forbidden_pages = if let Some(pages) = right_forbidden_pages {
                        pages.contains(&page)
                    } else {
                        false
                    };

                    let is_left_forbidden_pages = if let Some(pages) = left_forbidden_pages {
                        pages.contains(&page)
                    } else {
                        false
                    };

                    if !is_right_forbidden_pages && !is_left_forbidden_pages {
                        reordered.insert(right_of_insertion, *page);
                        let new_forbidden_pages = cannot_place_after_map.get(&page);
                        if let Some(new_forbidden_pages) = new_forbidden_pages {
                            for new_forbidden_page in new_forbidden_pages {
                                forbidden.insert(new_forbidden_page);
                            }
                        };
                        break;
                    } else {
                        right_of_insertion = left_of_insertion;
                    }
                }
            }
        }

        if !is_correct {
            // add middle number to answer
            ans += reordered[pages.len() / 2];
        }
    }

    println!("The answer is: {}", ans);
}
