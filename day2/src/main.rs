use std::fs;
use std::collections::HashMap;

#[derive(Eq, PartialEq)]
enum HandSign {
    Rock,
    Paper,
    Scissors
}

const LOSE_POINTS: i32 = 0;
const TIE_POINTS: i32 = 3;
const WIN_POINTS: i32 = 6;

fn match_points(opponent: &HandSign, you: &HandSign) -> i32 {
    if opponent == you {
        return TIE_POINTS;
    } else if get_corresponding_sign(GameStatus::Win, opponent) == you {
        return WIN_POINTS;
    } else {
        return LOSE_POINTS;
    }
}

fn sign_score(sign: &HandSign) -> i32 {
    match sign {
        HandSign::Rock => 1,
        HandSign::Paper => 2,
        HandSign::Scissors => 3,
    }
}

enum GameStatus {
    Win,
    Tie,
    Lose
}

fn get_corresponding_sign(target_status: GameStatus, opponent: &HandSign) -> &HandSign {
    match target_status {
        GameStatus::Win => match opponent {
            HandSign::Rock => &HandSign::Paper,
            HandSign::Paper => &HandSign::Scissors,
            HandSign::Scissors => &HandSign::Rock
        },
        GameStatus::Tie => opponent,
        GameStatus::Lose => match opponent {
            HandSign::Rock => &HandSign::Scissors,
            HandSign::Paper => &HandSign::Rock,
            HandSign::Scissors => &HandSign::Paper
        },
    }
}

fn main() {
    let mut guide: HashMap<&str, HandSign> = HashMap::new();
    guide.insert("A", HandSign::Rock);
    guide.insert("B", HandSign::Paper);
    guide.insert("C", HandSign::Scissors);

    guide.insert("X", HandSign::Rock);
    guide.insert("Y", HandSign::Paper);
    guide.insert("Z", HandSign::Scissors);

    let contents = fs::read_to_string("input")
        .expect("Error reading input file :(");

    let mut total_score = 0;

    for line in contents.split("\n") {
        let sides: Vec<&str> = line.split(" ").take(2).collect();
        
        let opponent = guide.get(sides[0]).expect("Expected a valid letter!");
        let you = guide.get(sides[1]).expect("Expected a valid letter!");


        total_score += sign_score(you) + match_points(opponent, you);
    }

    println!("Part 1:");
    println!("{total_score}");



    let mut total_score_2 = 0;

    for line in contents.split("\n") {
        let sides: Vec<&str> = line.split(" ").take(2).collect();
        
        let target_status: GameStatus = match sides[1] {
            "X" => GameStatus::Lose,
            "Y" => GameStatus::Tie,
            "Z" => GameStatus::Win,
            _ => panic!("Invalid game option!")
        };

        let opponent = guide.get(sides[0]).expect("Expected a valid letter!");
        let you = get_corresponding_sign(target_status, opponent);


        total_score_2 += sign_score(you) + match_points(opponent, you);
    }

    println!("Part 2:");
    println!("{total_score_2}");

    
    
}
