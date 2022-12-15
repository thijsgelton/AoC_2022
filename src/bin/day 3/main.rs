#![feature(iter_array_chunks)]

use std::{fs, io};
use std::collections::HashMap;
use std::path::Iter;

fn main() -> io::Result<()> {
    let path = "src/bin/day 3/input.txt";

    let input = fs::read_to_string(path)?;

    let total_score = compute_score_part_2(input);

    println!("{}", total_score.to_string());
    Ok(())
}

fn compute_score_part_1(input: String) -> usize {
    let mut total_score = 0;
    let score_card: &str = "0abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    for line in input.lines() {
        let (first, second) = line.split_at(line.len() / 2);
        for char in first.chars() {
            if second.contains(char) {
                total_score += score_card.find(char).unwrap();
                break;
            }
        }
    }
    total_score
}


fn compute_score_part_2(input: String) -> usize {
    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

    let total_score = input
        .lines()
        .array_chunks::<3>()
        .map(|[a, b, c]| {
            let common_char = a
                .chars()
                .find(|a_char| {
                    b.contains(*a_char)
                        && c.contains(*a_char)
                })
                .unwrap();
            letter_scores.get(&common_char).unwrap()
        })
        .sum::<usize>();

    total_score
}