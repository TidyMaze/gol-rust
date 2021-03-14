use macroquad::prelude::rand::gen_range;
use macroquad::prelude::*;

type Grid = Vec<Vec<bool>>;

fn show_bool(b: &bool) -> &str {
    return if *b { "O" } else { "." };
}

fn print_grid(grid: &Grid) {
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

fn count_neighbors(grid: &Grid, j: usize, i: usize) -> u8 {
    let mut cnt = 0;
    for (o_j, o_i) in &OFFSETS {
        let dj: i16 = j as i16 + o_j;
        let di: i16 = i as i16 + o_i;
        if dj >= 0
            && dj < grid[0].len() as i16
            && di >= 0
            && di < grid.len() as i16
            && grid[di as usize][dj as usize]
        {
            cnt += 1;
        }
    }

    return cnt;
}

fn one_step(grid: &Grid) -> Grid {
    let mut new_grid: Grid = Vec::new();
    for i in 0..grid.len() {
        let mut line: Vec<bool> = Vec::new();

        for j in 0..grid[0].len() {
            line.push(dead_or_alive(grid[i][j], count_neighbors(grid, j, i)));
        }
        new_grid.push(line);
    }
    return new_grid;
}

fn make_rand_grid(height: usize, width: usize) -> Grid {
    let mut res: Grid = Vec::new();
    for i in 0..height {
        let mut line: Vec<bool> = Vec::new();

        for j in 0..width {
            line.push(gen_range(0, 101) >= 50);
        }
        res.push(line);
    }
    return res;
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let height = screen_height() as u32;
    let width = screen_width() as u32;

    let init = make_rand_grid(height as usize, width as usize);
    let mut g = init;

    for i in 0..10000 {
        clear_background(BLACK);
        g = one_step(&g);
        println!("==================== - {}", i);
        // print_grid(&g)

        for i in 0..height as usize {
            for j in 0..width as usize {
                if g[i][j] {
                    draw_rectangle(j as f32, i as f32, 1 as f32, 1 as f32, BLUE);
                }
            }
        }

        next_frame().await
    }
}
