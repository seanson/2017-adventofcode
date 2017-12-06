use std::mem;

const GRID_SIZE: usize = 1024;
const ORIGIN: usize = GRID_SIZE / 2;
const DIRS: [[isize;2];4] = [[0, 1], [-1, 0], [0, -1], [1, 0]];
const SWEEP: [[isize;2];8] = [[-1, -1],
                             [-1, 0],
                             [-1, 1],
                             [0, -1],
                             [0, 1],
                             [1, -1],
                             [1, 0],
                             [1, 1],
                             ];

fn walk_grid(end: u32) -> (i32, u32) {
    let mut grid: [[u32;GRID_SIZE];GRID_SIZE] = [[0; GRID_SIZE];GRID_SIZE];
    println!("Size of grid {}x{}: {}", GRID_SIZE, GRID_SIZE, mem::size_of_val(&grid));
    let mut x: usize = ORIGIN;
    let mut y: usize = ORIGIN;
    grid[ORIGIN][ORIGIN] = 1;
    
    let mut dir_it = DIRS.iter().cycle();
    let mut cur: [isize;2] = [0, 0];
    let mut next = dir_it.next().unwrap();
    let mut found: bool = false;
    let mut result: u32 = 0;
    for _i in 1..end {
        if found {
            grid[y][x] = 1;
        }
        else {
            result = SWEEP.iter()
                          .map(|sweep| grid[y.wrapping_add(sweep[0] as usize)][x.wrapping_add(sweep[1] as usize)])
                          .sum();
            found = result > end;
            grid[y][x] += &result;
        }  
        
        let target_y = y.wrapping_add(next[0] as usize);
        let target_x = x.wrapping_add(next[1] as usize);

        if grid[target_y][target_x] == 0 {
            cur = *next;
            next = dir_it.next().unwrap();
        }
        y = y.wrapping_add(cur[0] as usize);
        x = x.wrapping_add(cur[1] as usize);   
    }
    ((x as i32 - ORIGIN as i32).abs() + (y as i32 - ORIGIN as i32).abs(), result)
}


fn main() {
    let results = walk_grid(325489);
    println!("Manhattan distance: {}, first cluster bigger than input: {}", results.0, results.1)
}