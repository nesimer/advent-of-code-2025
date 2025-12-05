use advent_of_code_2025::read_split;
use std::time::Instant;

/// Parse the input into ranges and IDs
fn parse_input(input: &[String]) -> (Vec<(u64, u64)>, Vec<u64>) {
    let separator = input.iter().position(|s| s.is_empty()).unwrap();

    let ranges = input[..separator]
        .iter()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();

    let ids = input[separator + 1..]
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    (ranges, ids)
}

/// Merge overlapping and contiguous ranges
/// ex: [(1, 3), (2, 5), (7, 9)] -> [(1, 5), (7, 9)]
fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return ranges;
    }

    ranges.sort_by_key(|r| r.0);
    let mut merged = vec![ranges[0]];

    for (start, end) in ranges.into_iter() {
        let last = merged.last_mut().unwrap();
        if start <= last.1 + 1 {
            last.1 = last.1.max(end);
        } else {
            merged.push((start, end));
        }
    }

    merged
}

/// Check if an ID is within any of the merged ranges
fn is_fresh(id: u64, ranges: &[(u64, u64)]) -> bool {
    let idx = ranges.partition_point(|r| r.0 <= id);
    idx > 0 && id <= ranges[idx - 1].1
}

/// Count IDs that are fresh (within ranges)
fn part1(input: &[String]) -> u64 {
    let (ranges, ids) = parse_input(input);
    let merged = merge_ranges(ranges);
    ids.iter().filter(|&&id| is_fresh(id, &merged)).count() as u64
}

/// Count total unique IDs covered by the merged ranges (ignoring given IDs)
fn part2(input: &[String]) -> u64 {
    let (ranges, _ids) = parse_input(input);
    let merged = merge_ranges(ranges);
    merged.iter().map(|(start, end)| end - start + 1).sum()
}

fn main() {
    let input = read_split(5, "\n");

    println!("Day 05 Results: ⭐⭐");

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
    println!("Part 1: Compter les IDs disponibles qui sont dans un range 'fresh'");
    println!("Part 2: Compter le nombre total d'IDs couverts par les ranges fusionnés");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part1() {
        let input: Vec<String> = EXAMPLE.lines().map(|s| s.trim().to_string()).collect();
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = EXAMPLE.lines().map(|s| s.trim().to_string()).collect();
        assert_eq!(part2(&input), 14);
    }
}
