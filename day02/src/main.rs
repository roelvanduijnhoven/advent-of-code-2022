use std::fs;

#[derive(Debug, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn main() {
    let contents = fs::read_to_string("input/part-1.input").unwrap();
    let rounds = contents.split("\n");

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

        const SCORE_FOR_WINNING: i32 = 6;
        const SCORE_FOR_DRAW: i32 = 3;
        const SCORE_FOR_LOSING: i32 = 0;
        let win_score = match me_move {
            Move::Rock if opponent_move == Move::Scissors => SCORE_FOR_WINNING,
            Move::Paper if opponent_move == Move::Rock => SCORE_FOR_WINNING,
            Move::Scissors if opponent_move == Move::Paper => SCORE_FOR_WINNING,

            Move::Scissors if opponent_move == Move::Rock => SCORE_FOR_LOSING,
            Move::Rock if opponent_move == Move::Paper => SCORE_FOR_LOSING,
            Move::Paper if opponent_move == Move::Scissors => SCORE_FOR_LOSING,

            _ => SCORE_FOR_DRAW,
        };

        let move_score = match me_move {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };

        let score = win_score + move_score;

        println!("Opponents plays {:?}, I am told to play {:?}. Scored {:?}", opponent_move, me_move, score);

        total_score += score;
    }

    println!("Scored a total of {:?} points.", total_score);
}
