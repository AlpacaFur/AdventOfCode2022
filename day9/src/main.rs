use core::panic;
use std::{fs, collections::{HashSet}};

fn part_1() -> usize {
    let input = 
        fs::read_to_string("input")
        .expect("Error reading input file :(");

    let instructions = input
        .trim()
        .split("\n")
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .map(line_to_instruction);

    let mut head = (0,0);
    let mut tail = (0,0);

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(tail);

    instructions.for_each(|instruction| {
        for _ in 0..instruction.distance {
            head = move_tuple(head, &instruction.direction);
            tail = move_tail(head, tail);
            visited.insert(tail);
        }
    });

    visited.len()
}

fn move_tuple(tuple: (i32, i32), direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (tuple.0, tuple.1 + 1),
        Direction::Down => (tuple.0, tuple.1 - 1),
        Direction::Left => (tuple.0 - 1, tuple.1),
        Direction::Right => (tuple.0 + 1, tuple.1)
    }
}

fn move_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    if tail.0 == head.0 && tail.1 == head.1 {
        (tail.0, tail.1)
    } else if (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1 {
        (tail.0, tail.1)
    } else {
        (tail.0 + (head.0 - tail.0).signum(),
        tail.1 + (head.1 - tail.1).signum())
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct Instruction {
    direction: Direction,
    distance: i32
}

fn line_to_instruction(line: Vec<&str>) -> Instruction {
    Instruction { 
        direction: match line[0] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid Direction!")
        },
        distance: line[1].parse().unwrap()
    }
}

fn part_2() -> usize {
    let input = 
        fs::read_to_string("input")
        .expect("Error reading input file :(");

    let instructions = input
        .trim()
        .split("\n")
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .map(line_to_instruction);

    let start_tuple: (i32, i32) = (0, 0);
    let mut rope: Vec<(i32, i32)> = vec![start_tuple; 10];

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(rope[rope.len() - 1]);

    instructions.for_each(|instruction| {
        for _ in 0..instruction.distance {
            rope[0] = move_tuple(rope[0], &instruction.direction);
            for knot in 1..rope.len() {
                rope[knot] = move_tail(rope[knot - 1], rope[knot]);
            }
            visited.insert(rope[9]);
            
        }
    });

    visited.len()
}

fn main() {
    println!("Part 1");
    let part_1 = part_1();
    println!("{part_1}");

    println!("Part 2");
    let part_2 = part_2();
    println!("{part_2}");
}