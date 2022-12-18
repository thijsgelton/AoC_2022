use std::{fs, io};

fn main() -> io::Result<()> {
    let path = "src/bin/day 4/input.txt";

    let input = fs::read_to_string(path)?;

    let total_score = part_two(input);
    println!("{}", total_score);

    Ok(())
}

fn part_one(input: String) -> i32 {
    let mut total_score: i32 = 0;

    for line in input.lines() {
        let (first, second): (&str, &str) = line.split_once(",").unwrap();
        let (first_one, first_two): (i32, i32) = split_and_parse(first);
        let (second_one, second_two): (i32, i32) = split_and_parse(second);

        println!("{} {}", first, second);
        println!("{} {} {} {}", first_one, first_two, second_one, second_two);

        if first_one <= second_one && first_two >= second_two {
            println!("{} is contained in {}", second, first);
            total_score += 1;
        } else if second_one <= first_one && second_two >= first_two {
            println!("{} is contained in {}", first, second);
            total_score += 1;
        };
    }
    total_score
}

fn part_two(input: String) -> i32 {
    let mut total_score: i32 = 0;

    for line in input.lines() {
        let (first, second): (&str, &str) = line.split_once(",").unwrap();
        let (first_one, first_two): (i32, i32) = split_and_parse(first);
        let (second_one, second_two): (i32, i32) = split_and_parse(second);

        println!("{} {}", first, second);
        println!("{} {} {} {}", first_one, first_two, second_one, second_two);

        if first_two < second_one || first_one > second_two {
            continue;
        };
        total_score += 1;
    }
    total_score
}


fn split_and_parse(line: &str) -> (i32, i32) {
    line.split_once("-").map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap())).unwrap()
}