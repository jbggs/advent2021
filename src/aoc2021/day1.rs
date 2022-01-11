// https://adventofcode.com/2021/day/1
use std::fs::read_to_string;

use crate::types::{Error, Solution};

pub fn solver(path: &str) -> Result<(Solution, Solution), Error> {
    let input = read_input(path)?;
    Ok((part1(&input), part2(&input)))
}

fn part1(input: &[i64]) -> Result<i64, Error> {
    let mut depth_counter = 0;
    for i in 1..input.len() {
        let prev = input[i - 1];
        let curr = input[i];
        if prev < curr {
            depth_counter += 1;
        }
    }

    Ok(depth_counter)
}

fn part2(input: &[i64]) -> Result<i64, Error> {
    let mut depth_counter = 0;
    for i in 1..input.len() - 2 {
        let prev = input[i - 1] + input[i] + input[i + 1];
        let curr = input[i] + input[i + 1] + input[i + 2];
        if prev < curr {
            depth_counter += 1;
        }
    }

    Ok(depth_counter)
}

fn read_input(path: &str) -> Result<Vec<i64>, std::io::Error> {
    let input = read_to_string(path)?
        .split("\n")
        .map(str::parse::<i64>)
        .flatten()
        .collect::<Vec<i64>>();
    Ok(input)
}
