use advent_of_code_2025::read_split;
use std::time::Instant;

/// Simulate the propagation of beams through the grid
/// Returns (number of splitters hit, total distinct timelines at the bottom)
/// The grid contains:
/// - 'S' : starting point of the beam
/// - '^' : splitter that divides the beam into two (left and right)
/// - '.' : empty space where the beam continues straight down
/// - any other character : blocks the beam
fn propagate_beams(input: &[String]) -> (u64, u64) {
    let width = input[0].len();
    let start_col = input[0].find('S').unwrap();

    let mut beams = vec![0u64; width];
    beams[start_col] = 1;
    let mut split_count = 0u64;

    for line in input.iter().skip(1) {
        let mut next_beams = vec![0u64; width];

        for (col, &timelines) in beams.iter().enumerate() {
            if timelines == 0 {
                continue;
            }

            match line.as_bytes()[col] {
                b'^' => {
                    split_count += 1;
                    if col > 0 {
                        next_beams[col - 1] += timelines;
                    }
                    if col + 1 < width {
                        next_beams[col + 1] += timelines;
                    }
                }
                b'.' | b'S' => {
                    next_beams[col] += timelines;
                }
                _ => {}
            }
        }

        beams = next_beams;
    }

    (split_count, beams.iter().sum())
}

fn main() {
    let input = read_split(7, "\n");

    println!("Day 07 Results: ⭐⭐");

    let start = Instant::now();
    let (part1_result, part2_result) = propagate_beams(&input);
    let duration = start.elapsed();
    println!("Part 1: {} (took {:?})", part1_result, duration);

    println!("Part 2: {} (took {:?})", part2_result, duration);

    println!("Total: {:?}", duration);

    println!("\n--- Résumé des solutions ---");
    println!("Part 1: Nombre de splitters ^ touchés par les faisceaux");
    println!("Part 2: Nombre de chemins distincts (timelines) en fin de grille");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1() {
        let input: Vec<String> = EXAMPLE.lines().map(|s| s.to_string()).collect();
        assert_eq!(propagate_beams(&input).0, 21);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = EXAMPLE.lines().map(|s| s.to_string()).collect();
        assert_eq!(propagate_beams(&input).1, 40);
    }
}
