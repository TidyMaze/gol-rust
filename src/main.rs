use macroquad::prelude::rand::gen_range;
use macroquad::prelude::*;

type Grid = Vec<Vec<bool>>;

// fn show_bool(b: &bool) -> &str {
//     return if *b { "O" } else { "." };
// }

// fn print_grid(grid: &Grid) {
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

fn make_empty_grid(height: usize, width: usize) -> Vec<Vec<u8>> {
    let mut res: Vec<Vec<u8>> = Vec::new();
    for _i in 0..height {
        let mut line: Vec<u8> = Vec::new();

        for _j in 0..width {
            line.push(0);
        }
        res.push(line);
    }
    return res;
}

fn make_rand_grid(height: usize, width: usize) -> Grid {
    let mut res: Grid = Vec::new();
    for _i in 0..height {
        let mut line: Vec<bool> = Vec::new();

        for _j in 0..width {
            line.push(gen_range(0, 101) < 7);
        }
        res.push(line);
    }
    return res;
}

fn map_range(from_range: (f32, f32), to_range: (f32, f32), s: f32) -> f32 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let height = screen_height() as u32;
    let width = screen_width() as u32;

    println!("{} {}", height, width);

    let init = make_rand_grid(height as usize, width as usize);

    let mut hot = make_empty_grid(height as usize, width as usize);

    let mut g = init;

    clear_background(BLACK);

    pub const BLACK_ALPHA: Color = Color::new(0.00, 0.00, 0.00, 1.0);

    let mut color = Color::new(0.00, 0.00, 0.00, 1.00);

    let texture = load_texture_from_image(&Image::gen_image_color(width as u16, height as u16, BLACK));

    for i in 0..10000 {
        clear_background(BLACK_ALPHA);

        let step = 1;

        for sub in 0..step {
            g = one_step(&g);
            println!("==================== - {}", step * i + sub);
        }
        // print_grid(&g)

        let mut img: Image = Image::gen_image_color(width as u16, height as u16, BLACK);

        for i in 0..height as usize {
            for j in 0..width as usize {
                if g[i][j] {
                    hot[i][j] = 255;
                    img.set_pixel(j as u32, i as u32, BLUE);
                } else {
                    if hot[i][j] > 100 {
                        hot[i][j] = hot[i][j] - 1;
                    }
                    
                    if hot[i][j] > 0 {
                        color.b = map_range((0 as f32,255 as f32), (0.0 as f32, 1.0 as f32), hot[i][j] as f32);
                        img.set_pixel(j as u32, i as u32, color);
                    }
                }
            }
        }

        update_texture(texture, &img);
        draw_texture(texture, 0 as f32, 0 as f32, WHITE);

        next_frame().await
    }
}
