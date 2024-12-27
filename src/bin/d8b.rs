use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

enum SlopeValue {
    Positive,
    Negative,
    Zero,
    Infinite,
}

fn get_absolute_distant(p1: usize, p2: usize) -> usize {
    if p1 > p2 {
        p1 - p2
    } else {
        p2 - p1
    }
}

fn is_valid_position(x: isize, y: isize, puzzle_width: usize, puzzle_height: usize) -> bool {
    x >= 0 && x < puzzle_width as isize && y >= 0 && y < puzzle_height as isize
}

fn get_antinodes_by_slope(
    antenna_1: (usize, usize),
    antenna_2: (usize, usize),
    slope_value: SlopeValue,
    x_distance: usize,
    y_distance: usize,
    puzzle_width: usize,
    puzzle_height: usize,
) -> Vec<(usize, usize)> {
    use SlopeValue::*;

    let mut antinodes = Vec::new();

    let (right_x, right_y) = antenna_1;
    let (left_x, left_y) = antenna_2;

    let x_distance = x_distance as isize;
    let y_distance = y_distance as isize;

    let mut right_x = right_x as isize;
    let mut right_y = right_y as isize;

    let mut left_x = left_x as isize;
    let mut left_y = left_y as isize;

    match slope_value {
        Positive => {
            loop {
                if is_valid_position(right_x, right_y, puzzle_width, puzzle_height) {
                    antinodes.push((right_x as usize, right_y as usize));
                    right_x += x_distance;
                    right_y += y_distance;
                } else {
                    break;
                }
            }
            loop {
                if is_valid_position(left_x, left_y, puzzle_width, puzzle_height) {
                    antinodes.push((left_x as usize, left_y as usize));
                    left_x -= x_distance;
                    left_y -= y_distance;
                } else {
                    break;
                }
            }
        }
        Negative => {
            loop {
                if is_valid_position(right_x, right_y, puzzle_width, puzzle_height) {
                    antinodes.push((right_x as usize, right_y as usize));
                    right_x += x_distance;
                    right_y -= y_distance;
                } else {
                    break;
                }
            }
            loop {
                if is_valid_position(left_x, left_y, puzzle_width, puzzle_height) {
                    antinodes.push((left_x as usize, left_y as usize));
                    left_x -= x_distance;
                    left_y += y_distance;
                } else {
                    break;
                }
            }
        }
        Zero => {
            loop {
                if is_valid_position(right_x, right_y, puzzle_width, puzzle_height) {
                    antinodes.push((right_x as usize, right_y as usize));
                    right_x += x_distance;
                } else {
                    break;
                }
            }
            loop {
                if is_valid_position(left_x, left_y, puzzle_width, puzzle_height) {
                    antinodes.push((left_x as usize, left_y as usize));
                    left_x -= x_distance;
                } else {
                    break;
                }
            }
        }
        Infinite => {
            loop {
                if is_valid_position(right_x, right_y, puzzle_width, puzzle_height) {
                    antinodes.push((right_x as usize, right_y as usize));
                    right_y += y_distance;
                } else {
                    break;
                }
            }
            loop {
                if is_valid_position(left_x, left_y, puzzle_width, puzzle_height) {
                    antinodes.push((left_x as usize, left_y as usize));
                    left_y -= y_distance;
                } else {
                    break;
                }
            }
        }
    }

    antinodes
}

fn get_antinodes(
    antenna_1: (usize, usize),
    antenna_2: (usize, usize),
    puzzle_width: usize,
    puzzle_height: usize,
) -> Vec<(usize, usize)> {
    use SlopeValue::*;

    let (x1, y1) = antenna_1;
    let (x2, y2) = antenna_2;

    let slope_value;

    if x1 > x2 {
        if y1 > y2 {
            slope_value = Positive;
        } else if y1 < y2 {
            slope_value = Negative;
        } else {
            slope_value = Zero;
        }
    } else if x1 < x2 {
        if y2 > y1 {
            slope_value = Positive;
        } else if y2 < y1 {
            slope_value = Negative;
        } else {
            slope_value = Zero;
        }
    } else {
        slope_value = Infinite;
    }

    let x_distance = get_absolute_distant(x1, x2);
    let y_distance = get_absolute_distant(y1, y2);

    get_antinodes_by_slope(
        antenna_1,
        antenna_2,
        slope_value,
        x_distance,
        y_distance,
        puzzle_width,
        puzzle_height,
    )
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
                let antinodes = get_antinodes(
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
