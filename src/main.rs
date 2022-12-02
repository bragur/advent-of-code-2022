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

fn find_max(input: Vec<String>) -> (i32, Vec<i32>) {
    let mut pickles: Vec<i32> = Vec::new();
    let (max, _) = input.into_iter().fold(
        (0, 0),
        |(current_max, current_elf_accumulation), calories_string| {
            let mut result = (current_max, current_elf_accumulation);
            if calories_string.is_empty() && current_elf_accumulation > current_max {
                pickles.push(current_elf_accumulation);
                result = (current_elf_accumulation, 0);
            } else if calories_string.is_empty() {
                pickles.push(current_elf_accumulation);
                result = (current_max, 0);
            }
            if let Ok(calories) = calories_string.parse::<i32>() {
                result = (current_max, current_elf_accumulation + calories)
            }
            result
        },
    );
    (max, pickles)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path: &String = &args[1];
    println!("File: {}", file_path);
    let vecs = vecs_from_file(file_path);

    use std::time::Instant;
    let now = Instant::now();
    let (max, calory_vec) = find_max(vecs);
    let mut cal_copy = calory_vec.clone();
    cal_copy.sort();
    cal_copy.reverse();
    cal_copy.truncate(3);
    let sum_of_top_three = cal_copy.iter().fold(0, |sum, x| sum + x);
    let elapsed = now.elapsed();
    println!("Max calories: {}", max);
    println!("Top 3: {:?}", cal_copy);
    println!("Sum of top 3: {}", sum_of_top_three);
    println!("Elapsed: {:.2?}", elapsed);
}
