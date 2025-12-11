use advent_of_code_2025::read_split;
use std::collections::HashMap;
use std::time::Instant;

/// Parse input into an adjacency list
fn parse_graph(input: &[String]) -> HashMap<&str, Vec<&str>> {
    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.iter().filter(|l| !l.is_empty()) {
        if let Some((from, rest)) = line.split_once(':') {
            for to in rest.split_whitespace() {
                adj.entry(from.trim()).or_default().push(to);
            }
        }
    }
    adj
}

/// Recursively count paths from node to "out" with memoization
fn count_paths<'a>(
    node: &'a str,
    adj: &HashMap<&'a str, Vec<&'a str>>,
    memo: &mut HashMap<&'a str, u64>,
) -> u64 {
    if node == "out" {
        return 1;
    }
    if let Some(&v) = memo.get(node) {
        return v;
    }

    let total = match adj.get(node) {
        Some(neighbors) => neighbors.iter().map(|n| count_paths(n, adj, memo)).sum(),
        None => 0,
    };

    memo.insert(node, total);
    total
}

/// Count paths with constraints
fn count_paths_constrained<'a>(
    node: &'a str,
    adj: &HashMap<&'a str, Vec<&'a str>>,
    memo: &mut HashMap<(&'a str, (bool, bool)), u64>,
    constraints: (&str, &str),
    visited: (bool, bool),
) -> u64 {
    let visited = (
        visited.0 || node == constraints.0,
        visited.1 || node == constraints.1,
    );

    if node == "out" {
        return if visited.0 && visited.1 { 1 } else { 0 };
    }

    let key = (node, visited);
    if let Some(&v) = memo.get(&key) {
        return v;
    }

    let total = match adj.get(node) {
        Some(neighbors) => neighbors
            .iter()
            .map(|n| count_paths_constrained(n, adj, memo, constraints, visited))
            .sum(),
        None => 0,
    };

    memo.insert(key, total);
    total
}

/// Count distinct paths from "you" to "out" in a directed graph.
fn part1(input: &[String]) -> u64 {
    let adj = parse_graph(input);
    count_paths("you", &adj, &mut HashMap::new())
}

/// Count paths from "svr" to "out" that visit both "dac" and "fft"
fn part2(input: &[String]) -> u64 {
    let adj = parse_graph(input);
    count_paths_constrained(
        "svr",
        &adj,
        &mut HashMap::new(),
        ("dac", "fft"),
        (false, false),
    )
}

fn main() {
    let input = read_split(11, "\n");

    println!("Day 11 Results: ⭐⭐");

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
    println!("Part 1: Compter les chemins distincts de 'you' vers 'out' dans le graphe");
    println!("Part 2: Compter les chemins de 'svr' vers 'out' passant par 'dac' et 'fft'");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const EXAMPLE2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_part1() {
        let input: Vec<String> = EXAMPLE.lines().map(|s| s.to_string()).collect();
        assert_eq!(part1(&input), 5);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = EXAMPLE2.lines().map(|s| s.to_string()).collect();
        assert_eq!(part2(&input), 2);
    }
}
