use std::fs;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn main() {
    let cheat_sheet = fs::read_to_string("input/part-1.input").unwrap();
    part1(&cheat_sheet.clone());
    part2(&cheat_sheet.clone());
}

fn part1(cheat_sheet: &str) {
    let rounds = cheat_sheet.split("\n");

    let mut total_score = 0;
    for round in rounds {
        let mut parts = round.split(" ");
        let opponent = parts.next().unwrap();
        let me = parts.next().unwrap();

        let opponent_move = match opponent {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("Unknown move"),
        };
        let me_move = match me {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => panic!("Unknown move"),
        };

        let score = score_for_game(me_move.clone(), opponent_move.clone());

        println!("Opponents plays {:?}, I am told to play {:?}. Scored {:?}", opponent_move, me_move, score);

        total_score += score;
    }

    println!("Scored a total of {:?} points.", total_score);
}

fn part2(cheat_sheet: &str) {
    let rounds = cheat_sheet.split("\n");

    let mut total_score = 0;
    for round in rounds {
        let mut parts = round.split(" ");
        let opponent = parts.next().unwrap();
        let me = parts.next().unwrap();

        let opponent_move = match opponent {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("Unknown move"),
        };
        let me_move = match me {
            "X" => match opponent_move {
                Move::Paper => Move::Rock,
                Move::Rock => Move::Scissors,
                Move::Scissors => Move::Paper
            },
            "Y" => opponent_move.clone(),
            "Z" => match opponent_move {
                Move::Paper => Move::Scissors,
                Move::Rock => Move::Paper,
                Move::Scissors => Move::Rock
            }
            _ => panic!("Unknown move"),
        };

        let score = score_for_game(me_move.clone(), opponent_move.clone());

        println!("Opponents plays {:?}, I am told to play {:?}. Scored {:?}", opponent_move, me_move, score);

        total_score += score;
    }

    println!("Scored a total of {:?} points.", total_score);
}

fn score_for_game(me: Move, opponent: Move) -> i32 {
    const SCORE_FOR_WINNING: i32 = 6;
    const SCORE_FOR_DRAW: i32 = 3;
    const SCORE_FOR_LOSING: i32 = 0;

    let win_score = match me {
        Move::Rock if opponent == Move::Scissors => SCORE_FOR_WINNING,
        Move::Paper if opponent == Move::Rock => SCORE_FOR_WINNING,
        Move::Scissors if opponent == Move::Paper => SCORE_FOR_WINNING,

        Move::Scissors if opponent == Move::Rock => SCORE_FOR_LOSING,
        Move::Rock if opponent == Move::Paper => SCORE_FOR_LOSING,
        Move::Paper if opponent == Move::Scissors => SCORE_FOR_LOSING,

        _ => SCORE_FOR_DRAW,
    };

    let move_score = match me {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    };

    win_score + move_score
}
