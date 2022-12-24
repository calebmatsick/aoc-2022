use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let mut dirs = Vec::new();
    let mut dir_sizes: HashMap<String, i32> = HashMap::new();
    let mut i = 0;

    for line in input.lines() {
        if line == "$ cd .." {
            dirs.pop();
            continue;
        }
        else if line.split(" ").collect::<Vec<&str>>()[1] == "cd" {
            dirs.push(format!("{}{}", i.to_string().as_str(), line.split(" ").collect::<Vec<&str>>()[2]).to_owned());
            i += 1;
            continue;
        }
        else if line == "$ ls" {
            continue;
        }
        else if line.split(" ").collect::<Vec<&str>>()[0] == "dir" {
            continue;
        }
        else {
            for dir in dirs.clone() {
                dir_sizes.entry(dir.to_string()).and_modify(|value| *value += line.split(" ").collect::<Vec<&str>>()[0].parse::<i32>().unwrap()).or_insert(line.split(" ").collect::<Vec<&str>>()[0].parse::<i32>().unwrap());
            }
        }
    }
    println!("{}", dir_sizes.into_values().filter(|x| x >= &mut 3598596).collect::<Vec<i32>>().iter().min().unwrap());
}