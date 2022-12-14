use std::{fs, collections::HashMap};

#[derive(Debug, Eq, Hash, PartialEq)]
struct Cell { x: usize, y: usize, height: u8 }

#[derive(Debug)]
struct Heightmap {
    width: usize,
    height: usize,

    // Height runs from 0 to 25
    // Cells is a 2D structure of width x height
    cells: Vec<Vec<Cell>>,
}

fn read_puzzle_input(input: &str) -> (Heightmap, Cell, Cell) {
    let mut start = Cell { x: 0, y: 0, height: 0 };
    let mut end = Cell { x: 0, y: 0, height: 0 };

    let mut cells: Vec<_> = vec![];
    for (y, line) in input.split("\n").enumerate() {
        let mut row: Vec<_> = vec![];
        for (x, char) in line.chars().enumerate() {
            let height = match char {
                'S' => 0,
                'E' => 25,
                char => ((char as u32) - 'a' as u32) as u8
            };
            row.push(Cell { x, y, height });

            if char == 'S' {
                start.x = x;
                start.y = y;
            } else if char == 'E' {
                end.x = x;
                end.y = y;
            }
        }
        cells.push(row);
    }

    let width = cells[0].len();
    let height = cells.len();

    (Heightmap { cells, width, height }, start, end)
}

// TODO No idea what this lifetime is doing. And what function does it serve?
fn get_neighbors_of<'a>(heightmap: &'a Heightmap, cell: &'a Cell) -> Vec<&'a Cell> {
    let mut neighbors = vec![];

    if cell.y > 0 {
        neighbors.push(&heightmap.cells[cell.y - 1][cell.x]);
    }
    if cell.y < heightmap.height - 1 {
        neighbors.push(&heightmap.cells[cell.y + 1][cell.x]);
    }
    if cell.x > 0 {
        neighbors.push(&heightmap.cells[cell.y][cell.x - 1]);
    }
    if cell.x < heightmap.width - 1 {
        neighbors.push(&heightmap.cells[cell.y][cell.x + 1]);
    }    

    return neighbors
        .into_iter()
        .filter(|neighbor| neighbor.height <= cell.height + 1)
        .collect();
}

fn find_shortest_path_from(heightmap: &Heightmap, from_x: usize, from_y: usize) -> HashMap<&Cell, Option<&Cell>> {
    let mut unexplored: Vec<&Cell> = vec![];
    let mut distances: HashMap<&Cell, Option<usize>> = HashMap::new();
    let mut parent: HashMap<&Cell, Option<&Cell>> = HashMap::new();

    for row in &heightmap.cells {
        for cell in row {
            unexplored.push(cell);
            distances.insert(cell, None);
            parent.insert(cell, None);
        }
    }

    distances.insert(&heightmap.cells[from_y][from_x], Some(0)); 
    while unexplored.len() > 0 {
        // TODO Change so we don't need to sort every time.
        let mut candidates: Vec<_> = unexplored
            .iter()
            .map(|item| (item, distances[item]))
            .filter(|(_, distance)| !distance.is_none())
            .collect();
        candidates.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

        // There was no longer an unexplored cell that was reachable. So we looked at all options, and can safely terminate.
        if candidates.len() == 0 {
            break;
        }

        let current = *candidates[0].0;

        let position = unexplored.iter().position(|&item| item == current).unwrap();
        unexplored.remove(position);

        for neighbor in get_neighbors_of(heightmap, current) {
            let new_potential_distance = distances[current].unwrap() + 1;
            let replaces_best = match distances[neighbor] {
                None => true,
                Some(distance) => new_potential_distance < distance
            };

            if replaces_best {
                distances.insert(neighbor, Some(new_potential_distance));
                parent.insert(neighbor, Some(current));
            }
        }
    }

    return parent;
}

fn find_path<'a>(heightmap: &'a Heightmap, parent: &'a HashMap<&Cell, Option<&'a Cell>>, from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> Option<Vec<&'a Cell>> {
    let start = &heightmap.cells[from_y][from_x];
    let mut current = &heightmap.cells[to_y][to_x];
    let mut path: Vec<&Cell> = vec![];
    loop {
        path.push(&current);
        current = match parent[current] {
            None => return None,
            Some(current) => current,
        };

        if current == start {
            break;
        }
    }
    
    return Some(path);
}

fn print_ascii_art_path(heightmap: &Heightmap, path: Vec<&Cell>) {
    for y in 0..heightmap.height {
        for x in 0..heightmap.width {
            if path.contains(&&heightmap.cells[y][x]) {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }
}

fn main() {
    let input = fs::read_to_string("assets/puzzle.input").unwrap();
    let (mut heightmap, start, to) = read_puzzle_input(&input);

    // Part 1
    let parents = find_shortest_path_from(&heightmap, start.x, start.y);
    let path = find_path(&heightmap, &parents, start.x, start.y, to.x, to.y).unwrap();

    println!("Found a path from ({}, {}) to ({}, {}) of length {}.", start.x, start.y, to.x, to.y, path.len());
    println!("Path shown below:");
    print_ascii_art_path(&heightmap, path);

    // Part 2
    // Let's invert the heights. Afterwards we can solve this using the shortest_path algorithm with the end as the start.
    for y in 0..heightmap.height {
        for x in 0..heightmap.width {
            heightmap.cells[y][x].height = 25 as u8 - heightmap.cells[y][x].height;
        }
    }

    let parents = find_shortest_path_from(&heightmap, to.x, to.y);

    let mut shortest_path: Option<Vec<&Cell>> = None;
    for y in 0..heightmap.height {
        for x in 0..heightmap.width {
            if heightmap.cells[y][x].height != 25 {
                continue;
            }

            let path = find_path(&heightmap, &parents, to.x, to.y, x, y);
            if path.is_none() {
                continue;
            }

            let path = path.unwrap();
            shortest_path = match shortest_path {
                Some(current) if current.len() < path.len() => Some(current),
                _ => Some(path),
            };
        }
    }    

    let shortest_path = shortest_path.unwrap();

    println!();
    println!("Best start location at elevation a is of length {}", shortest_path.len());
    println!("Path shown below:");
    print_ascii_art_path(&heightmap, shortest_path);
}