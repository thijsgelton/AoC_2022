use std::{fs, io};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let path = "src/bin/day 2/input.txt";

    let input = fs::read_to_string(path)?;

    let _strategy_one: HashMap<&str, (&str, i32)> = HashMap::from([
        ("A X", ("3 (draw) + 1", 4)),
        ("A Y", ("6 (win) + 2", 8)),
        ("A Z", ("0 (lose) + 3", 3)),
        ("B X", ("0 (lose) + 1", 1)),
        ("B Y", ("3 (draw) + 2", 5)),
        ("B Z", ("6 (win) + 3", 9)),
        ("C X", ("6 (win) + 1", 7)),
        ("C Y", ("0 (lose) + 2", 2)),
        ("C Z", ("3 (draw) + 3", 6)),
    ]);
    let strategy_two: HashMap<&str, (&str, i32)> = HashMap::from([
        ("A X", ("0 (lose) + 3", 3)),
        ("A Y", ("3 (draw) + 1", 4)),
        ("A Z", ("6 (win) + 2", 8)),
        ("B X", ("0 (lose) + 1", 1)),
        ("B Y", ("3 (draw) + 2", 5)),
        ("B Z", ("6 (win) + 3", 9)),
        ("C X", ("0 (lose) + 2", 2)),
        ("C Y", ("3 (draw) + 3", 6)),
        ("C Z", ("6 (win) + 1", 7)),
    ]);

    let mut total_score: i32 = 0;

    for line in input.lines() {
        let (outcome_string, score): (&str, i32) = *strategy_two.get(line.trim()).unwrap();
        if let Some((theirs, mine)) = line.split_once(" ") {
            println!("They play: {}, I play: {} | {} = {}", theirs, mine, outcome_string, score);
        };
        total_score += score;
    }
    println!("Total score: {}", total_score);
    Ok(())
}
