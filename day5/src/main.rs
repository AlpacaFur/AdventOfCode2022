use std::fs;

fn part_1() -> String {
    let input = 
        fs::read_to_string("input")
        .expect("Error reading input file :(");
    
    let input = input.split("\n\n").collect::<Vec<&str>>();

    let mut towersInput = input[0].split("\n").collect::<Vec<&str>>();
    towersInput.pop();
    let mut towers: Vec<Vec<char>> = vec![vec![];(towersInput[0].len()/4 + 1)];
    for row in (0..towersInput.len()).rev() {
        let row = towersInput[row].chars().collect::<Vec<char>>();
        for tower in 0..(row.len()/4 + 1) {
            let char = row[(tower*4) + 1];
            if char != ' ' {
                towers[tower].push(char)
            }
        }
    }

    input[1].trim().split("\n").for_each(|row| {
        let segments = row.split(" ").collect::<Vec<&str>>();
        let quantity: usize = segments[1].parse::<usize>().unwrap();
        let from: usize = segments[3].parse::<usize>().unwrap() - 1;
        let to: usize = segments[5].parse::<usize>().unwrap() - 1;

        for _ in 0..quantity {
            let container = towers[from].pop().unwrap();
            towers[to].push(container);
        }
    });

    towers.iter().map(|tower| tower.last().unwrap()).collect::<String>()
}

fn part_2() -> String {
    let input = 
        fs::read_to_string("input")
        .expect("Error reading input file :(");
    
    let input = input.split("\n\n").collect::<Vec<&str>>();

    let mut towersInput = input[0].split("\n").collect::<Vec<&str>>();
    towersInput.pop();
    let mut towers: Vec<Vec<char>> = vec![vec![];(towersInput[0].len()/4 + 1)];
    for row in (0..towersInput.len()).rev() {
        let row = towersInput[row].chars().collect::<Vec<char>>();
        for tower in 0..(row.len()/4 + 1) {
            let char = row[(tower*4) + 1];
            if char != ' ' {
                towers[tower].push(char)
            }
        }
    }

    input[1].trim().split("\n").for_each(|row| {
        let segments = row.split(" ").collect::<Vec<&str>>();
        let quantity: usize = segments[1].parse::<usize>().unwrap();
        let from: usize = segments[3].parse::<usize>().unwrap() - 1;
        let to: usize = segments[5].parse::<usize>().unwrap() - 1;

        let mut in_crane: Vec<char> = vec![];
        for _ in 0..quantity {
            in_crane.push(towers[from].pop().unwrap());
        }
        in_crane.into_iter().rev().for_each(|item| {
            towers[to].push(item);
        });
    });

    towers.iter().map(|tower| tower.last().unwrap()).collect::<String>()
}

fn main() {
    println!("Part 1");
    let part_1 = part_1();
    println!("{part_1}");

    println!("Part 2");
    let part_2 = part_2();
    println!("{part_2}");
}