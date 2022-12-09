use core::panic;
use std::{fs};

#[derive(Clone, Copy)]
enum Visibility {
    Unknown,
    Visible,
    Hidden
}

fn part_1() {
    let contents = fs::read_to_string("input")
        .expect("Error reading input file :(")
        .trim()
        .split("\n")
        .map(|row| row.chars().map(|char| {
            char.to_string()
                .parse::<i32>()
                .expect("Invalid num!")
        }).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    let mut visible: Vec<Vec<Visibility>> = vec![vec![Visibility::Unknown; contents[0].len()]; contents.len()];

    
    for y in 0..contents.len() {
        let mut largest_tree = -1;
        for x in 0..contents[y].len() {
            let (new_largest_tree, new_visiblilty) = check_tree(largest_tree, contents[y][x], visible[y][x]);
            largest_tree = new_largest_tree;
            visible[y][x] = new_visiblilty;
        }
    } 

    for y in 0..contents.len() {
        let mut largest_tree = -1;
        for x in (0..contents[y].len()).rev() {
            let (new_largest_tree, new_visiblilty) = check_tree(largest_tree, contents[y][x], visible[y][x]);
            largest_tree = new_largest_tree;
            visible[y][x] = new_visiblilty;
        }
    } 

    for x in 0..contents[0].len() {
        let mut largest_tree = -1;
        for y in 0..contents.len() {
            let (new_largest_tree, new_visiblilty) = check_tree(largest_tree, contents[y][x], visible[y][x]);
            largest_tree = new_largest_tree;
            visible[y][x] = new_visiblilty;
        }
    }

    for x in 0..contents[0].len() {
        let mut largest_tree = -1;
        for y in (0..contents.len()).rev() {
            let (new_largest_tree, new_visiblilty) = check_tree(largest_tree, contents[y][x], visible[y][x]);
            largest_tree = new_largest_tree;
            visible[y][x] = new_visiblilty;
        }
    }

    let visible_trees: i32 = visible.into_iter().map(|row| {
        row.into_iter().map(|cell| {
            match cell {
                Visibility::Unknown => panic!("Should be no unknown trees!"),
                Visibility::Visible => 1,
                Visibility::Hidden => 0
            }
        }).sum::<i32>()
    }).sum();

    println!("{visible_trees}");
}

fn check_tree(largest_tree: i32, tree: i32, current_visibility: Visibility) -> (i32, Visibility) {
    if tree > largest_tree{
        (tree, Visibility::Visible)
    }
    else { //tree <= largestTree
        let visibility = match current_visibility {
            Visibility::Unknown => Visibility::Hidden,
            Visibility::Visible => Visibility::Visible,
            Visibility::Hidden => Visibility::Hidden
        };
        (largest_tree, visibility)
    }
}

fn part_2() {
    let contents = fs::read_to_string("input")
        .expect("Error reading input file :(")
        .trim()
        .split("\n")
        .map(|row| row.chars().map(|char| {
            char.to_string()
                .parse::<i32>()
                .expect("Invalid num!")
        }).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    let mut scenic_scores: Vec<Vec<i32>> = vec![vec![0; contents[0].len()]; contents.len()];

    for y in 0..contents.len() {
        for x in 0..contents[0].len() {
            scenic_scores[y][x] = calculate_scenic_score(&contents, x, y);
        };
    }

    let max_scenic_score: i32 = scenic_scores.into_iter().map(|row| {
        row.into_iter().max().unwrap()
    }).max().unwrap();

    println!("{max_scenic_score}")
}

fn calculate_scenic_score(trees: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    let own_height = trees[y][x];

    let mut local_score = 0;
    for x in x + 1..trees[0].len() {
        local_score += 1;
        if trees[y][x] >= own_height {  
            break;
        }
    }
    let mut score = local_score;

    let mut local_score = 0;
    for x in (0..x).rev() {
        local_score += 1;
        if trees[y][x] >= own_height {  
            break;
        }
    }
    score *= local_score;

    let mut local_score = 0;
    for y in y + 1..trees.len() {
        local_score += 1;
        if trees[y][x] >= own_height {  
            break;
        }
    }
    score *= local_score;

    let mut local_score = 0;
    for y in (0..y).rev() {
        local_score += 1;
        if trees[y][x] >= own_height {  
            break;
        }
    }
    score *= local_score;
    
    score
}



fn main() {
    println!("Part 1:");
    part_1();
    println!("Part 2:");
    part_2();
}