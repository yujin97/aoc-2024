use std::fs::read_to_string;

enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

fn search_word(
    puzzle: &Vec<Vec<char>>,
    from: (usize, usize),
    word: &str,
    direction: Direction,
) -> bool {
    use Direction::*;

    let puzzle_width = puzzle[0].len();
    let puzzle_height = puzzle.len();

    let (x, y) = from;
    let target = word.chars().nth(0).unwrap();
    let current = puzzle[y][x];

    let is_target = current == target;

    if word.len() == 1 {
        return is_target;
    }

    let next_char = match direction {
        N => {
            if y > 0 {
                Some((x, y - 1))
            } else {
                None
            }
        }
        NE => {
            if x < puzzle_width - 1 && y > 0 {
                Some((x + 1, y - 1))
            } else {
                None
            }
        }
        E => {
            if x < puzzle_width - 1 {
                Some((x + 1, y))
            } else {
                None
            }
        }
        SE => {
            if x < puzzle_width - 1 && y < puzzle_height - 1 {
                Some((x + 1, y + 1))
            } else {
                None
            }
        }
        S => {
            if y < puzzle_height - 1 {
                Some((x, y + 1))
            } else {
                None
            }
        }
        SW => {
            if x > 0 && y < puzzle_height - 1 {
                Some((x - 1, y + 1))
            } else {
                None
            }
        }
        W => {
            if x > 0 {
                Some((x - 1, y))
            } else {
                None
            }
        }
        NW => {
            if x > 0 && y > 0 {
                Some((x - 1, y - 1))
            } else {
                None
            }
        }
    };

    match next_char {
        Some((next_x, next_y)) => {
            is_target && search_word(puzzle, (next_x, next_y), &word[1..], direction)
        }
        None => false,
    }
}

fn main() {
    use Direction::*;

    let input = read_to_string("src/inputs/day4.txt").expect("Failed to read input.");

    let mut puzzle = Vec::new();

    for line in input.lines() {
        let line: Vec<char> = line.chars().collect();

        puzzle.push(line);
    }

    let mut ans = 0;

    let puzzle_width = puzzle[0].len();
    let puzzle_height = puzzle.len();

    for y in 0..puzzle_height {
        for x in 0..puzzle_width {
            if search_word(&puzzle, (x, y), "XMAS", N) {
                ans += 1;
            }
            if search_word(&puzzle, (x, y), "XMAS", NE) {
                ans += 1;
            }
            if search_word(&puzzle, (x, y), "XMAS", E) {
                ans += 1;
            }
            if search_word(&puzzle, (x, y), "XMAS", SE) {
                ans += 1;
            }
            if search_word(&puzzle, (x, y), "XMAS", S) {
                ans += 1;
            }
            if search_word(&puzzle, (x, y), "XMAS", SW) {
                ans += 1;
            }
            if search_word(&puzzle, (x, y), "XMAS", W) {
                ans += 1;
            }
            if search_word(&puzzle, (x, y), "XMAS", NW) {
                ans += 1;
            }
        }
    }

    println!("We have {} XMAS!!!", ans);
}
