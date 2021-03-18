use macroquad::prelude::rand::gen_range;
use macroquad::prelude::*;
use std::time::SystemTime;

type Grid = Vec<Vec<bool>>;

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

fn in_map(grid: &Grid, j: i16, i: i16) -> bool {
    return j >= 0 && j < grid[0].len() as i16 && i >= 0 && i < grid.len() as i16;
}

fn count_neighbors(grid: &Grid, j: usize, i: usize) -> u8 {
    let mut cnt = 0;
    for (o_j, o_i) in &OFFSETS {
        let dj: i16 = j as i16 + o_j;
        let di: i16 = i as i16 + o_i;
        if in_map(grid, dj, di) && grid[di as usize][dj as usize] {
            cnt += 1;
        }
    }

    return cnt;
}

fn one_step(grid: &mut Grid, buffer: &mut Grid, neighbor_of_changed_cell: &mut Grid) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if neighbor_of_changed_cell[i][j] {
                buffer[i][j] = dead_or_alive(grid[i][j], count_neighbors(grid, j, i));
                neighbor_of_changed_cell[i][j] = false;
            }
        }
    }

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if buffer[i][j] != grid[i][j] {
                for (o_j, o_i) in &OFFSETS {
                    let dj: i16 = j as i16 + o_j;
                    let di: i16 = i as i16 + o_i;
                    if in_map(grid, dj, di) {
                        neighbor_of_changed_cell[di as usize][dj as usize] = true;
                    }
                }
                neighbor_of_changed_cell[i][j] = true;
            }
            grid[i][j] = buffer[i][j];
        }
    }
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

fn make_empty_grid_bool(height: usize, width: usize) -> Vec<Vec<bool>> {
    let mut res: Vec<Vec<bool>> = Vec::new();
    for _i in 0..height {
        let mut line: Vec<bool> = Vec::new();

        for _j in 0..width {
            line.push(false);
        }
        res.push(line);
    }
    return res;
}

fn make_empty_grid_bool_true(height: usize, width: usize) -> Vec<Vec<bool>> {
    let mut res: Vec<Vec<bool>> = Vec::new();
    for _i in 0..height {
        let mut line: Vec<bool> = Vec::new();

        for _j in 0..width {
            line.push(true);
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

    let mut hot = make_empty_grid(height as usize, width as usize);

    let mut g = make_rand_grid(height as usize, width as usize);
    let mut buffer = make_empty_grid_bool(height as usize, width as usize);
    let mut look_change_map = make_empty_grid_bool_true(height as usize, width as usize);

    let mut color = Color::new(0.00, 0.00, 0.00, 1.00);

    let texture =
        load_texture_from_image(&Image::gen_image_color(width as u16, height as u16, BLACK));

    let start = SystemTime::now();

    let mut count_step: u32 = 0;

    let total_cells = height * width;

    let mut img: Image = Image::gen_image_color(width as u16, height as u16, BLACK);

    for _i in 0..10000 {
        let step = 2;

        for _sub in 0..step {
            one_step(&mut g, &mut buffer, &mut look_change_map);
            count_step += 1;
            let elapsed = SystemTime::now().duration_since(start).unwrap().as_secs();
            let speed = count_step as f32 / elapsed as f32;

            let mut cnt_changed = 0;
            for i in 0..g.len() {
                for j in 0..g[0].len() {
                    if look_change_map[i][j] {
                        cnt_changed += 1;
                    }
                }
            }

            println!(
                "{} - {} - changed look up {} / {} (skipped {}% with lookup)",
                count_step,
                speed as u16,
                cnt_changed,
                total_cells,
                100 - ((cnt_changed as f32 / total_cells as f32) * 100 as f32) as u16
            );
        }

        for i in 0..height as usize {
            for j in 0..width as usize {
                if g[i][j] {
                    hot[i][j] = 255;
                    img.set_pixel(j as u32, i as u32, WHITE);
                } else {
                    if hot[i][j] > 100 {
                        hot[i][j] = hot[i][j] - 1;
                    }
                    if hot[i][j] > 0 {
                        color.b = map_range(
                            (0 as f32, 255 as f32),
                            (0.0 as f32, 1.0 as f32),
                            hot[i][j] as f32,
                        );
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
