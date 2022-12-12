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

struct Point {
    x: isize, 
    y: isize,
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

fn move_head(p: &mut Point, direction: &Direction) {
    match direction {
        Direction::Up => p.y += 1,
        Direction::Right => p.x += 1,
        Direction::Down => p.y -= 1,
        Direction::Left => p.x -= 1,
    };
}

fn move_tail(tail: &mut Point, head: &Point) {
    let x_diff = head.x - tail.x;
    let y_diff = head.y - tail.y;
    
    let x_step = if x_diff <= -1 { -1 } else if x_diff >= 1 { 1 } else { 0 };
    let y_step = if y_diff <= -1 { -1 } else if y_diff >= 1 { 1 } else { 0 };

    if x_diff.abs() > 1 && y_diff == 0 {
        tail.x += x_step;
    } else if y_diff.abs() > 1 && x_diff == 0 {
        tail.y += y_step;
    } else if x_diff.abs() > 1 || y_diff.abs() > 1 {
        tail.x += x_step;
        tail.y += y_step;
    }
}

fn main() {    
    let input = fs::read_to_string("assets/puzzle.input").unwrap();
    let instructions = read_instructions_from_string(&input);

    // Part 1
    let mut head = Point { x: 0, y: 0 };
    let mut tail= Point { x: 0, y: 0 };

    let mut seen: HashSet<(isize, isize)> = HashSet::new();
    for instruction in instructions.iter() {
        for _ in 0..instruction.amount {
            move_head(&mut head, &instruction.direction);
            move_tail(&mut tail, &head);
            
            seen.insert((tail.x, tail.y));
        }
    }

    println!("Tail visited {} unique positions on grid.", seen.len());

    // Part 2
    let chain_length = 10;
    let mut chain: Vec<Point> = vec![];
    for _ in 0..chain_length {
        chain.push(Point { x: 0, y: 0 });
    }

    let mut seen: HashSet<(isize, isize)> = HashSet::new();
    for instruction in instructions.iter() {
        for _ in 0..instruction.amount {
            // Apply move to first in chain
            let mut head = chain.get_mut(0).unwrap();
            move_head(&mut head, &instruction.direction);

            // Now move rest of tail
            for i in 1..chain_length {
                let head = &chain[(i - 1) as usize];
                let head = Point { x: head.x, y: head.y }; // TODO Necessary to let this compile.
                let mut tail = chain.get_mut(i).unwrap();
                move_tail(&mut tail, &head);
            }

            // Keep track of last item of chain
            seen.insert((chain[chain_length - 1].x, chain[chain_length - 1].y));            
        }
    }

    println!("Last item of rope visited {} unique positions on grid.", seen.len());

}
