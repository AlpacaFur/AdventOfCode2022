use std::fs;

fn part_1() -> i32 {
    let input = 
        fs::read_to_string("input")
        .expect("Error reading input file :(");
    
    let mut instructions = input.trim().split("\n").map(str_to_instruction);

    let mut x = 1;
    let mut clock_cycle = 0;

    let mut signal_strengths: Vec<i32> = vec![];

    let mut current_instr: Instruction = Instruction::noop;
    let mut current_elapsed = 1;

    loop {
        if current_elapsed == instruction_duration(current_instr) {
            match current_instr {
                Instruction::noop => {},
                Instruction::addx(val) => {
                    x += val;
                }
            }
            match instructions.next() {
                None => {
                    break;
                }
                Some(instr) => {
                    current_instr = instr;
                }
            }
            current_elapsed = 0;
        };
        current_elapsed += 1;
        clock_cycle += 1;

        if is_notable_clock_cycle(clock_cycle) {
            signal_strengths.push(clock_cycle * x)
        }
    };

    signal_strengths.iter().sum()
}

fn is_notable_clock_cycle(clock_cycle: i32) -> bool {
    return clock_cycle >= 20 && ((clock_cycle - 20) % 40) == 0
}

fn instruction_duration(instr: Instruction) -> i32 {
    match instr {
        Instruction::noop => 1,
        Instruction::addx(_) => 2
    }
}

#[derive(Clone, Copy)]
enum Instruction {
    noop,
    addx(i32)
}

fn str_to_instruction(string: &str) -> Instruction {
    if string == "noop" {
        Instruction::noop
    } else {
        let num = string.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
        Instruction::addx(num)
    }
}


const CRT_WIDTH: usize = 40;
const CRT_HEIGHT: usize = 6;

fn part_2() -> String {
    let input = 
        fs::read_to_string("input")
        .expect("Error reading input file :(");
    
    let mut instructions = input.trim().split("\n").map(str_to_instruction);

    let mut x = 1;
    let mut clock_cycle: i32 = 1;


    let mut crt: Vec<Vec<char>> = vec![vec!['.';CRT_WIDTH]; CRT_HEIGHT];

    let mut current_instr: Instruction = instructions.next().unwrap();
    let mut current_elapsed = 0;

    loop {
        if current_elapsed == instruction_duration(current_instr) {
            match current_instr {
                Instruction::noop => {},
                Instruction::addx(val) => {
                    x += val;
                }
            }
            match instructions.next() {
                None => {
                    break;
                }
                Some(instr) => {
                    current_instr = instr;
                }
            }
            current_elapsed = 0;
        };

        let char = if (x - ((clock_cycle - 1) % 40)).abs() <= 1 {'#'} else {'.'};
        crt[(clock_cycle - 1) as usize / 40][(clock_cycle - 1) as usize % 40] = char;

        current_elapsed += 1;
        clock_cycle += 1;

        
    };

    crt.iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

fn main() {
    println!("Part 1");
    let part_1 = part_1();
    println!("{part_1}");

    println!("Part 2");
    let part_2 = part_2();
    println!("{part_2}");
}