use advent_of_code_2025::read_split;
use std::time::Instant;

/// Directions for neighbor cells
const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

/// Parse the input grid into a 2D vector of bytes
fn parse_grid(input: &[String]) -> Vec<Vec<u8>> {
    input.iter().map(|l| l.bytes().collect()).collect()
}

/// Build a neighbor count grid
fn build_neighbor_counts(grid: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let height = grid.len();
    let width = grid[0].len();
    let mut counts = vec![vec![0u8; width]; height];

    for row in 0..height {
        for col in 0..width {
            if grid[row][col] != b'@' {
                continue;
            }
            for (dr, dc) in DIRECTIONS {
                let r = row as i32 + dr;
                let c = col as i32 + dc;
                if r >= 0 && r < height as i32 && c >= 0 && c < width as i32 {
                    counts[r as usize][c as usize] += 1;
                }
            }
        }
    }
    counts
}

/// Check if a cell is accessible based on the grid and neighbor counts
fn is_accessible(grid: &[Vec<u8>], counts: &[Vec<u8>], row: usize, col: usize) -> bool {
    grid[row][col] == b'@' && counts[row][col] < 4
}

/// Find all accessible cells in the grid
fn find_accessible(grid: &[Vec<u8>], counts: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if is_accessible(grid, counts, row, col) {
                result.push((row, col));
            }
        }
    }
    result
}

/// Count accessible cells in the grid
fn part1(input: &[String]) -> u64 {
    let grid = parse_grid(input);
    let counts = build_neighbor_counts(&grid);
    find_accessible(&grid, &counts).len() as u64
}

/// Remove accessible cells iteratively and count them while updating neighbor counts
fn part2(input: &[String]) -> u64 {
    let mut grid = parse_grid(input);
    let height = grid.len();
    let width = grid[0].len();
    let mut counts = build_neighbor_counts(&grid);

    let mut stack = find_accessible(&grid, &counts);
    let mut total_removed = 0;

    while let Some((row, col)) = stack.pop() {
        if grid[row][col] != b'@' {
            continue;
        }

        grid[row][col] = b'.';
        total_removed += 1;

        for (dr, dc) in DIRECTIONS {
            let r = row as i32 + dr;
            let c = col as i32 + dc;
            if r < 0 || r >= height as i32 || c < 0 || c >= width as i32 {
                continue;
            }
            let (nr, nc) = (r as usize, c as usize);
            counts[nr][nc] -= 1;

            if is_accessible(&grid, &counts, nr, nc) {
                stack.push((nr, nc));
            }
        }
    }

    total_removed
}

fn main() {
    let input = read_split(4, "\n");

    println!("Day 04 Results: ⭐⭐");

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
    println!("Part 1: Compter les rouleaux @ accessibles (< 4 voisins @)");
    println!(
        "Part 2: Retirer les rouleaux accessibles en boucle jusqu'à stabilisation (propagation BFS)"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1() {
        let input: Vec<String> = EXAMPLE.lines().map(|s| s.trim().to_string()).collect();
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = EXAMPLE.lines().map(|s| s.trim().to_string()).collect();
        assert_eq!(part2(&input), 43);
    }
}
