use itertools::Itertools;
use std::{fs, collections::{HashSet, HashMap}};

fn chars_to_hashmap(chars: &str) -> HashMap<char, i32> {
    let mut hashmap = HashMap::new();
    let mut index = 1;
    chars.chars().for_each(|char| {
        hashmap.insert(char, index);
        index += 1
    });

    hashmap
}

fn in_common(group1: &str, group2: &str) -> char {
    let mut in_first_half: HashSet<char> = HashSet::new();

    group1.chars().for_each(|char| {
        in_first_half.insert(char);
    });
    
    let mut odd_type_out: char = 'c';
    for char in group2.chars() {
        if in_first_half.contains(&char) {
            odd_type_out = char;
            break;
        }
    };

    odd_type_out
}

fn in_common_triple(group1: &str, group2: &str, group3: &str) -> char {
    let mut in_first_half: HashMap<char, i32> = HashMap::new();

    group1.chars().for_each(|char| {
        in_first_half.insert(char, 1);
    });

    group2.chars().for_each(|char| {
        if in_first_half.contains_key(&char) {
            in_first_half.insert(char, 2);
        }
    });
    
    let mut odd_type_out: char = 'c';
    for char in group3.chars() {
        match in_first_half.get(&char) {
            Some(num_appearances) => {
                if num_appearances == &2i32 {
                    odd_type_out = char;
                    break;
                }
            },
            None => {}
        };
    };

    odd_type_out
}

fn get_input() -> String {
    fs::read_to_string("input")
        .expect("Error reading input file :(").trim().to_owned()
}

fn part1() -> i32 {
    let charmap = chars_to_hashmap("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");

    get_input().split("\n").into_iter().map(|line| {
        
        let first_half = &line[0..line.len()/2];
        let second_half = &line[line.len()/2..line.len()];

        in_common(first_half, second_half)

    }).map(|char| {
        charmap.get(&char).expect("Invalid Letter")
    }).sum()
}

fn part2() -> i32 {
    let charmap = chars_to_hashmap("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");

    // I was initially trying to use array_chunks, but I realized that
    // it's not actually stable yet ( :( ) so I used itertools' chunks function
    // instead.

    get_input().split("\n").into_iter().chunks(3).into_iter().map(|lines| {
        let lines: Vec<&str> = lines.collect();
        
        let group1 = lines[0];
        let group2 = lines[1];
        let group3 = lines[2];

        in_common_triple(group1, group2, group3)

    }).map(|char| {
        charmap.get(&char).expect("Invalid Letter")
    }).sum()
}

fn main() {
    println!("Part 1:");
    let part1_ans = part1(); 
    println!("{part1_ans}");
    let part2_ans = part2();
    println!("{part2_ans}");
}
