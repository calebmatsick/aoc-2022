use std::fs;

fn main() {
    // Potential alternate answer
    //let answer = include_str!("../input.txt").split("/n").map(|x| x.lines().map(|x| x.parse::<u32>()).sum::<u32>()).max();
    //println!("{:?}", Some(answer));

    let file_path = "C:\\Users\\caleb\\Documents\\Personal Documents\\Advent of Code 2022\\day_1\\input.txt";

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
    println!{"{}", top_3_calories_total};
}
