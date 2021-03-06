// https://adventofcode.com/2021/day/3
use std::{fs::read_to_string, ops::Index};

use crate::types::Solution;
use anyhow::Result;

pub fn solve(path: &str) -> Result<(Solution, Solution)> {
    let report = read_input(path)?;
    let soln1 = part1(&report);
    let soln2 = part2(&report);
    Ok((soln1, soln2))
}

pub fn part1(report: &Vec<Vec<bool>>) -> Result<i64> {
    if let Some(row) = report.first() {
        let common_bits: Vec<bool> = (0..row.len())
            .map(|i| get_column(report.iter(), i))
            .map(|col| most_common_bit(&col))
            .collect();

        let uncommon_bits = common_bits.iter().map(|b| !b).collect::<Vec<bool>>();

        let gamma_rate = greek_rate(&common_bits);
        let epsilon_rate = greek_rate(&uncommon_bits);

        Ok(gamma_rate * epsilon_rate)
    } else {
        Ok(0)
    }
}

pub fn part2(report: &Vec<Vec<bool>>) -> Result<i64> {
    let o2_rating = chem_rating(report, &|col| most_common_bit(col));
    let co2_rating = chem_rating(report, &|col| !most_common_bit(col));

    return Ok(o2_rating * co2_rating);
}

pub fn get_column<'a, O, I, E>(rows: O, column_idx: usize) -> Vec<E>
where
    O: Iterator<Item = &'a I> + 'a,            // Outer Collection (rows)
    I: Index<usize, Output = E> + 'a + ?Sized, // Inner Collection (row)
    E: Copy,                                   // Element type
{
    rows.map(|b| b[column_idx]).collect()
}

fn most_common_bit(bits: &[bool]) -> bool {
    let ham_weight: usize = bits.iter().map(|&b| usize::from(b)).sum();
    return ham_weight >= bits.len() / 2;
}

fn greek_rate(row: &[bool]) -> i64 {
    let bitstring = row
        .iter()
        .map(|&b| i64::from(b).to_string())
        .collect::<Vec<String>>()
        .join("");

    return i64::from_str_radix(bitstring.as_str(), 2).unwrap();
}

fn chem_rating<'a>(report: &Vec<Vec<bool>>, calc_mode: &dyn Fn(&[bool]) -> bool) -> i64 {
    let mut rows: Vec<&[bool]> = report.iter().map(Vec::as_slice).collect();
    if let Some(row) = rows.first() {
        for i in 0..row.len() {
            let col = get_column(rows.clone().into_iter(), i);
            let mode = calc_mode(&col);
            rows = rows.into_iter().filter(|row| row[i] == mode).collect();
            if rows.len() == 1 {
                break;
            }
        }
        greek_rate(&rows[0])
    } else {
        0
    }
}

fn read_input(path: &str) -> Result<Vec<Vec<bool>>> {
    let diagnostic_report: Vec<Vec<bool>> = read_to_string(path)?
        .split("\n")
        .filter(|&s| s != "")
        .map(bitstring_to_bools)
        .collect();
    Ok(diagnostic_report)
}

fn bitstring_to_bools(string: &str) -> Vec<bool> {
    string.chars().map(|c| c == '1').collect()
}
