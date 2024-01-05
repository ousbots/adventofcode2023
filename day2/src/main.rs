use std::env;
use std::fs;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug)]
struct Play {
    game: u32,
    sets: Vec<Set>,
}

#[derive(Clone, Copy, Debug)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 3);

    let first_part = match args[2].as_str() {
        "--part1" => true,
        "--part2" => false,
        _ => panic!("invalid argument"),
    };

    let contents = read_file(&args[1]);

    if first_part {
        part_one(contents);
    } else {
        part_two(contents);
    }
}

// Sum the game numbers which are possible with a given maximum of 12 red, 13 green, and 14 blue.
fn part_one(games: Vec<Play>) {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut possible: Vec<u32> = Vec::new();
    'game: for game in games {
        for set in game.sets {
            if set.red > max_red {
                continue 'game;
            }

            if set.green > max_green {
                continue 'game;
            }

            if set.blue > max_blue {
                continue 'game;
            }
        }

        possible.push(game.game);
    }

    let total: u32 = possible.iter().sum();
    println!("sum of possible game numbers {}", total);
}

// Sum the powers of the minimum number of colors needed for each set.
fn part_two(games: Vec<Play>) {
    let mut minimum_sets: Vec<Set> = Vec::new();

    for game in games {
        let mut min_set = Set {
            red: 0,
            green: 0,
            blue: 0,
        };

        for set in game.sets {
            min_set.red = set.red.max(min_set.red);
            min_set.green = set.green.max(min_set.green);
            min_set.blue = set.blue.max(min_set.blue);
        }

        minimum_sets.push(min_set);
    }

    let sum: u32 = minimum_sets
        .iter()
        .map(|set| set.red * set.green * set.blue)
        .sum();

    println!("sum of powers of minimum sets: {}", sum);
}

fn read_file(path: &str) -> Vec<Play> {
    let mut contents: Vec<Play> = Vec::new();

    let file = fs::File::open(path).unwrap();
    for line in BufReader::new(file).lines() {
        if let Ok(data) = line {
            let mut play = Play {
                game: 0,
                sets: Vec::new(),
            };

            let orig: Vec<&str> = data.split(":").collect();
            assert!(orig.len() == 2);
            assert!(&orig[0][0..5] == "Game ");

            let game_number = orig[0][5..orig[0].len()].trim().parse::<u32>().unwrap();
            play.game = game_number;

            let raw_sets: Vec<&str> = orig[1].split(";").collect();
            for raw_set in raw_sets {
                let mut set = Set {
                    red: 0,
                    green: 0,
                    blue: 0,
                };

                let raw_colors: Vec<&str> = raw_set.split(",").collect();
                for raw_color in raw_colors {
                    let data: Vec<&str> = raw_color.trim().split(" ").collect();
                    let count = data[0].parse::<u32>().unwrap();

                    match data[1] {
                        "red" => {
                            set.red = count;
                        }
                        "green" => {
                            set.green = count;
                        }
                        "blue" => {
                            set.blue = count;
                        }
                        _ => {
                            panic!("unknown color {}", data[1]);
                        }
                    }
                }

                play.sets.push(set);
            }

            contents.push(play);
        }
    }

    contents
}
