use std::{fs, io};

fn main() -> io::Result<()> {
    let path = "src/bin/day 1/input.txt";

    let input = fs::read_to_string(path)?;

    let mut total_calories: Vec<i32> = Vec::new();

    for line in input.split("\r\n\r\n") {
        let calories: Vec<i32> = line.lines().map(|s| s.trim().parse::<i32>().unwrap()).collect();
        let calo_sum: i32 = calories.iter().sum();
        total_calories.push(calo_sum);
    }

    total_calories.sort();
    let total_three: i32 = total_calories.iter().rev().take(3).sum();

    println!("{}", total_three);

    Ok(())
}