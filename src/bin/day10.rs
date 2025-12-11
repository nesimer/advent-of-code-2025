use advent_of_code_2025::read_split;
use std::collections::HashSet;
use std::time::Instant;

/// (Target Lights Mask, Target Joltage Vector, Buttons List)
type ParsedData = (u64, Vec<usize>, Vec<Vec<usize>>);

/// Parse input lines into structured data
fn parse_input(input: &[String]) -> Vec<ParsedData> {
    input
        .iter()
        .filter(|s| !s.is_empty())
        .map(|line| {
            let bracket_start = line.find('[').unwrap();
            let bracket_end = line.find(']').unwrap();
            let target_lights = line[bracket_start + 1..bracket_end]
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .fold(0, |acc, (i, _)| acc | (1 << i));

            let rest = &line[bracket_end + 1..];
            let brace_start = rest.find('{').unwrap_or(rest.len());

            let buttons_part = &rest[..brace_start];
            let buttons: Vec<Vec<usize>> = buttons_part
                .split('(')
                .skip(1)
                .filter_map(|s| s.find(')').map(|end| &s[..end]))
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.split(',').filter_map(|n| n.trim().parse().ok()).collect())
                .collect();

            let target_joltage = if brace_start < rest.len() {
                let brace_end = rest.find('}').unwrap();
                rest[brace_start + 1..brace_end]
                    .split(',')
                    .filter_map(|n| n.trim().parse().ok())
                    .collect()
            } else {
                Vec::new()
            };

            (target_lights, target_joltage, buttons)
        })
        .collect()
}

/// Find the minimum number of button presses to match the light pattern (Part 1)
fn part1(input: &[String]) -> u64 {
    let data = parse_input(input);
    data.iter()
        .map(|(target, _, buttons)| {
            // Convert button indices to bitmasks for efficient XOR operations
            let button_masks: Vec<u64> = buttons
                .iter()
                .map(|b| b.iter().fold(0, |acc, &i| acc | (1 << i)))
                .collect();

            let n = button_masks.len();
            // Brute force: check all subsets of buttons by increasing size
            for size in 0..=n {
                for mask in 0u64..(1 << n) {
                    if mask.count_ones() as usize != size {
                        continue;
                    }

                    let mut state = 0;
                    for i in 0..n {
                        if (mask >> i) & 1 != 0 {
                            state ^= button_masks[i];
                        }
                    }
                    if state == *target {
                        return size as u64;
                    }
                }
            }
            0
        })
        .sum()
}

/// Computes the GCD of two numbers
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a.abs().max(1)
    } else {
        gcd(b, a % b)
    }
}

/// Computes the GCD of all non-zero elements in a vector
fn gcd_vec(row: &[i64]) -> i64 {
    row.iter().copied().filter(|&x| x != 0).fold(0, gcd).max(1)
}

/// Solves using linear algebra.
///
/// Treat the problem as a system of equations where it needs to find the right number of presses for each button.
/// To solve this, use [Gaussian Elimination](https://en.wikipedia.org/wiki/Gaussian_elimination) to simplify the system.
///
/// Since there can be multiple valid solutions, then search through them to find the one that requires
/// the minimum total number of button presses.
fn part2(input: &[String]) -> u64 {
    let data = parse_input(input);

    data.iter()
        .map(|(_, targets, buttons)| {
            if buttons.is_empty() {
                return 0;
            }

            let rows = targets.len();
            let cols = buttons.len();

            let mut matrix: Vec<Vec<i64>> = (0..rows)
                .map(|r| {
                    let mut row = Vec::with_capacity(cols + 1);
                    for b in buttons {
                        row.push(if b.contains(&r) { 1 } else { 0 });
                    }
                    row.push(targets[r] as i64);
                    row
                })
                .collect();

            let mut pivots = Vec::new();
            let mut current_row = 0;

            for col in 0..cols {
                if current_row >= rows {
                    break;
                }

                if let Some(pivot_row) = (current_row..rows).find(|&r| matrix[r][col] != 0) {
                    matrix.swap(current_row, pivot_row);
                    pivots.push((current_row, col));
                    let pivot_val = matrix[current_row][col];

                    let rows_to_elim: Vec<usize> = (0..rows)
                        .filter(|&r| r != current_row && matrix[r][col] != 0)
                        .collect();

                    for r in rows_to_elim {
                        let factor = matrix[r][col];
                        for c in 0..=cols {
                            matrix[r][c] =
                                matrix[r][c] * pivot_val - matrix[current_row][c] * factor;
                        }
                        let g = gcd_vec(&matrix[r]);
                        if g > 1 {
                            for x in matrix[r].iter_mut() {
                                *x /= g;
                            }
                        }
                    }
                    current_row += 1;
                }
            }

            for row in &matrix[pivots.len()..] {
                if row[cols] != 0 {
                    return 0;
                }
            }

            let pivot_cols: HashSet<usize> = pivots.iter().map(|&(_, c)| c).collect();
            let free_cols: Vec<usize> = (0..cols).filter(|c| !pivot_cols.contains(c)).collect();

            let bounds: Vec<i64> = free_cols
                .iter()
                .map(|&col| {
                    buttons[col]
                        .iter()
                        .map(|&row_idx| targets[row_idx] as i64)
                        .min()
                        .unwrap_or(0)
                })
                .collect();

            let mut min_total = u64::MAX;
            let mut current_free = vec![0; free_cols.len()];

            fn recurse(
                idx: usize,
                free_cols: &[usize],
                bounds: &[i64],
                current_free: &mut [i64],
                matrix: &[Vec<i64>],
                pivots: &[(usize, usize)],
                min_total: &mut u64,
            ) {
                if idx == free_cols.len() {
                    let cols = matrix[0].len() - 1;
                    let mut solution = vec![0; cols];

                    for (i, &col) in free_cols.iter().enumerate() {
                        solution[col] = current_free[i];
                    }

                    for &(row, col) in pivots.iter().rev() {
                        let pivot = matrix[row][col];
                        let mut sum = 0;
                        for c in 0..cols {
                            if c != col {
                                sum += matrix[row][c] * solution[c];
                            }
                        }
                        let rhs = matrix[row][cols] - sum;

                        if pivot == 0 || rhs % pivot != 0 {
                            return;
                        }
                        solution[col] = rhs / pivot;
                    }

                    if solution.iter().all(|&x| x >= 0) {
                        *min_total = (*min_total).min(solution.iter().map(|&x| x as u64).sum());
                    }
                    return;
                }

                for val in 0..=bounds[idx] {
                    current_free[idx] = val;
                    recurse(
                        idx + 1,
                        free_cols,
                        bounds,
                        current_free,
                        matrix,
                        pivots,
                        min_total,
                    );
                }
            }

            recurse(
                0,
                &free_cols,
                &bounds,
                &mut current_free,
                &matrix,
                &pivots,
                &mut min_total,
            );

            if min_total == u64::MAX { 0 } else { min_total }
        })
        .sum()
}

fn main() {
    let input = read_split(10, "\n");

    println!("Day 10 Results: ⭐⭐");

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
    println!(
        "Part 1: Minimum de pressions de boutons pour activer toutes les lumières (Brute-force)"
    );
    println!("Part 2: Minimum de pressions pour atteindre le voltage cible (Élimination de Gauss)");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_part1() {
        let input: Vec<String> = EXAMPLE.lines().map(|s| s.to_string()).collect();
        assert_eq!(part1(&input), 7);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = EXAMPLE.lines().map(|s| s.to_string()).collect();
        assert_eq!(part2(&input), 33);
    }
}
