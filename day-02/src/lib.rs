use std::str::FromStr;

#[derive(PartialEq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err("Not a known move".to_string()),
        }
    }
}

impl Move {
    fn get_points(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn beats(&self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
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

impl FromStr for GameResult {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Z" => Ok(GameResult::Win),
            "X" => Ok(GameResult::Lose),
            "Y" => Ok(GameResult::Draw),
            _ => Err("Not a known game result".to_string()),
        }
    }
}

impl GameResult {
    fn get_points(&self) -> u32 {
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

pub fn process_part1(input: &str) -> String {
    let line_scores = input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| {
            let moves: Vec<_> = line
                .split_whitespace()
                .map(|s| s.parse::<Move>().unwrap())
                .collect();

            let my_move = moves[1];
            let opponents_move = moves[0];

            let game_result = GameResult::from_moves(&my_move, &opponents_move);

            return my_move.get_points() + game_result.get_points();
        })
        .collect::<Vec<u32>>();

    line_scores.iter().sum::<u32>().to_string()
}

pub fn process_part2(input: &str) -> String {
    let line_scores = input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| {
            let moves: Vec<_> = line.split_whitespace().collect();

            let opponents_move = moves[0].parse::<Move>().unwrap();
            let wanted_result = moves[1].parse::<GameResult>().unwrap();
            let my_move = Move::should_play(&opponents_move, &wanted_result);

            let game_result = GameResult::from_moves(&my_move, &opponents_move);

            return my_move.get_points() + game_result.get_points();
        })
        .collect::<Vec<u32>>();

    line_scores.iter().sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "15");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "12");
    }
}
