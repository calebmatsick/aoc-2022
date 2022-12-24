use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    let mut duplicate = Vec::new();
    
    let mut lines = input.lines();
    while let (Some(line1), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next()) {
        
        let mut first_set = HashSet::new();
        let mut second_set = HashSet::new();
        let mut third_set = HashSet::new();
        let mut intersect_sets = HashSet::new();

        for c in line1.chars() {
            first_set.insert(c);
        }
        for c in line2.chars() {
            second_set.insert(c);
        }
        for c in line3.chars() { 
            third_set.insert(c);
        }

        for x in first_set.intersection(&second_set) {
            intersect_sets.insert(*x);
        }

        for x in third_set.intersection(&intersect_sets) {
            if x.is_uppercase() {
                let final_val = *x as i32 - 38;
                duplicate.push(final_val);
            }
            if x.is_lowercase() {
                let final_val = *x as i32 - 96;
                duplicate.push(final_val);
            }
        }
    }
    println!("{:?}", duplicate.iter().sum::<i32>());
}

/*
use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    let mut duplicate = Vec::new();

    for line in input.lines() {
        let (first_half, second_half) = line.split_at(line.len()/2);

        let mut first_set = HashSet::new();
        let mut second_set = HashSet::new();

        for c in first_half.chars() {
            first_set.insert(c);
        }

        for c in second_half.chars() {
            second_set.insert(c);
        }

        for c in second_set { 
            if first_set.contains(&c) { 
                if c.is_uppercase() {
                    let final_val = c as i32 - 38;
                    duplicate.push(final_val);
                }
                if c.is_lowercase() {
                    let final_val = c as i32 - 96;
                    duplicate.push(final_val);
                }
            }
        }
    }
    println!("{:?}", duplicate.iter().sum::<i32>());
}
*/