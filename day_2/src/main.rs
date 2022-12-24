use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");

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

// A for Rock, B for Paper, and C for Scissors - Opp
// X for Rock, Y for Paper, and Z - You
// 1 for Rock, 2 for Paper, and 3 for Scissors
// 0 if you lost, 3 if the round was a draw, and 6 if you won

// PART TWO: X - lose, Y - draw, Z - win