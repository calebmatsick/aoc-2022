use std::fs;


fn original_solution() {
    let file_path = "/home/firstcitizen/aoc-2022/day_1/input.txt";

    let contents = fs::read_to_string(file_path).expect("Should be able to read the file");

    let mut high_calories_1: i32 = 0;
    let mut high_calories_2: i32 = 0;
    let mut high_calories_3: i32 = 0;
    let mut running_total: i32 = 0;

    for line in contents.lines() {
        if !line.is_empty() {
            let line_num: i32 = line.parse().unwrap();
            running_total = running_total + line_num;
        }
        if line.is_empty() {
            if running_total > high_calories_1 {
                high_calories_3 = high_calories_2;
                high_calories_2 = high_calories_1;
                high_calories_1 = running_total;
            }
            else if running_total > high_calories_2 {
                high_calories_3 = high_calories_2;
                high_calories_2 = running_total;
            }
            else if running_total > high_calories_3 {
                high_calories_3 = running_total;
            }
            running_total = 0;
        }
    }
    let top_3_calories_total: i32 = high_calories_1 + high_calories_2 + high_calories_3;
    println!{"(Part 2) Original solution answer:{}", top_3_calories_total};
}


fn improved_solution_part1(){
    let answer = include_str!("../input.txt")
        .split("\n\n")
        .map(|group| group
             .lines()
             .flat_map(|line| line.parse::<usize>())
             .sum::<usize>()
        )
        .max();

    println!("(Part 1) Improved solution answer: {}", answer.unwrap());
}

fn improved_solution_part2(){
    let mut answer = include_str!("../input.txt")
        .split("\n\n")
        .map(|group| group
             .lines()
             .flat_map(|line| line.parse::<usize>())
             .sum::<usize>()
        )
        .collect::<Vec<usize>>();

    answer.sort_by(|a, b| b.cmp(a));

    println!("(Part 2) Improved solution answer: {}", answer.iter().take(3).sum::<usize>());
}

fn main() {
    original_solution();
    improved_solution_part1();
    improved_solution_part2();
}
