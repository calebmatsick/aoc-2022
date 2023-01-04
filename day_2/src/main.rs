use std::{collections::HashMap, io::Stderr};

fn original_solution_part1(input: &str) {
    let rubric = HashMap::from([
        ("A X", 4),
        ("A Y", 8),
        ("A Z", 3),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 7),
        ("C Y", 2),
        ("C Z", 6),
    ]);

    let mut score = 0;

    for lines in input.lines() {
        let round = rubric[lines];
        score = score + round;
    }    

    println!("{}", score);
}

fn original_solution_part2(input: &str) {
    let rubric = HashMap::from([
        ("A X", 3),
        ("A Y", 4),
        ("A Z", 8),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 2),
        ("C Y", 6),
        ("C Z", 7),
    ]);

    let mut score = 0;

    for lines in input.lines() {
        let round = rubric[lines];
        score = score + round;
    }    

    println!("{}", score);
}

fn improved_solution_part1(input: &str) {
    let rubric = HashMap::from([
        ("A X", 4),
        ("A Y", 8),
        ("A Z", 3),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 7),
        ("C Y", 2),
        ("C Z", 6),
    ]);

    let score: usize = input.lines().map(|line| rubric[line]).sum();

    println!("{}", score);
}

fn improved_solution_part2(input: &str) {
    let rubric = HashMap::from([
        ("A X", 3),
        ("A Y", 4),
        ("A Z", 8),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 2),
        ("C Y", 6),
        ("C Z", 7),
    ]);

    let score: usize = input.lines().map(|line| rubric[line]).sum();

    println!("{}", score);
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
struct Round {
    their: Move,
    your: Move,
}

impl TryFrom<char> for Move {
    fn try_from(c: char) -> Result<Self, Stderr> {
        match c {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err(),
        }
    }
}

impl FromStr for Round {
    fn from_str(s: &str) -> Result<Self, Stderr> {
        let mut chars = s.chars();
                let (Some(theirs), Some(' '), Some(ours), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err();
        };

        Ok(Self {
            their: their.try_into()?,
            your: your.try_into()?,
        })
    }
}

fn main() {
    let input = include_str!("../input.txt");

    original_solution_part1(&input);
    original_solution_part2(&input);
    improved_solution_part1(&input);
    improved_solution_part2(&input);
}
