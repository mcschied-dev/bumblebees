use std::io::{self, BufRead};

fn count_xmas(grid: &Vec<Vec<char>>) -> usize {
    let word = "XMAS".chars().collect::<Vec<_>>();
    let word_len = word.len();
    let mut count = 0;

    let rows = grid.len();
    let cols = grid[0].len();

    // All possible directions: (row_step, col_step)
    let directions = [
        (0, 1),  // right
        (0, -1), // left
        (1, 0),  // down
        (-1, 0), // up
        (1, 1),  // diagonal down-right
        (-1, -1), // diagonal up-left
        (1, -1), // diagonal down-left
        (-1, 1), // diagonal up-right
    ];

    for r in 0..rows {
        for c in 0..cols {
            for &(dr, dc) in &directions {
                let mut found = true;
                for i in 0..word_len {
                    let nr = r as isize + i as isize * dr;
                    let nc = c as isize + i as isize * dc;

                    if nr < 0 || nr >= rows as isize || nc < 0 || nc >= cols as isize {
                        found = false;
                        break;
                    }

                    if grid[nr as usize][nc as usize] != word[i] {
                        found = false;
                        break;
                    }
                }
                if found {
                    count += 1;
                }
            }
        }
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let input: Vec<Vec<char>> = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let result = count_xmas(&input);
    println!("Total occurrences of 'XMAS': {}", result);
}