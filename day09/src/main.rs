use std::{fs, collections::HashSet};

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    amount: usize,
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn read_instructions_from_string(input: &str) -> Vec<Instruction> {
    return input
        .split("\n")
        .map(|line| {
            let parts: Vec<_> = line.split(" ").collect();
            let direction = match parts[0] {
                "U" => Direction::Up,
                "R" => Direction::Right,
                "D" => Direction::Down,
                "L" => Direction::Left,
                _ => panic!("Invalid input"),
            };
            let amount = parts[1].parse::<usize>().unwrap();
            return Instruction { direction, amount };
        })

        .collect();
}

fn move_head(x: isize, y: isize, direction: &Direction) -> (isize, isize) {
    match direction {
        Direction::Up => (x, y + 1),
        Direction::Right => (x + 1, y),
        Direction::Down => (x, y - 1),
        Direction::Left => (x - 1, y),
    }
}

fn move_tail(tail_x: isize, tail_y: isize, head_x: isize, head_y: isize) -> (isize, isize) {
    let mut new_tail_x = tail_x;
    let mut new_tail_y = tail_y;
    
    let x_diff = head_x - tail_x;
    let y_diff = head_y - tail_y;
    
    let x_step = if x_diff <= -1 { -1 } else if x_diff >= 1 { 1 } else { 0 };
    let y_step = if y_diff <= -1 { -1 } else if y_diff >= 1 { 1 } else { 0 };

    if x_diff.abs() > 1 && y_diff == 0 {
        new_tail_x += x_step;
    } else if y_diff.abs() > 1 && x_diff == 0 {
        new_tail_y += y_step;
    } else if x_diff.abs() > 1 || y_diff.abs() > 1 {
        new_tail_x += x_step;
        new_tail_y += y_step;
    }

    (new_tail_x, new_tail_y)
}

fn main() {    
    let input = fs::read_to_string("assets/puzzle.input").unwrap();
    let instructions = read_instructions_from_string(&input);

    println!("{:?}", instructions);

    let mut head_x = 0;
    let mut head_y = 0;

    let mut tail_x= 0;
    let mut tail_y = 0;

    let mut seen: HashSet<(isize, isize)> = HashSet::new();
    for instruction in instructions.iter() {
        for _ in 0..instruction.amount {
            seen.insert((tail_x, tail_y));

            (head_x, head_y) = move_head(head_x, head_y, &instruction.direction);
            (tail_x, tail_y) = move_tail(tail_x, tail_y, head_x, head_y);
        }
    }

    println!("Tail visited {} unique positions on grid.", seen.len());
}
