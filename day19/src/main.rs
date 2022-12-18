use std::fs;

const N: usize = 70;
type Grid = [[[bool; N]; N]; N];

// Note: we'll move the whole thing by (+1, +1, +1). This makes it easier to look around neighboring cells.
fn create_grid_from_string(input: &str) -> Grid {
    let mut grid = [[[false; N]; N]; N];
    
    for line in input.split("\n") {
        let parts: Vec<_> = line
            .split(",")
            .map(|part| part.parse::<usize>().unwrap())
            .collect();

        grid[parts[0] + 1][parts[1] + 1][parts[2] + 1] = true;
    }

    return grid;
}

fn count_sides(grid: &Grid) -> usize {
    let mut count = 0;
    for x in 1..N-1 {
        for y in 1..N-1 {
            for z in 1..N-1 {
                if !grid[x][y][z] {
                    continue;
                }

                if !grid[x+1][y][z] {
                    count += 1;
                }
                if !grid[x-1][y][z] {
                    count += 1;
                }
                if !grid[x][y+1][z] {
                    count += 1;
                }
                if !grid[x][y-1][z] {
                    count += 1;
                }    
                if !grid[x][y][z+1] {
                    count += 1;
                }
                if !grid[x][y][z-1] {
                    count += 1;
                }            
            }
        }
    }

    return count;
}

fn main() {
    let input = fs::read_to_string("assets/puzzle.input").unwrap();

    let grid = create_grid_from_string(&input);

    println!("{}", count_sides(&grid));
}
