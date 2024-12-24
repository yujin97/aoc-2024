use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn get_anitonodes(
    antenna_1: (usize, usize),
    antenna_2: (usize, usize),
    puzzle_width: usize,
    puzzle_height: usize,
) -> Vec<(usize, usize)> {
    let mut antinodes = Vec::new();

    let (x1, y1) = antenna_1;
    let (x2, y2) = antenna_2;

    let x3 = 2 * x1 as isize - x2 as isize;
    let y3 = 2 * y1 as isize - y2 as isize;

    let x4 = 2 * x2 as isize - x1 as isize;
    let y4 = 2 * y2 as isize - y1 as isize;

    if x3 >= 0 && y3 >= 0 && (x3 as usize) < puzzle_width && (y3 as usize) < puzzle_height {
        antinodes.push((x3 as usize, y3 as usize));
    }

    if x4 >= 0 && y4 >= 0 && (x4 as usize) < puzzle_width && (y4 as usize) < puzzle_height {
        antinodes.push((x4 as usize, y4 as usize));
    }

    return antinodes;
}

fn main() {
    let input = read_to_string("src/inputs/day8.txt").expect("Failed to load input.");

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

    let puzzle_width = puzzle[0].len();
    let puzzle_height = puzzle.len();

    for y in 0..puzzle_height {
        for x in 0..puzzle_width {
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

    for (_, antenna_group) in &antenna_groups {
        for i in 0..antenna_group.len() {
            let mut new_antinodes = Vec::new();
            for j in i + 1..antenna_group.len() {
                let antinodes = get_anitonodes(
                    antenna_group[i],
                    antenna_group[j],
                    puzzle_width,
                    puzzle_height,
                );
                for antinode in antinodes {
                    new_antinodes.push(antinode);
                }
            }

            for antinode in new_antinodes {
                antinodes.insert(antinode);
            }
        }
    }

    println!("There are {} unique locations", antinodes.len());
}
