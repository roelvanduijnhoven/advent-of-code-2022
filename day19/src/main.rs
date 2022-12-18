use std::fs;

const N: usize = 100;
type Grid = [[[bool; N]; N]; N];

// Note: we'll move the whole thing by (1, 1, 1). This makes it easier to look around neighboring cells.
fn create_grid_from_string(input: &str) -> Grid {
    let mut grid = [[[false; N]; N]; N];
    
    for line in input.split("\n") {
        let parts: Vec<_> = line
            .split(",")
            .map(|part| part.parse::<usize>().unwrap())
            .collect();

        grid[parts[0] + 3][parts[1] + 3][parts[2] + 3] = true;
    }

    return grid;
}

fn fill_gaps_in_grid(grid: &Grid) -> Grid {
    let mut outside = [[[false; N]; N]; N];
    
    for x in 1..N-1 {
        for y in 1..N-1 {
            for z in 1..N-1 {
                if grid[x][y][z] {
                    continue;
                }

                let cell_outside = 
                    (x == 1 || y == 1 || z == 1)
                 || outside[x+1][y][z] 
                 || outside[x-1][y][z]
                 || outside[x][y+1][z] 
                 || outside[x][y-1][z] 
                 || outside[x][y][z+1] 
                 || outside[x][y][z-1];

                if cell_outside {
                    outside[x][y][z] = true;
                }
            }
        }
    }

    for x in (1..N-1).rev() {
        for y in (1..N-1).rev() { 
            for z in (1..N-1).rev() {
                if grid[x][y][z] {
                    continue;
                }

                let cell_outside = 
                    (x == 1 || y == 1 || z == 1)
                 || outside[x+1][y][z] 
                 || outside[x-1][y][z]
                 || outside[x][y+1][z] 
                 || outside[x][y-1][z] 
                 || outside[x][y][z+1] 
                 || outside[x][y][z-1];

                if cell_outside {
                    outside[x][y][z] = true;
                }
            }
        }
    }

    let mut filled_grid = [[[false; N]; N]; N];
    for x in 1..N-1 {
        for y in 1..N-1 {
            for z in 1..N-1 {            
                let reachable = 
                    outside[x+1][y][z] 
                 || outside[x-1][y][z]
                 || outside[x][y+1][z] 
                 || outside[x][y-1][z] 
                 || outside[x][y][z+1] 
                 || outside[x][y][z-1];

                filled_grid[x][y][z] = grid[x][y][z] || !reachable;
            }
        }
    }

    return filled_grid;
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

    let filled_grid = fill_gaps_in_grid(&grid);
    println!("Number of sides, after filling the wholes, {}.", count_sides(&filled_grid));
}
