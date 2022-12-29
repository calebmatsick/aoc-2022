use std::collections::HashMap;

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

fn main() {
    let input = include_str!("../input.txt");

    original_solution_part1(&input);
    original_solution_part2(&input);
    improved_solution_part1(&input);
    improved_solution_part2(&input);
}
