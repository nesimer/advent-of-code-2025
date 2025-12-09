use advent_of_code_2025::read_split;
use std::time::Instant;

type Point = (i64, i64);

/// Parse input lines into a list of tile coordinates
fn parse_tiles(input: &[String]) -> Vec<Point> {
    input
        .iter()
        .filter(|s| !s.is_empty())
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

/// Calculate the area of the rectangle defined by two opposite corners
fn rect_area((x1, y1): Point, (x2, y2): Point) -> u64 {
    (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1)
}

/// Find the largest rectangle defined by two red tiles at opposite corners
fn part1(input: &[String]) -> u64 {
    let tiles = parse_tiles(input);
    tiles
        .iter()
        .enumerate()
        .flat_map(|(i, &a)| tiles[i + 1..].iter().map(move |&b| rect_area(a, b)))
        .max()
        .unwrap_or(0)
}

/// Iterates over tiles to build H segments (y, x_min, x_max) and V segments (x, y_min, y_max).
fn get_segments(tiles: &[Point]) -> (Vec<(i64, i64, i64)>, Vec<(i64, i64, i64)>) {
    let (mut h, mut v) = (Vec::new(), Vec::new());
    for i in 0..tiles.len() {
        let (x1, y1) = tiles[i];
        let (x2, y2) = tiles[(i + 1) % tiles.len()];
        if y1 == y2 {
            h.push((y1, x1.min(x2), x1.max(x2)));
        } else {
            v.push((x1, y1.min(y2), y1.max(y2)));
        }
    }
    (h, v)
}

/// Checks that no H or V segment crosses the strict interior of the rectangle.
///
/// A rectangle is valid if it lies entirely within the polygon.
/// Since both corners are red tiles (on the polygon boundary),
/// we only need to verify that no polygon edge cuts through the rectangle's interior.
fn rectangle_fits(a: Point, b: Point, h: &[(i64, i64, i64)], v: &[(i64, i64, i64)]) -> bool {
    let (min_x, max_x) = (a.0.min(b.0), a.0.max(b.0));
    let (min_y, max_y) = (a.1.min(b.1), a.1.max(b.1));
    !v.iter()
        .any(|&(x, y1, y2)| x > min_x && x < max_x && y2 > min_y && y1 < max_y)
        && !h
            .iter()
            .any(|&(y, x1, x2)| y > min_y && y < max_y && x2 > min_x && x1 < max_x)
}

/// Sorts pairs by descending area and returns the first one whose rectangle is valid.
fn part2(input: &[String]) -> u64 {
    let tiles = parse_tiles(input);
    let (h, v) = get_segments(&tiles);

    let mut pairs: Vec<_> = tiles
        .iter()
        .enumerate()
        .flat_map(|(i, &a)| tiles[i + 1..].iter().map(move |&b| (rect_area(a, b), a, b)))
        .collect();

    pairs.sort_unstable_by(|a, b| b.0.cmp(&a.0));

    pairs
        .into_iter()
        .find(|&(_, a, b)| rectangle_fits(a, b, &h, &v))
        .map(|(area, _, _)| area)
        .unwrap_or(0)
}

fn main() {
    let input = read_split(9, "\n");

    println!("Day 09 Results: ⭐⭐");

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
    println!("Part 1: Plus grand rectangle avec 2 tuiles rouges en coins opposés");
    println!(
        "Part 2: Plus grand rectangle entièrement dans le polygone rouge/vert (le polygone est formé en reliant les tuiles rouges)"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part1() {
        let input: Vec<String> = EXAMPLE.lines().map(|s| s.to_string()).collect();
        assert_eq!(part1(&input), 50);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = EXAMPLE.lines().map(|s| s.to_string()).collect();
        assert_eq!(part2(&input), 24);
    }
}
