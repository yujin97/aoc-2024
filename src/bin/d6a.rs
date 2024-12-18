use std::{collections::HashSet, fs::read_to_string};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    use Direction::*;

    let input = read_to_string("src/inputs/day6.txt").expect("Failed to load input.");

    let mut map = Vec::new();

    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }

        map.push(row);
    }

    let map_width: isize = map[0].len().try_into().unwrap();
    let map_height: isize = map.len().try_into().unwrap();

    let mut current = (-1, -1);
    let mut direction = Up;

    let mut visited = HashSet::new();

    for (y, row) in map.iter().enumerate() {
        for x in 0..row.len() {
            if row[x] == '^' {
                let x: isize = x.try_into().unwrap();
                let y: isize = y.try_into().unwrap();
                current = (x, y);
            }
        }
    }

    loop {
        let (x, y) = current;
        let next_position = match direction {
            Up => (x, y - 1),
            Down => (x, y + 1),
            Left => (x - 1, y),
            Right => (x + 1, y),
        };

        let (next_x, next_y) = next_position;

        if next_x < 0 || next_x >= map_width || next_y < 0 || next_y >= map_height {
            break;
        }

        let next_x: usize = next_x.try_into().unwrap();
        let next_y: usize = next_y.try_into().unwrap();

        if map[next_y][next_x] == '#' {
            direction = match direction {
                Up => Right,
                Right => Down,
                Down => Left,
                Left => Up,
            };

            continue;
        } else {
            visited.insert((next_x, next_y));

            let next_x: isize = next_x.try_into().unwrap();
            let next_y: isize = next_y.try_into().unwrap();
            current = (next_x, next_y);
        }
    }

    println!("{} distinct positions are visited!", visited.len());
}
