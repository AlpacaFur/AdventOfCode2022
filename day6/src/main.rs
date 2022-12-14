use core::{panic};
use std::{fs};

fn alpha_to_bitmask(alpha: char) -> i32 {
    if !alpha.is_alphabetic() {
        panic!("Invalid alphabet char!");
    }
    1 << (alpha as i32 - 97)
}

fn part_1() -> i32 {
    let input = 
        fs::read_to_string("input")
        .expect("Error reading input file :(");
    
    let mut chars = input.trim().chars();
    let mut last_chars = (
        alpha_to_bitmask(chars.next().unwrap()),
        alpha_to_bitmask(chars.next().unwrap()),
        alpha_to_bitmask(chars.next().unwrap()),
        alpha_to_bitmask(chars.next().unwrap())
    );
    let mut index = 4;
    loop {
        if (last_chars.0 | last_chars.1 | last_chars.2 | last_chars.3).count_ones() == 4 {
            break;
        }
        match chars.next() {
            Some(char) => {
                last_chars = (
                    last_chars.1,
                    last_chars.2, 
                    last_chars.3, 
                    alpha_to_bitmask(char)
                )
            }
            None => panic!("Didn't find signal before end of string!")
        }
        index += 1;
    }
    
    
    index
}
fn part_2() -> i32 {
    let input = 
        fs::read_to_string("input")
        .expect("Error reading input file :(");
    
    let mut chars = input.trim().chars();
    let mut last_chars: Vec<i32> = vec![];
    for _ in 0..14 {
        last_chars.push(alpha_to_bitmask(chars.next().unwrap()))
    }
    let mut index = 14;
    loop {
        if last_chars.iter().fold(0, |total, current| {
            total | current
        }).count_ones() == 14 {
            break;
        }
        match chars.next() {
            Some(char) => {
                for i in 0..13 {
                    last_chars[i] = last_chars[i + 1]
                }
                last_chars[13] = alpha_to_bitmask(char);
            }
            None => panic!("Didn't find signal before end of string!")
        }
        index += 1;
    }
    
    
    index
}

fn main() {
    println!("Part 1");
    let part_1 = part_1();
    println!("{part_1}");

    println!("Part 2");
    let part_2 = part_2();
    println!("{part_2}");
}