use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 3);

    let contents = read_file(&args[1]);

    match args[2].as_str() {
        "--part1" => part_one(contents),
        "--part2" => part_two(contents),
        _ => panic!("unknown argument {}", args[2]),
    }
}

// Sum all the numbers that are adjacent to a symbol that is not '.'.
fn part_one(contents: Vec<Vec<char>>) {
    let mut column = 0;
    let mut row = 0;

    let mut part_numbers: Vec<u32> = Vec::new();

    while row < contents.len() {
        while column < contents[row].len() {
            if contents[row][column].is_digit(10) {
                let mut end = column;
                while end + 1 < contents[row].len() && contents[row][end + 1].is_digit(10) {
                    end += 1;
                }

                let number = contents[row][column..=end]
                    .iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();

                let check_row_start = if row == 0 { 0 } else { row - 1 };
                let check_row_end = if row >= contents.len() - 1 {
                    contents.len() - 1
                } else {
                    row + 1
                };

                'check: for check_row in check_row_start..=check_row_end {
                    let check_col_start = if column == 0 { 0 } else { column - 1 };
                    let check_col_end = if end >= contents[row].len() - 1 {
                        contents[row].len() - 1
                    } else {
                        end + 1
                    };

                    for check_col in check_col_start..=check_col_end {
                        let check = contents[check_row][check_col];
                        if !check.is_digit(10) && check != '.' {
                            part_numbers.push(number);
                            break 'check;
                        }
                    }
                }

                column = end;
            }

            column += 1;
        }
        row += 1;
        column = 0;
    }

    let sum: u32 = part_numbers.iter().sum();
    println!("sum of part numbers: {}", sum);
}

// Sum the product of adjacent numbers for every star with exactly two adjacent numbers.
fn part_two(contents: Vec<Vec<char>>) {
    let mut ratios: Vec<u32> = Vec::new();

    let mut row = 0;
    let mut column = 0;

    while row < contents.len() {
        while column < contents[row].len() {
            print!("{}", contents[row][column]);
            if contents[row][column] == '*' {
                let adjacent_nums = adjacent_numbers(&contents, row, column);
                if adjacent_nums.len() == 2 {
                    ratios.push(adjacent_nums[0] * adjacent_nums[1]);
                }
            }

            column += 1;
        }

        row += 1;
        column = 0;
        println!("");
    }

    let sum: u32 = ratios.iter().sum();
    println!("sum of gear ratios: {}", sum);
}

fn adjacent_numbers(contents: &Vec<Vec<char>>, row: usize, column: usize) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();

    if row > 0 {
        if contents[row - 1][column].is_digit(10) {
            numbers.push(parse_number(contents, row - 1, column));
        } else {
            if column > 0 {
                if contents[row - 1][column - 1].is_digit(10) {
                    numbers.push(parse_number(contents, row - 1, column - 1));
                }
            }

            if column < contents[row - 1].len() {
                if contents[row - 1][column + 1].is_digit(10) {
                    numbers.push(parse_number(contents, row - 1, column + 1));
                }
            }
        }
    }

    if row < contents.len() {
        if contents[row + 1][column].is_digit(10) {
            numbers.push(parse_number(contents, row + 1, column));
        } else {
            if column > 0 {
                if contents[row + 1][column - 1].is_digit(10) {
                    numbers.push(parse_number(contents, row + 1, column - 1));
                }
            }

            if column < contents[row + 1].len() {
                if contents[row + 1][column + 1].is_digit(10) {
                    numbers.push(parse_number(contents, row + 1, column + 1));
                }
            }
        }
    }

    if column > 0 {
        if contents[row][column - 1].is_digit(10) {
            numbers.push(parse_number(contents, row, column - 1));
        }
    }

    if column < contents[row].len() {
        if contents[row][column + 1].is_digit(10) {
            numbers.push(parse_number(contents, row, column + 1));
        }
    }

    numbers
}

fn parse_number(contents: &Vec<Vec<char>>, row: usize, column: usize) -> u32 {
    let mut start = column;
    while start > 0 && contents[row][start - 1].is_digit(10) {
        start -= 1;
    }

    let mut end = column;
    while end + 1 < contents[row].len() && contents[row][end + 1].is_digit(10) {
        end += 1;
    }

    contents[row][start..=end]
        .iter()
        .collect::<String>()
        .parse::<u32>()
        .unwrap()
}

fn read_file(path: &str) -> Vec<Vec<char>> {
    let mut contents: Vec<Vec<char>> = Vec::new();

    let file = File::open(path).unwrap();
    for line in BufReader::new(file).lines() {
        if let Ok(data) = line {
            contents.push(data.chars().collect());
        }
    }

    contents
}
