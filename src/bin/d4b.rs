use std::fs::read_to_string;

fn search_x_mas(from: (usize, usize), puzzle: &Vec<Vec<char>>) -> bool {
    let (x, y) = from;
    let puzzle_width = puzzle[0].len();
    let puzzle_height = puzzle.len();

    let current = puzzle[y][x];

    if current != 'A' {
        return false;
    }

    if x < 1 || x == puzzle_width - 1 || y < 1 || y == puzzle_height - 1 {
        return false;
    }

    let nw = puzzle[y - 1][x - 1];
    let ne = puzzle[y - 1][x + 1];
    let se = puzzle[y + 1][x + 1];
    let sw = puzzle[y + 1][x - 1];

    let slash_is_mas = match ne {
        'M' => match sw {
            'S' => true,
            _ => false,
        },
        'S' => match sw {
            'M' => true,
            _ => false,
        },
        _ => false,
    };

    if !slash_is_mas {
        return false;
    }

    let back_slash_is_mas = match nw {
        'M' => match se {
            'S' => true,
            _ => false,
        },
        'S' => match se {
            'M' => true,
            _ => false,
        },
        _ => false,
    };

    return slash_is_mas && back_slash_is_mas;
}

fn main() {
    let input = read_to_string("src/inputs/day4.txt").expect("Failed to load input.");

    let mut puzzle = Vec::new();

    for line in input.lines() {
        let line: Vec<char> = line.chars().collect();

        puzzle.push(line);
    }

    let mut ans = 0;

    for y in 0..puzzle.len() {
        for x in 0..puzzle[0].len() {
            if search_x_mas((x, y), &puzzle) {
                ans += 1;
            }
        }
    }

    println!("X-MAS appeared {} times!!", ans);
}
