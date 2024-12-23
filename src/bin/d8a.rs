use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/inputs/day6.txt").expect("Failed to load input.");

    let mut puzzle = Vec::new();

    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }

        puzzle.push(row);
    }

    let mut antinodes = HashSet::new();

    let mut antenna_groups: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for y in 0..puzzle.len() {
        for x in 0..puzzle[0].len() {
            let current = puzzle[y][x];

            if current != '.' {
                let antenna_group = antenna_groups.get_mut(&current);

                match antenna_group {
                    Some(antenna_group) => {
                        let coordinates = (x, y);
                        antenna_group.push(coordinates);
                    }
                    None => {
                        let coordinates = (x, y);
                        antenna_groups.insert(current, vec![coordinates]);
                    }
                }
            }
        }
    }
}
