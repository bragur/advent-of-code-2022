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
fn _calorie_counting(input: Vec<String>) -> (i32, i32) {
    let mut calorie_collection: Vec<i32> = Vec::new();
    let (max, _) =
        input
            .into_iter()
            .fold((0, 0), |(current_max, current_elf_acc), calories_string| {
                let mut result = (current_max, current_elf_acc);
                if calories_string.is_empty() && current_elf_acc > current_max {
                    calorie_collection.push(current_elf_acc);
                    result = (current_elf_acc, 0);
                } else if calories_string.is_empty() {
                    calorie_collection.push(current_elf_acc);
                    result = (current_max, 0);
                }
                if let Ok(calories) = calories_string.parse::<i32>() {
                    result = (current_max, current_elf_acc + calories)
                }
                result
            });

    calorie_collection.sort();
    calorie_collection.reverse();
    calorie_collection.truncate(3);

    (max, calorie_collection.iter().fold(0, |acc, x| acc + x))
}

// #3 ⭐ / #4 ⭐
fn _rock_paper_scissors(lines: Vec<String>) -> (i32, i32) {
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

// #5 ⭐ / #6 ⭐
fn _rucksack_reorganization(lines: Vec<String>) -> i32 {
    let (result, _noop) = lines.iter().fold((0, Vec::new()), |mut acc, x| {
        if acc.1.len() < 3 {
            acc.1.push(x.to_string());
        }
        let mut result = 0;
        if acc.1.len() == 3 {
            match (acc.1.get(0), acc.1.get(1), acc.1.get(2)) {
                (Some(first), Some(second), Some(third)) => {
                    for c in first.chars() {
                        if second.find(c) != None && third.find(c) != None {
                            result = if c.is_uppercase() {
                                c as i32 - 38
                            } else {
                                c as i32 - 96
                            };

                            acc.1 = Vec::new();
                            break;
                        }
                    }
                }
                _ => (),
            }
        }
        (acc.0 + result, acc.1)
    });

    result
}

// #7 ⭐
fn camp_cleanup(input: Vec<String>) -> i32 {
    input.iter().fold(0, |acc, x| {
        let pairs = x.split_once(',').unwrap();
        let _first_section = pairs.0.split_once('-').unwrap();
        let first_section = (
            _first_section.0.parse::<i32>().unwrap(),
            _first_section.1.parse::<i32>().unwrap(),
        );
        let _second_section = pairs.1.split_once('-').unwrap();
        let second_section = (
            _second_section.0.parse::<i32>().unwrap(),
            _second_section.1.parse::<i32>().unwrap(),
        );
        let first_section_is_subset =
            first_section.0 >= second_section.0 && first_section.1 <= second_section.1;
        let second_seciont_is_subset =
            second_section.0 >= first_section.0 && second_section.1 <= first_section.1;
        if first_section_is_subset || second_seciont_is_subset {
            acc + 1
        } else {
            acc
        }
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path: &String = &args[1];
    println!("File: {}", file_path);

    let lines = vecs_from_file(file_path);
    use std::time::Instant;
    let now = Instant::now();
    let sum = camp_cleanup(lines);
    println!("Score: {}", sum);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
