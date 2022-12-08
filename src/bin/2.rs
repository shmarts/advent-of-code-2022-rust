use std::fs;

#[derive(PartialEq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from_str(input: &str) -> Move {
        match input {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => panic!("Invalid move"),
        }
    }

    fn beats(&self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    fn get_value(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn should_play(opponents_move: &Move, wanted_result: &GameResult) -> Move {
        match wanted_result {
            GameResult::Win => opponents_move.beats().beats(),
            GameResult::Lose => opponents_move.beats(),
            GameResult::Draw => *opponents_move,
        }
    }
}

enum GameResult {
    Win,
    Lose,
    Draw,
}

impl GameResult {
    fn from_str(input: &str) -> GameResult {
        match input {
            "Z" => GameResult::Win,
            "X" => GameResult::Lose,
            "Y" => GameResult::Draw,
            _ => panic!("Invalid game result"),
        }
    }

    fn get_value(&self) -> u32 {
        match self {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Lose => 0,
        }
    }

    fn from_moves(my_move: &Move, opponents_move: &Move) -> GameResult {
        if my_move.beats() == *opponents_move {
            GameResult::Win
        } else if opponents_move.beats() == *my_move {
            GameResult::Lose
        } else {
            GameResult::Draw
        }
    }
}

fn main() {
    let input = fs::read_to_string("src/input/2.txt").unwrap();

    let lines = input.split("\n").collect::<Vec<&str>>();

    let line_scores = lines
        .iter()
        .map(|line| {
            let moves: Vec<_> = line.split_whitespace().collect();

            let opponents_move = Move::from_str(moves[0]);
            let wanted_result = GameResult::from_str(moves[1]);
            let my_move = Move::should_play(&opponents_move, &wanted_result);

            // get my_move's value
            let my_move_value = my_move.get_value();

            // get game result value
            let result = GameResult::from_moves(&my_move, &opponents_move);
            let result_value = result.get_value();

            my_move_value + result_value
        })
        .collect::<Vec<u32>>();

    let total_score = line_scores.iter().sum::<u32>();

    println!("total score: {:?}", total_score);
}
