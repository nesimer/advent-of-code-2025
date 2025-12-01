use advent_of_code_2025::read_split;
use std::time::Instant;

/// Checks if a number has even-length digits split in two equal parts.
fn has_equal_parts(n: u64) -> bool {
    let digits = n.to_string();
    let len = digits.len();
    
    if len % 2 != 0 {
        return false;
    }
    
    let mid = len / 2;
    digits[..mid] == digits[mid..]
}

/// Checks if a number can be formed by repeating a pattern at least twice.
fn has_repeating_pattern(n: u64) -> bool {
    let digits = n.to_string();
    let len = digits.len();
    
    for pattern_size in 1..=len / 2 {
        if len % pattern_size != 0 {
            continue;
        }
        
        let repeat_count = len / pattern_size;
        if repeat_count < 2 {
            continue;
        }
        
        let pattern = &digits[..pattern_size];
        if pattern.repeat(repeat_count) == digits {
            return true;
        }
    }
    
    false
}

/// Solves the problem by applying the validator function over the given ranges.
fn solve<F>(input: &[String], validator: F) -> u64
where
    F: Fn(u64) -> bool,
{
    input
        .iter()
        .filter_map(|range| {
            let range = range.trim();
            if range.is_empty() {
                return None;
            }
            
            let (start, end) = range.split_once('-')?;
            let start: u64 = start.parse().ok()?;
            let end: u64 = end.parse().ok()?;
            
            Some((start..=end).filter(|&n| validator(n)).sum::<u64>())
        })
        .sum()
}

fn part1(input: &[String]) -> u64 {
    solve(input, has_equal_parts)
}

fn part2(input: &[String]) -> u64 {
    solve(input, has_repeating_pattern)
}

fn main() {
    let input = read_split(2, ",");
    
    println!("Day 02 Results: ⭐⭐");
    
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

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        let input: Vec<String> = EXAMPLE.split(',').map(|s| s.trim().to_string()).collect();
        assert_eq!(part1(&input), 1227775554);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = EXAMPLE.split(',').map(|s| s.trim().to_string()).collect();
        assert_eq!(part2(&input), 4174379265);
    }
}
