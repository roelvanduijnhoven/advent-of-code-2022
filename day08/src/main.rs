use std::fs;

#[derive(Debug)]
struct HeightMap { 
    width: usize,
    height: usize,
    // Max is 9, so u8 is enough.
    cells: Vec<Vec<u8>>,
}

enum Directions {
    North, 
    East,
    South,
    West,
}

fn create_heightmap_from_string(input: &str) -> HeightMap {
    let cells = input
        .split("\n")
        .map(|line| {
            return line
                .chars()
                .map(|height| ((height as u32) - '0' as u32) as u8)
                .collect::<Vec<_>>();
        })
        .collect::<Vec<_>>();

    HeightMap { width: cells[0].len(), height: cells.len(), cells }
}

fn is_visible_from_edge(map: &HeightMap, start_x: isize, start_y: isize) -> bool {
    for direction in [Directions::North, Directions::East, Directions::South, Directions::West] {
        let (mut x, mut y) = (start_x, start_y);
        let height = map.cells[y as usize][x as usize];
        loop {
            (x, y) = move_position(x, y, &direction);

            // We could see the boundary!
            if x < 0 || x >= map.width as isize || y < 0 || y >= map.height as isize {
                return true;
            }

            if map.cells[y as usize][x as usize] >= height {
                break
            }
        }
    }

    return false;    
}

fn get_scenic_score(map: &HeightMap, start_x: isize, start_y: isize) -> usize {
    let mut scores: Vec<usize> = vec![];

    let height = map.cells[start_y as usize][start_x as usize];
    for direction in [Directions::North, Directions::East, Directions::South, Directions::West] {
        let mut direction_score = 0;
        let (mut x, mut y) = (start_x, start_y);
        loop {
            (x, y) = move_position(x, y, &direction);

            // We stumbled upon the boundary
            if x < 0 || x >= map.width as isize || y < 0 || y >= map.height as isize {
                break;
            }

            direction_score += 1;

            // Line of sight blocked
            if map.cells[y as usize][x as usize] >= height {
                break
            }
        }

        scores.push(direction_score);
    }

    return scores.iter().product();
}

fn move_position(x: isize, y: isize, direction: &Directions) -> (isize, isize) {
    match direction {
        Directions::North => (x, y - 1),
        Directions::East => (x + 1, y),
        Directions::South => (x, y + 1),
        Directions::West => (x - 1, y),
    }
}

fn main() {
    let content = fs::read_to_string("assets/problem.input").unwrap();
    let result = create_heightmap_from_string(&content);    
    
    let mut visible = 0;
    for y in 0..result.height {
        for x in 0..result.width {
            if is_visible_from_edge(&result, x as isize, y as isize) {
                visible += 1;
            }
        }
    }

    println!("{} trees visible from the edge.", visible);

    let mut highest_score = 0;
    for y in 0..result.height {
        for x in 0..result.width {
            let score = get_scenic_score(&result, x as isize, y as isize);
            highest_score = highest_score.max(score);
        }
    }

    println!("Highest scenic score is {}", highest_score);
}
