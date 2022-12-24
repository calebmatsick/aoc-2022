use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let input_slice: Vec<_> = input.chars().collect();
    let mut index = 14;
    for window in input_slice.windows(14) {

        let mut cur_window = HashSet::new();
        cur_window.insert(window[0]);
        cur_window.insert(window[1]);
        cur_window.insert(window[2]);
        cur_window.insert(window[3]);
        cur_window.insert(window[4]);
        cur_window.insert(window[5]);
        cur_window.insert(window[6]);
        cur_window.insert(window[7]);
        cur_window.insert(window[8]);
        cur_window.insert(window[9]);
        cur_window.insert(window[10]);
        cur_window.insert(window[11]);        
        cur_window.insert(window[12]);
        cur_window.insert(window[13]);

        if cur_window.len() == 14 {
            println!("{}", index);
            break;
        }
        else {
            index = index + 1;
        }
    }
}