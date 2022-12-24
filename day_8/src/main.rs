fn main() {
    let forest: Vec<Vec<u32>> = include_str!("../input.txt").lines().map(|line| line.chars().flat_map(|ch| ch.to_digit(10)).collect::<Vec<u32>>()).collect();
    let mut tree_scenic_scores = Vec::new();
    let mut y = 0;

    for row in &forest {
        let mut col_index = 0;
        let mut x = 0;

        for candidate_tree in row {
            let col = forest.iter().map(|t| t[col_index]).collect::<Vec<_>>();

            if x < row.len()-1 && y == 0 || x == row.len()-1 || y < col.len()-1 && x == 0 || y == col.len()-1 {
                x += 1;
                col_index += 1;
                continue;
            }

            let mut left_view_trees = 0;
            let mut right_view_trees = 0;
            let mut top_view_trees = 0;
            let mut bottom_view_trees = 0;

            for view_tree in row.get(..x).unwrap().iter().rev().collect::<Vec<_>>() {
                if view_tree < candidate_tree {
                    left_view_trees += 1;
                }
                else {
                    left_view_trees += 1;
                    break; 
                }
            }
            for view_tree in row.get(x+1..).unwrap() {
                if view_tree < candidate_tree {
                    right_view_trees += 1;
                }
                else {
                    right_view_trees += 1;
                    break; 
                }
            }
            for view_tree in col.get(..y).unwrap().iter().rev().collect::<Vec<_>>() {
                if view_tree < candidate_tree {
                    top_view_trees += 1;
                }
                else {
                    top_view_trees += 1;
                    break; 
                }
            }
            for view_tree in col.get(y+1..).unwrap() {
                if view_tree < candidate_tree {
                    bottom_view_trees += 1;
                }
                else {
                    bottom_view_trees += 1;
                    break; 
                }
            }
            tree_scenic_scores.push(left_view_trees * right_view_trees * top_view_trees * bottom_view_trees);
            x += 1;
            col_index += 1;
        }
        y += 1;
    }
    println!("{}", tree_scenic_scores.iter().max().unwrap());
}

/*
fn main() {
    let forest: Vec<Vec<u32>> = include_str!("../input_test.txt").lines().map(|line| line.chars().flat_map(|ch| ch.to_digit(10)).collect::<Vec<u32>>()).collect();
    let mut total_visible_trees = 0;
    let mut y = 0;

    for row in &forest {
        let mut col_index = 0;
        let mut x = 0;

        for tree in row {
            let col = forest.iter().map(|t| t[col_index]).collect::<Vec<_>>();

            if row.get(..x).unwrap_or_default().iter().max().unwrap_or_else(|| {&0}) < &tree || row.get(x+1..).unwrap_or_default().iter().max().unwrap_or_else(|| {&0}) < &tree || col.get(..y).unwrap_or_default().iter().max().unwrap_or_else(|| {&0}) < &tree || col.get(y+1..).unwrap_or_default().iter().max().unwrap_or_else(|| {&0}) < &tree || x == 0 || y == 0 || x == row.len()-1 || y == col.len()-1 {
                  total_visible_trees += 1;
            }
            x += 1;
            col_index += 1;
        }
        y += 1;
    }
    println!("{}", total_visible_trees);
}
*/