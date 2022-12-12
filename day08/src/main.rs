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

fn is_cell_visible(map: &HeightMap, start_x: isize, start_y: isize) -> bool {
    for direction in [Directions::North, Directions::East, Directions::South, Directions::West] {
        let (mut x, mut y) = (start_x, start_y);
        let height = map.cells[y as usize][x as usize];
        loop {
            (x, y) = match direction {
                Directions::North => (x, y - 1),
                Directions::East => (x + 1, y),
                Directions::South => (x, y + 1),
                Directions::West => (x - 1, y),
            };

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

fn main() {
    let content = fs::read_to_string("assets/problem.input").unwrap();
    let result = create_heightmap_from_string(&content);    
    
    let mut visible = 0;
    for y in 0..result.height {
        for x in 0..result.width {
            if is_cell_visible(&result, x as isize, y as isize) {
                visible += 1;
            }
        }
    }

    println!("There are {} trees visible", visible);
}
