use std::error::Error;
use std::fs;
use std::io::{BufRead, BufReader};

pub fn main(part: u32, input: String) -> Result<(), Box<dyn Error>> {
    let contents = read_file(input)?;

    match part {
        1 => part_one(contents).ok_or("part one failed".into()),
        2 => part_two(contents).ok_or("part two failed".into()),
        _ => panic!("invalid part {}", part),
    }
}

// For each line in the input, combine the first and last digits form a two digit number, then sum
// the numbers of every line.
fn part_one(contents: Vec<String>) -> Option<()> {
    let mut numbers: Vec<u32> = Vec::new();

    for data in contents.iter() {
        let line: Vec<char> = data.chars().collect();
        let mut start = 0;
        let mut end = line.len() - 1;

        while start <= end {
            if line[start].is_digit(10) && line[end].is_digit(10) {
                break;
            }

            if !line[start].is_digit(10) {
                start += 1;
            }

            if !line[end].is_digit(10) {
                end -= 1;
            }
        }

        numbers.push((line[start].to_digit(10)? * 10) + line[end].to_digit(10)?);
    }

    let sum: u32 = numbers.iter().sum();
    println!("part one sum: {}", sum);

    Some(())
}

// For each line in the input, combine the first and last digits form a two digit number, then sum
// the numbers of every line. The digits can also be spelled out.
fn part_two(contents: Vec<String>) -> Option<()> {
    let mut numbers: Vec<u32> = Vec::new();

    for data in contents.iter() {
        let line: Vec<char> = data.chars().collect();
        let mut start = 0;
        let mut end = line.len() - 1;

        while start <= end {
            let first = convert_num(&line, start);
            let second = convert_num(&line, end);

            if first.is_none() {
                start += 1;
            }

            if second.is_none() {
                end -= 1;
            }

            if let Some(first_val) = first {
                if let Some(second_val) = second {
                    numbers.push((first_val * 10) + second_val);
                    break;
                }
            }
        }
    }

    let sum: u32 = numbers.iter().sum();
    println!("part two sum: {}", sum);

    Some(())
}

// Convert a string into a number. The input can be either the character representation or written
// as a word.
fn convert_num(input: &Vec<char>, position: usize) -> Option<u32> {
    if input[position].is_digit(10) {
        return input[position].to_digit(10);
    }

    let mut offset = 1;
    while position + offset < input.len() && offset <= 4 {
        let word: String = input[position..position + offset + 1].iter().collect();
        match word.as_str() {
            "one" => return Some(1),
            "two" => return Some(2),
            "three" => return Some(3),
            "four" => return Some(4),
            "five" => return Some(5),
            "six" => return Some(6),
            "seven" => return Some(7),
            "eight" => return Some(8),
            "nine" => return Some(9),
            _ => {}
        }
        offset += 1;
    }

    None
}

fn read_file(path: String) -> std::io::Result<Vec<String>> {
    let mut contents = Vec::new();
    let file = fs::File::open(path)?;

    for line in BufReader::new(file).lines() {
        if let Ok(data) = line {
            contents.push(data.to_string());
        }
    }

    Ok(contents)
}
