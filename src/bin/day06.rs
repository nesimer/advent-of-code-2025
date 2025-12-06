use advent_of_code_2025::read_split;
use std::time::Instant;

/// Build the grid from input lines
fn build_grid(input: &[String]) -> (Vec<Vec<char>>, usize, usize) {
    let height = input.len();
    let width = input.iter().map(|s| s.len()).max().unwrap_or(0);
    let grid = input
        .iter()
        .map(|line| {
            let mut chars: Vec<char> = line.chars().collect();
            chars.resize(width, ' ');
            chars
        })
        .collect();
    (grid, height, width)
}

/// Find problems (operations with their column ranges) in the grid
fn find_problems(grid: &[Vec<char>], height: usize, width: usize) -> Vec<(char, usize, usize)> {
    // Check if a column is a separator (all spaces)
    let is_separator = |col: usize| (0..height).all(|row| grid[row][col] == ' ');

    let mut problems = Vec::new();
    let mut col = 0;

    while col < width {
        if is_separator(col) {
            col += 1;
            continue;
        }

        let start_col = col;
        while col < width && !is_separator(col) {
            col += 1;
        }

        let op = (start_col..col)
            .find_map(|c| match grid[height - 1][c] {
                '+' | '*' => Some(grid[height - 1][c]),
                _ => None,
            })
            .unwrap_or('+');

        problems.push((op, start_col, col));
    }

    problems
}

/// Apply the operation to the list of numbers
fn apply_op(op: char, nums: Vec<u64>) -> u64 {
    match op {
        '+' => nums.iter().sum(),
        '*' => nums.iter().product(),
        _ => 0,
    }
}

fn solve<F>(input: &[String], extract_nums: F) -> u64
where
    F: Fn(&[Vec<char>], usize, usize, usize) -> Vec<u64>,
{
    let (grid, height, width) = build_grid(input);
    find_problems(&grid, height, width)
        .iter()
        .map(|&(op, start, end)| apply_op(op, extract_nums(&grid, height, start, end)))
        .sum()
}

fn part1(input: &[String]) -> u64 {
    solve(input, |grid, height, start, end| {
        (0..height - 1) // iterate over rows
            .map(|row| {
                (start..end) // iterate over columns
                    .filter_map(|c| grid[row][c].to_digit(10).map(|d| d as u64))
                    .fold(0, |acc, d| acc * 10 + d)
            })
            .collect()
    })
}

fn part2(input: &[String]) -> u64 {
    solve(input, |grid, height, start, end| {
        (start..end) // iterate over columns
            .rev() // right to left
            .map(|c| {
                (0..height - 1) // iterate over rows
                    .filter_map(|row| grid[row][c].to_digit(10).map(|d| d as u64))
                    .fold(0, |acc, d| acc * 10 + d)
            })
            .collect()
    })
}

fn main() {
    let input = read_split(6, "\n");

    println!("Day 06 Results: ⭐⭐");

    let start = Instant::now();
    let result1 = part1(&input);
    let duration1 = start.elapsed();
    println!("Part 1: {} (took {:?})", result1, duration1);

    let start = Instant::now();
    let result2 = part2(&input);
    let duration2 = start.elapsed();
    println!("Part 2: {} (took {:?})", result2, duration2);

    println!("Total: {:?}", duration1 + duration2);

    println!("\n--- Résumé des solutions ---");
    println!("Part 1: Lecture horizontale des nombres, puis + ou * selon l'opérateur");
    println!("Part 2: Lecture verticale des colonnes de droite à gauche");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1() {
        let input: Vec<String> = EXAMPLE.lines().map(|s| s.to_string()).collect();
        assert_eq!(part1(&input), 4277556);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = EXAMPLE.lines().map(|s| s.to_string()).collect();
        assert_eq!(part2(&input), 3263827);
    }
}
