//! Utility functions for Advent of Code 2025

use std::fs;

/// Reads the input file for a given day.
fn read_input(day: u8) -> String {
    let path = format!("inputs/day{:02}.txt", day);
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read {}", path))
}

/// Reads the input file and returns lines as a vector of strings.
pub fn read_lines(day: u8) -> Vec<String> {
    read_input(day).lines().map(String::from).collect()
}
