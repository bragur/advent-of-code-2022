use std::env;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn vecs_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file: File = File::open(filename).expect("no such file");
    let buf: BufReader<File> = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

// #1 ⭐ / #2 ⭐
fn _find_max(input: Vec<String>) -> (i32, i32) {
    let mut calorie_collection: Vec<i32> = Vec::new();
    let (max, _) = input.into_iter().fold(
        (0, 0),
        |(current_max, current_elf_accumulation), calories_string| {
            let mut result = (current_max, current_elf_accumulation);
            if calories_string.is_empty() && current_elf_accumulation > current_max {
                calorie_collection.push(current_elf_accumulation);
                result = (current_elf_accumulation, 0);
            } else if calories_string.is_empty() {
                calorie_collection.push(current_elf_accumulation);
                result = (current_max, 0);
            }
            if let Ok(calories) = calories_string.parse::<i32>() {
                result = (current_max, current_elf_accumulation + calories)
            }
            result
        },
    );

    calorie_collection.sort();
    calorie_collection.reverse();
    calorie_collection.truncate(3);

    (max, calorie_collection.iter().fold(0, |acc, x| acc + x))
}

// #3 ⭐ / #4 ⭐
fn _get_score(lines: Vec<String>) -> (i32, i32) {
    lines.iter().fold((0, 0), |acc, x| {
        let score = match x.as_str() {
            "A X" => (4, 3),
            "A Y" => (8, 4),
            "A Z" => (3, 8),
            "B X" => (1, 1),
            "B Y" => (5, 5),
            "B Z" => (9, 9),
            "C X" => (1, 2),
            "C Y" => (6, 6),
            "C Z" => (7, 7),
            _ => (0, 0),
        };
        (acc.0 + score.0, acc.1 + score.1)
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path: &String = &args[1];
    println!("File: {}", file_path);

    let lines = vecs_from_file(file_path);
    use std::time::Instant;
    let now = Instant::now();
    let score = _find_max(lines);
    let elapsed = now.elapsed();
    println!("Score: {:?}", score);
    println!("Elapsed: {:.2?}", elapsed);
}
