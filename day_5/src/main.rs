fn main() {
    let input = include_str!("../move_input.txt");

    let mut ship_crates = vec![vec!["V", "C", "D", "R", "Z", "G", "B", "W"],
                            vec!["G", "W", "F", "C", "B", "S", "T", "V"],
                            vec!["C", "B", "S", "N", "W"],
                            vec!["Q", "G", "M", "N", "J", "V", "C", "P"],
                            vec!["T", "S", "L", "F", "D", "H", "B"],
                            vec!["J", "V", "T", "W", "M", "N"],
                            vec!["P", "F", "L", "C", "S", "T", "G"],
                            vec!["B", "D", "Z"],
                            vec!["M", "N", "Z", "W"]];

    for line in input.lines() {
        let vec_line: Vec<&str> = line.split(" ").collect();
        let mut crane_stack = Vec::new();
        let mut i = 0;
        while i < vec_line[1].parse::<usize>().unwrap() {
            let move_crate: Option<&str> = ship_crates[vec_line[3].parse::<usize>().unwrap()-1].last().copied();
            ship_crates[vec_line[3].parse::<usize>().unwrap()-1].pop();
            crane_stack.push(move_crate.unwrap());
            i = i + 1;
        }
        i = 0;
        let crane_len = crane_stack.len();
        while i < crane_len {
            ship_crates[vec_line[5].parse::<usize>().unwrap()-1].push(crane_stack.pop().unwrap());
            i = i + 1;
        }
    }
    println!("{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}", ship_crates[0].last(), ship_crates[1].last(), ship_crates[2].last(), ship_crates[3].last(), ship_crates[4].last(), ship_crates[5].last(), ship_crates[6].last(), ship_crates[7].last(), ship_crates[8].last());
}