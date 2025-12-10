use advent_of_code_2025::read_split;
use std::time::Instant;

/// Parse a machine line into (target_state, buttons) where states are bitmasks
fn parse_machine(line: &str) -> (u64, Vec<u64>) {
    let bracket_start = line.find('[').unwrap();
    let bracket_end = line.find(']').unwrap();
    let diagram = &line[bracket_start + 1..bracket_end];

    // [.#..]  -> 0b0100 -> 4
    let target: u64 = diagram
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '#')
        .fold(0, |acc, (i, _)| acc | (1 << i));

    let mut buttons = Vec::new();
    let rest = &line[bracket_end + 1..];

    for part in rest.split('(').skip(1) {
        if let Some(end) = part.find(')') {
            let content = &part[..end];
            if content.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                let mut mask = 0u64;
                for s in content.split(',') {
                    if let Ok(i) = s.trim().parse::<u64>() {
                        mask |= 1 << i;
                    }
                }
                buttons.push(mask);
            }
        }
    }

    (target, buttons)
}

/// Enumerates subsets by size (0, 1, 2, ...) and returns as soon as one reaches target
fn min_presses(target: u64, buttons: &[u64]) -> u64 {
    let n = buttons.len();
    // Test subsets by increasing size
    for num_presses in 0..=n {
        for mask in 0u64..(1 << n) {
            // Skip if this subset doesn't have exactly num_presses buttons
            if mask.count_ones() as usize != num_presses {
                continue;
            }
            // Compute final state by XORing all pressed buttons' effects
            let mut state = 0u64;
            for i in 0..n {
                if mask & (1 << i) != 0 {
                    state ^= buttons[i];
                }
            }
            // First solution found is minimal
            if state == target {
                return num_presses as u64;
            }
        }
    }
    0
}

/// Sum of minimum button presses for all machines
fn part1(input: &[String]) -> u64 {
    input
        .iter()
        .filter(|s| !s.is_empty())
        .map(|line| {
            let (target, buttons) = parse_machine(line);
            min_presses(target, &buttons)
        })
        .sum()
}

fn part2(input: &[String]) -> u64 {
    0
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
    println!("Part 1: Minimum de pressions de boutons pour activer toutes les machines");
    println!("Part 2: TODO");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

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
