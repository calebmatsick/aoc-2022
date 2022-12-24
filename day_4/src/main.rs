fn main() {
    let input = include_str!("../input.txt");

    let mut total_overlap = 0;

    for line in input.lines() {
        let split_line = line.split(",").collect::<Vec<_>>();
        let left_split = split_line[0].split("-").collect::<Vec<_>>();
        let right_split = split_line[1].split("-").collect::<Vec<_>>();

        if left_split[0].parse::<i32>().unwrap() >= right_split[0].parse::<i32>().unwrap() && left_split[0].parse::<i32>().unwrap() <= right_split[1].parse::<i32>().unwrap() || left_split[1].parse::<i32>().unwrap() <= right_split[1].parse::<i32>().unwrap() && left_split[1].parse::<i32>().unwrap() >= right_split[0].parse::<i32>().unwrap() {
            total_overlap = total_overlap + 1;
            continue;
        }
        if right_split[0].parse::<i32>().unwrap() >= left_split[0].parse::<i32>().unwrap() && right_split[0].parse::<i32>().unwrap() <= left_split[1].parse::<i32>().unwrap() || right_split[1].parse::<i32>().unwrap() <= left_split[1].parse::<i32>().unwrap() && right_split[1].parse::<i32>().unwrap() >= left_split[0].parse::<i32>().unwrap(){
            total_overlap = total_overlap + 1;
            continue;
        }
    }
    println!("{}", total_overlap);
}