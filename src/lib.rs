//! Utility functions for Advent of Code 2025

use std::fs;

/// Reads the input file for a given day.
fn read_input(day: u8) -> String {
    let path = format!("inputs/day{:02}.txt", day);
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read {}", path))
}

/// Reads the input file and splits by a separator.
pub fn read_split(day: u8, separator: &str) -> Vec<String> {
    read_input(day)
        .split(separator)
        .map(String::from)
        .collect()
}
