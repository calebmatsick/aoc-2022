use std::collections::HashSet;

fn main() {
    let input = include_str!("../input_test.txt");
    let mut visited_pos = HashSet::new();
    let mut cur_head_pos = (0, 0);
    let mut cur_tail_pos = (0, 0);

    for line in input.lines() {
        let moves = line.split(" ").collect::<Vec<&str>>();
        println!("{:?}", moves);

        match moves[0] {
            "L" => for i in 0..moves[1].parse::<i32>().unwrap() {
                visited_pos.insert(cur_tail_pos);
                if cur_tail_pos == (cur_head_pos.0+1, cur_head_pos.1) {
                    cur_head_pos.0 -= 1;
                    cur_tail_pos.0 -= 1;
                }
            },
            "R" => for i in 0..moves[1].parse::<i32>().unwrap() {
                visited_pos.insert(cur_tail_pos);
                if cur_tail_pos == (cur_head_pos.0-1, cur_head_pos.1) {
                    cur_head_pos.0 += 1;
                    cur_tail_pos.0 += 1;
                }
            },
            "U" => for i in 0..moves[1].parse::<i32>().unwrap() {
                visited_pos.insert(cur_tail_pos);
                if cur_tail_pos == (cur_head_pos.0, cur_head_pos.1-1) {
                    cur_head_pos.0 += 1;
                    cur_tail_pos.0 += 1;
                }
            },
            "D" => for i in 0..moves[1].parse::<i32>().unwrap() {
                visited_pos.insert(cur_tail_pos);
                if cur_tail_pos == (cur_head_pos.0, cur_head_pos.1+1) {
                    cur_head_pos.0 -= 1;
                    cur_tail_pos.0 -= 1;
                }
            },
            _ => println!("Hopefully we never see this..."),
        }
    }
    println!("{}", visited_pos.len());
}
