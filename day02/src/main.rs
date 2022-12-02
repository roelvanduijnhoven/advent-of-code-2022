use std::fs;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

struct Round {
    opponent_move: Move,
    strategy: char,
}

fn strategy_from_line(line: &str) -> Round {
    let chars: Vec<char> = line.chars().collect();
    Round {
        opponent_move: match *chars.get(0).unwrap() {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scissors,
            _ => panic!("Unknown move")
        },
        strategy: *chars.get(2).unwrap()
    }
}

fn main() {
    let contents = fs::read_to_string("input/part-1.input").unwrap();
    let cheat_sheet: Vec<Round> = contents
        .split("\n")
        .map(|line| strategy_from_line(line))
        .collect();

    println!("In part 1, scored a total of {:?} points.", get_score_according_to_part_1_rules(&cheat_sheet));
    println!("In part 2, scored a total of {:?} points.", get_score_according_to_part_2_rules(&cheat_sheet));
}

fn get_score_according_to_part_1_rules(cheat_sheet: &Vec<Round>) -> i32 {
    cheat_sheet
        .iter()
        .map(|round| {
            let me_move = match round.strategy {
                'X' => Move::Rock,
                'Y' => Move::Paper,
                'Z' => Move::Scissors,
                _ => panic!("Unknown move")
            };
    
            score_for_game(&me_move, &round.opponent_move)
        })
        .sum()
}

fn get_score_according_to_part_2_rules(cheat_sheet: &Vec<Round>) -> i32 {
    cheat_sheet
        .iter()
        .map(|round| {
            let me_move = match round.strategy {
                // We are told to lose
                'X' => match round.opponent_move {
                    Move::Paper => Move::Rock,
                    Move::Rock => Move::Scissors,
                    Move::Scissors => Move::Paper
                },

                // We are told to play a draw
                'Y' => round.opponent_move.clone(),

                // We are told to lose
                'Z' => match round.opponent_move {
                    Move::Paper => Move::Scissors,
                    Move::Rock => Move::Paper,
                    Move::Scissors => Move::Rock
                }
                _ => panic!("Unknown move"),
            };

            score_for_game(&me_move, &round.opponent_move)
        })
        .sum()
}

fn score_for_game(me: &Move, opponent: &Move) -> i32 {
    const SCORE_FOR_WINNING: i32 = 6;
    const SCORE_FOR_DRAW: i32 = 3;
    const SCORE_FOR_LOSING: i32 = 0;

    let win_score = match me {
        Move::Rock if opponent == &Move::Scissors => SCORE_FOR_WINNING,
        Move::Paper if opponent == &Move::Rock => SCORE_FOR_WINNING,
        Move::Scissors if opponent == &Move::Paper => SCORE_FOR_WINNING,

        Move::Scissors if opponent == &Move::Rock => SCORE_FOR_LOSING,
        Move::Rock if opponent == &Move::Paper => SCORE_FOR_LOSING,
        Move::Paper if opponent == &Move::Scissors => SCORE_FOR_LOSING,

        _ => SCORE_FOR_DRAW,
    };

    let move_score = match me {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    };

    win_score + move_score
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_outcome(){
        assert_eq!(7, score_for_game(Move::Rock, Move::Scissors));
    }
}