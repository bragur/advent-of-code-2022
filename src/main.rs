use std::env;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use substring::Substring;

fn vecs_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file: File = File::open(filename).expect("no such file");
    let buf: BufReader<File> = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

// #1 ⭐ / #2 ⭐ // DAY 1
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

// #3 ⭐ / #4 ⭐ // DAY 2
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

// #5 ⭐ // DAY 3
fn _rucksack_reorganization_1(lines: Vec<String>) -> i32 {
    lines.iter().fold(0, |acc, x| {
        let first = x.substring(0, x.len() / 2);
        let second = x.substring(x.len() / 2, x.len());
        let mut result = 0;
        for c in first.chars() {
            if second.find(c) != None {
                result = if c.is_uppercase() {
                    c as i32 - 38
                } else {
                    c as i32 - 96
                };
                break;
            }
        }
        acc + result
    })
}

// #6 ⭐ // DAY 3
fn _rucksack_reorganization_2(lines: Vec<String>) -> i32 {
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

// #7 ⭐ // DAY 4
fn _camp_cleanup_1(input: Vec<String>) -> i32 {
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

// #8 ⭐ // DAY 4
fn _camp_cleanup_2(input: Vec<String>) -> i32 {
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
        let overlaps = first_section.0 <= second_section.1 && second_section.0 <= first_section.1;
        if overlaps {
            acc + 1
        } else {
            acc
        }
    })
}

// #9 ⭐
fn _supply_stacks_9(input: Vec<String>) -> String {
    let mut rotated = Vec::new();
    let mut instructions_lines = Vec::new();
    for line in input {
        let first = line.chars().collect::<Vec<char>>().into_iter().nth(1);
        if !line.is_empty() && first != Some('1') && first != Some('o') {
            rotated.push(
                line.chars()
                    .collect::<Vec<char>>()
                    .into_iter()
                    .enumerate()
                    .filter_map(|(i, c)| if (i - 1) % 4 == 0 { Some(c) } else { None })
                    .collect::<Vec<char>>(),
            )
        } else if first == Some('o') {
            instructions_lines.push(line)
        }
    }
    let crates_length = match rotated.iter().nth(1) {
        Some(vec) => vec.len(),
        None => 0,
    };

    let mut crates: Vec<Vec<char>> = Vec::new();
    for pos in 0..(crates_length) {
        let mut c: Vec<char> = Vec::new();
        for l in rotated.iter().rev() {
            match l.iter().nth(pos) {
                Some(foo) => {
                    if !foo.is_whitespace() {
                        c.push(*foo)
                    }
                }
                None => (),
            }
        }
        crates.push(c);
    }

    let instructions = instructions_lines.iter().map(|l| {
        l.split(' ')
            .into_iter()
            .flat_map(|c| c.parse::<i32>())
            .collect::<Vec<i32>>()
    });

    for instruction in instructions {
        let amount = instruction.get(0).unwrap();
        let from_stack = instruction.get(1).unwrap();
        let to_stack = instruction.get(2).unwrap();

        for _i in 0..(*amount) {
            let vec: &mut Vec<char> = crates.get_mut(*from_stack as usize - 1).unwrap();
            let c: char = vec.pop().unwrap();
            crates.get_mut(*to_stack as usize - 1).unwrap().push(c);
        }
    }
    let mut chars = String::from("");
    crates.iter().for_each(|c| {
        let c = c.last().unwrap();
        chars.push(*c);
    });

    chars
}

// #10 ⭐
fn supply_stacks_10(input: Vec<String>) -> String {
    let mut rotated = Vec::new();
    let mut instructions_lines = Vec::new();
    for line in input {
        let first = line.chars().collect::<Vec<char>>().into_iter().nth(1);
        if !line.is_empty() && first != Some('1') && first != Some('o') {
            rotated.push(
                line.chars()
                    .collect::<Vec<char>>()
                    .into_iter()
                    .enumerate()
                    .filter_map(|(i, c)| if (i - 1) % 4 == 0 { Some(c) } else { None })
                    .collect::<Vec<char>>(),
            )
        } else if first == Some('o') {
            instructions_lines.push(line)
        }
    }
    let crates_length = match rotated.iter().nth(1) {
        Some(vec) => vec.len(),
        None => 0,
    };

    let mut crates: Vec<Vec<char>> = Vec::new();
    for pos in 0..(crates_length) {
        let mut c: Vec<char> = Vec::new();
        for l in rotated.iter().rev() {
            match l.iter().nth(pos) {
                Some(foo) => {
                    if !foo.is_whitespace() {
                        c.push(*foo)
                    }
                }
                None => (),
            }
        }
        crates.push(c);
    }

    let instructions = instructions_lines.iter().map(|l| {
        l.split(' ')
            .into_iter()
            .flat_map(|c| c.parse::<i32>())
            .collect::<Vec<i32>>()
    });

    for instruction in instructions {
        let amount = instruction.get(0).unwrap();
        let from_stack = instruction.get(1).unwrap();
        let to_stack = instruction.get(2).unwrap();

        let mut whole_stack = Vec::new();
        for _i in 0..(*amount) {
            let vec: &mut Vec<char> = crates.get_mut(*from_stack as usize - 1).unwrap();
            let c: char = vec.pop().unwrap();
            whole_stack.push(c);
        }
        whole_stack
            .iter()
            .rev()
            .for_each(|c| crates.get_mut(*to_stack as usize - 1).unwrap().push(*c));
    }
    let mut chars = String::from("");
    crates.iter().for_each(|c| {
        let c = c.last().unwrap();
        chars.push(*c);
    });

    chars
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path: &String = &args[1];
    println!("File: {}", file_path);

    let lines = vecs_from_file(file_path);
    use std::time::Instant;
    let now = Instant::now();
    let message = supply_stacks_10(lines);
    println!("Crates: {}", message);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
