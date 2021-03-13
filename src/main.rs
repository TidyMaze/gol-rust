use rand::prelude::*;

const HEIGHT: usize = 10;
const WIDTH: usize = 20;
type Grid = [[bool; WIDTH]; HEIGHT];

fn show_bool(b: &bool) -> &str {
    return if *b { "O" } else { "." };
}

fn print_grid(grid: Grid) {
    let lines = grid
        .iter()
        .map(|l| l.iter().map(show_bool).collect::<Vec<&str>>().join(""))
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", lines);
}

fn dead_or_alive(alive: bool, neighbors: u8) -> bool {
    return (alive && neighbors >= 2 && neighbors <= 3) || (!alive && neighbors == 3);
}

fn count_neighbors(grid: Grid, j: usize, i: usize) -> u8 {
    let neighbors: [(i8, i8); 8] = [
        (j as i8 - 1, i as i8 - 1),
        (j as i8, i as i8 - 1),
        (j as i8 + 1, i as i8 - 1),
        (j as i8 - 1, i as i8),
        (j as i8 + 1, i as i8),
        (j as i8 - 1, i as i8 + 1),
        (j as i8, i as i8 + 1),
        (j as i8 + 1, i as i8 + 1),
    ];
    return neighbors
        .iter()
        .filter(|(oJ, oI)| {
            *oJ >= 0
                && *oJ < WIDTH as i8
                && *oI >= 0
                && *oI < HEIGHT as i8
                && grid[*oI as usize][*oJ as usize]
        })
        .count() as u8;
}

fn one_step(grid: Grid) -> Grid {
    let mut new_grid: Grid = [[false; WIDTH]; HEIGHT];
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            new_grid[i][j] = dead_or_alive(grid[i][j], count_neighbors(grid, j, i));
        }
    }
    return new_grid;
}

fn make_rand_grid() -> Grid {
    let mut res = [[false; WIDTH]; HEIGHT];
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            res[i][j] = random();
        }
    }
    return res;
}

fn main() {
    let init = make_rand_grid();
    let mut g = init;
    for _i in 0..100 {
        g = one_step(g);
        println!("====================");
        print_grid(g)
    }
}
