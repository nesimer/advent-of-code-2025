use advent_of_code_2025::read_split;
use std::time::Instant;

/// Find the maximum joltage that can be formed with the given number of digits
fn find_max_joltage(line: &str, num_digits: usize) -> u64 {
    let mut best: Vec<u64> = vec![0; num_digits];

    for c in line.chars() {
        let Some(digit) = c.to_digit(10) else {
            continue;
        };
        let d = digit as u64;

        for i in (1..num_digits).rev() {
            let new_val = best[i - 1] * 10 + d;
            best[i] = best[i].max(new_val);
        }

        best[0] = best[0].max(d);
    }

    best[num_digits - 1] as u64
}

/// Solve the problem for the given number of digits
fn solve(num_digits: usize, input: &[String]) -> u64 {
    let mut total = 0;
    for line in input {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let joltage = find_max_joltage(line, num_digits);
        total += joltage;
    }
    total
}

/// Solve the problem for the 2 of digits
fn part1(input: &[String]) -> u64 {
    solve(2, input)
}

/// Solve the problem for the 12 of digits
fn part2(input: &[String]) -> u64 {
    solve(12, input)
}

fn main() {
    let input = read_split(3, "\n");

    println!("Day 03 Results: ⭐⭐");

    let start = Instant::now();
    let result1 = part1(&input);
    let duration1 = start.elapsed();
    println!("Part 1: {} (took {:?})", result1, duration1);

    let start = Instant::now();
    let result2 = part2(&input);
    let duration2 = start.elapsed();
    println!("Part 2: {} (took {:?})", result2, duration2);

    println!("Total: {:?}", duration1 + duration2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1() {
        let input: Vec<String> = EXAMPLE.lines().map(|s| s.trim().to_string()).collect();
        assert_eq!(part1(&input), 357);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = EXAMPLE.lines().map(|s| s.trim().to_string()).collect();
        assert_eq!(part2(&input), 3121910778619);
    }
}
