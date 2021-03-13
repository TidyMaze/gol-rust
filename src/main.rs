use rand::prelude::*;

const HEIGHT: usize = 200;
const WIDTH: usize = 400;
type Grid = [[bool; WIDTH]; HEIGHT];

// fn show_bool(b: &bool) -> &str {
//     return if *b { "O" } else { "." };
// }

// fn print_grid(grid: Grid) {
//     let lines = grid
//         .iter()
//         .map(|l| l.iter().map(show_bool).collect::<Vec<&str>>().join(""))
//         .collect::<Vec<String>>()
//         .join("\n");
//     println!("{}", lines);
// }

fn dead_or_alive(alive: bool, neighbors: u8) -> bool {
    return (alive && neighbors >= 2 && neighbors <= 3) || (!alive && neighbors == 3);
}

const OFFSETS: [(i16, i16); 8] = [
    (0 - 1, 0 - 1),
    (0, 0 - 1),
    (0 + 1, 0 - 1),
    (0 - 1, 0),
    (0 + 1, 0),
    (0 - 1, 0 + 1),
    (0, 0 + 1),
    (0 + 1, 0 + 1),
];

fn count_neighbors(grid: Grid, j: usize, i: usize) -> u8 {
    let mut cnt = 0;
    for (o_j, o_i) in &OFFSETS {
        let dj: i16 = j as i16 + o_j;
        let di: i16 = i as i16 + o_i;
        if dj >= 0
            && dj < WIDTH as i16
            && di >= 0
            && di < HEIGHT as i16
            && grid[di as usize][dj as usize]
        {
            cnt += 1;
        }
    }

    return cnt;
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
    for i in 0..100 {
        g = one_step(g);
        println!("==================== - {}", i);
        // print_grid(g)
    }
}
