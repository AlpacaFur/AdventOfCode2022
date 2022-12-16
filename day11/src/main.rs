use core::panic;
use std::{fs};

enum Operand {
    Old,
    Num(i64)
}

enum Operator {
    Times,
    Plus
}

struct Operation {
    left: Operand,
    operator: Operator,
    right: Operand
}

struct Monkey {
    starting_items: Vec<i64>,
    operation: Operation,
    check_divisible_by: i64,
    true_monkey: i64,
    false_monkey: i64
}

fn str_to_operand(operand: &str) -> Operand {
    match operand {
        "old" => Operand::Old,
        other => Operand::Num(other.parse::<i64>().unwrap())
    }
}

fn str_to_operator(operand: &str) -> Operator {
    match operand {
        "+" => Operator::Plus,
        "*" => Operator::Times,
        _ => panic!("Invalid operator")
    }
}

fn get_monkeys_from_input() -> Vec<Monkey> {
    fs::read_to_string("input")
        .unwrap()
        .split("\n\n")
        .map(get_monkey_from_input)
        .collect()
}

fn get_monkey_from_input(monkey: &str) -> Monkey {
    let lines = monkey.lines().map(|line| line.trim()).collect::<Vec<&str>>();
    let starting_items = lines[1][16..]
        .split(", ")
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let operation = {
        let mut operation = lines[2][17..].split(" ");
        let left = str_to_operand(operation.next().unwrap());
        let operator = str_to_operator(operation.next().unwrap());
        let right = str_to_operand(operation.next().unwrap());
        Operation {left, operator, right}
    };
    let check_divisible_by = lines[3][19..].parse::<i64>().unwrap();
    let true_monkey = lines[4][25..].parse::<i64>().unwrap();
    let false_monkey = lines[5][26..].parse::<i64>().unwrap();
    Monkey { 
        starting_items, 
        operation, 
        check_divisible_by,
        true_monkey, 
        false_monkey 
    }
}

fn perform_item_operation(item: i64, operation: &Operation) -> i64 {
    let left = match operation.left {
        Operand::Num(num) => num,
        Operand::Old => item
    };
    let right = match operation.right {
        Operand::Num(num) => num,
        Operand::Old => item
    };
    match operation.operator {
        Operator::Plus => left + right,
        Operator::Times => left * right,
    }
}

fn part_1() -> i32 {
    let monkeys = get_monkeys_from_input();
    let mut monkey_items: Vec<Vec<i64>> = monkeys.iter().map(|monkey| monkey.starting_items.clone()).collect();
    let mut monkey_counts = vec![0; monkeys.len()];

    const ROUNDS: i32 = 20;

    for _ in 0..ROUNDS {
        for monkey in 0..monkeys.len() {
            let items: Vec<i64> = monkey_items[monkey].drain(..).collect();
            items.iter().for_each(|&item| {
                let monkey_info = &monkeys[monkey];
                let new_worry = perform_item_operation(item, &monkey_info.operation);
                let new_worry = new_worry / 3;
                let target_monkey = if new_worry % monkey_info.check_divisible_by == 0 {
                    monkey_info.true_monkey
                } else {
                    monkey_info.false_monkey
                };
                monkey_items[target_monkey as usize].push(new_worry);
                monkey_counts[monkey] += 1;
            });
        }
    }

    monkey_counts.sort_by(|a, b| b.cmp(a));
    

    monkey_counts[0] * monkey_counts[1]
}

fn part_2() -> i64 {
    let monkeys = get_monkeys_from_input();
    let mut monkey_items: Vec<Vec<i64>> = monkeys.iter().map(|monkey| monkey.starting_items.clone()).collect();
    let mut monkey_counts: Vec<i64> = vec![0; monkeys.len()];

    const ROUNDS: i32 = 10000;

    let lcm = monkeys.iter().fold(1,|num: i64, monkey| {
        monkey.check_divisible_by * num
    });

    for _ in 0..ROUNDS {
        for monkey in 0..monkeys.len() {
            let items: Vec<i64> = monkey_items[monkey].drain(..).collect();
            items.iter().for_each(|&item| {
                let monkey_info = &monkeys[monkey];
                if item > lcm {
                    panic!("{}", lcm)
                }
                let new_worry = perform_item_operation(item, &monkey_info.operation);
                let target_monkey = if new_worry % monkey_info.check_divisible_by == 0 {
                    monkey_info.true_monkey
                } else {
                    monkey_info.false_monkey
                };
                monkey_items[target_monkey as usize].push(new_worry % lcm);
                monkey_counts[monkey] += 1;
            });
        }
    }

    monkey_counts.sort_by(|a, b| b.cmp(a));
    
    monkey_counts[0] * monkey_counts[1]
}

fn main() {
    println!("Part 1");
    let part_1 = part_1();
    println!("{part_1}");

    println!("Part 2");
    let part_2 = part_2();
    println!("{part_2}");
}