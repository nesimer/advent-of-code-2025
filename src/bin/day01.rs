use advent_of_code_2025::read_lines;
use std::time::Instant;

const DIAL_SIZE: i32 = 100;
const START: i32 = 50;

/// Counts how many times the dial ends exactly on zero after each move.
fn part1(input: &[String]) -> u64 {
    let mut position = START;
    let mut zero_count = 0;

    for line in input {
        let (direction, distance_str) = line.split_at(1);
        let distance: i32 = distance_str.parse().unwrap();
        let steps = distance.rem_euclid(DIAL_SIZE);

        position = match direction {
            "L" => (position - steps).rem_euclid(DIAL_SIZE),
            "R" => (position + steps).rem_euclid(DIAL_SIZE),
            _ => panic!("Invalid direction: {}", direction),
        };

        if position == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

/// Counts how many times the dial crosses zero during all movements.
fn part2(input: &[String]) -> u64 {
    let mut position = START;
    let mut zero_count = 0u64;

    for line in input {
        let (direction, distance_str) = line.split_at(1);
        let distance: i32 = distance_str.parse().unwrap();

        let distance_to_zero = match direction {
            "L" => if position == 0 { DIAL_SIZE } else { position },
            "R" => if position == 0 { DIAL_SIZE } else { DIAL_SIZE - position },
            _ => panic!("Invalid direction: {}", direction),
        };

        if distance >= distance_to_zero {
            zero_count += (1 + (distance - distance_to_zero) / DIAL_SIZE) as u64;
        }

        let steps = distance.rem_euclid(DIAL_SIZE);
        position = match direction {
            "L" => (position - steps).rem_euclid(DIAL_SIZE),
            "R" => (position + steps).rem_euclid(DIAL_SIZE),
            _ => panic!("Invalid direction: {}", direction),
        };
    }

    zero_count
}

fn main() {
    let input = read_lines(1);
    
    println!("Day 01 Results: ⭐⭐");
    
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

    const EXAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1() {
        let input: Vec<String> = EXAMPLE.lines().map(String::from).collect();
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = EXAMPLE.lines().map(String::from).collect();
        assert_eq!(part2(&input), 6);
    }
}
