use advent_of_code_2025::read_split;
use std::time::Instant;

/// Disjoint Set Union (Union-Find) structure for managing connected components
fn find(parent: &mut [usize], x: usize) -> usize {
    if parent[x] != x {
        parent[x] = find(parent, parent[x]);
    }
    parent[x]
}

/// Union two components
/// Connects the components containing x and y
/// Uses union by size optimization
fn union(parent: &mut [usize], size: &mut [usize], x: usize, y: usize) {
    let (rx, ry) = (find(parent, x), find(parent, y));
    if rx != ry {
        let (small, big) = if size[rx] < size[ry] {
            (rx, ry)
        } else {
            (ry, rx)
        };
        parent[small] = big;
        size[big] += size[small];
    }
}

/// Parses a list of strings into a vector of 3D points represented as tuples of u64
fn parse_points(input: &[String]) -> Vec<(u64, u64, u64)> {
    input
        .iter()
        .filter(|s| !s.is_empty())
        .map(|line| {
            let mut parts = line.split(',');
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

/// Calculates the squared Euclidean distance between two 3D points
fn squared_distance(a: (u64, u64, u64), b: (u64, u64, u64)) -> u64 {
    let (dx, dy, dz) = (a.0.abs_diff(b.0), a.1.abs_diff(b.1), a.2.abs_diff(b.2));
    dx * dx + dy * dy + dz * dz
}

/// Computes all pairwise squared distances between points and returns them sorted
fn compute_sorted_distances(points: &[(u64, u64, u64)]) -> Vec<(u64, usize, usize)> {
    let n = points.len();
    let mut distances: Vec<(u64, usize, usize)> = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            distances.push((squared_distance(points[i], points[j]), i, j));
        }
    }
    distances.sort_unstable_by_key(|d| d.0);
    distances
}

fn part1(input: &[String]) -> u64 {
    part1_with_n_connections(input, 1000)
}

fn part1_with_n_connections(input: &[String], num_connections: usize) -> u64 {
    let points = parse_points(input);
    let n = points.len();
    let mut distances = compute_sorted_distances(&points);

    // Keep only the smallest distances
    distances.truncate(num_connections);

    // Union-Find: each point starts as its own group
    let mut parent: Vec<usize> = (0..n).collect();
    let mut size: Vec<usize> = vec![1; n];

    // Connect each pair, merging their groups
    for &(_, i, j) in &distances {
        union(&mut parent, &mut size, i, j);
    }

    // Collect group sizes (only roots hold the real size)
    let mut sizes: Vec<usize> = (0..n)
        .filter(|&i| find(&mut parent, i) == i)
        .map(|i| size[i])
        .collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    sizes.iter().take(3).map(|&s| s as u64).product()
}

fn part2(input: &[String]) -> u64 {
    let points = parse_points(input);
    let n = points.len();
    let distances = compute_sorted_distances(&points);

    // Union-Find: each point starts as its own group
    let mut parent: Vec<usize> = (0..n).collect();
    let mut size: Vec<usize> = vec![1; n];

    // Connect pairs until all points are in one circuit
    for &(_, i, j) in &distances {
        if find(&mut parent, i) != find(&mut parent, j) {
            union(&mut parent, &mut size, i, j);
            if size[find(&mut parent, i)] == n {
                return points[i].0 * points[j].0;
            }
        }
    }
    0
}

fn main() {
    let input = read_split(8, "\n");

    println!("Day 08 Results: ⭐⭐");

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
        "Part 1: Connecter les 1000 paires les plus proches, produit des 3 plus grands circuits"
    );
    println!("Part 2: Connecter jusqu'à un seul circuit, produit des X des 2 dernières boîtes");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part1() {
        let input: Vec<String> = EXAMPLE.lines().map(|s| s.to_string()).collect();
        assert_eq!(part1_with_n_connections(&input, 10), 40);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = EXAMPLE.lines().map(|s| s.to_string()).collect();
        assert_eq!(part2(&input), 25272);
    }
}
