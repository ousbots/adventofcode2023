use std::error::Error;
use std::fs;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug)]
struct Card {
    winners: Vec<i32>,
    scratch: Vec<i32>,
}

pub fn main(part: u32, input: String) -> Result<(), Box<dyn Error>> {
    let contents = read_file(input)?;

    match part {
        1 => part_one(contents),
        2 => part_two(contents),
        _ => panic!("invalid part {}", part),
    }
}

fn part_one(cards: Vec<Card>) -> Result<(), Box<dyn Error>> {
    let wins = calc_wins(cards)?;

    let score: u32 = wins
        .iter()
        .map(|count| u32::pow(2, *count))
        .collect::<Vec<u32>>()
        .iter()
        .sum();

    println!("scratch cards are worth {} points", score);

    Ok(())
}

fn part_two(cards: Vec<Card>) -> Result<(), Box<dyn Error>> {
    let wins = calc_wins(cards)?;
    let mut dups: Vec<i32> = vec![1; wins.len()];

    for i in 0..wins.len() {
        if wins[i] > 0 {
            for j in (i + 1)..(i + 1 + (wins[i] as usize)).min(wins.len()) {
                dups[j] += dups[i];
            }
        }
    }

    let sum: i32 = dups.iter().sum();

    println!("{} total scratch cards won", sum);

    Ok(())
}

fn calc_wins(cards: Vec<Card>) -> Result<Vec<u32>, Box<dyn Error>> {
    let mut wins: Vec<u32> = Vec::new();

    for mut card in cards {
        card.winners.sort_unstable();
        card.scratch.sort_unstable();

        let mut first = 0;
        let mut second = 0;
        let mut count = 0;

        while first < card.winners.len() && second < card.scratch.len() {
            if card.winners[first] < card.scratch[second] {
                first += 1;
            } else if card.winners[first] > card.scratch[second] {
                second += 1;
            } else {
                count += 1;
                first += 1;
            }
        }

        wins.push(count);
    }

    Ok(wins)
}

fn read_file(path: String) -> Result<Vec<Card>, Box<dyn Error>> {
    let mut contents: Vec<Card> = Vec::new();

    let file = fs::File::open(path)?;
    for line in BufReader::new(file).lines() {
        if let Ok(data) = line {
            let split: Vec<&str> = data.split("|").collect();
            let winners_strs: Vec<&str> = split[0].split(":").collect::<Vec<&str>>()[1]
                .split(" ")
                .collect();
            let scratch_strs: Vec<&str> = split[1].split(" ").collect();

            let card = Card {
                winners: winners_strs
                    .iter()
                    .filter(|x| x.len() > 0)
                    .map(|x| x.parse::<i32>())
                    .collect::<Result<Vec<i32>, _>>()?,
                scratch: scratch_strs
                    .iter()
                    .filter(|x| x.len() > 0)
                    .map(|x| x.parse::<i32>())
                    .collect::<Result<Vec<i32>, _>>()?,
            };

            contents.push(card);
        }
    }

    Ok(contents)
}
