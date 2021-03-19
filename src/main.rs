use macroquad::prelude::rand::gen_range;
use macroquad::prelude::*;
use std::time::SystemTime;

type Grid = Vec<bool>;

fn coord_to_index(width: usize, j: usize, i: usize) -> usize {
    return i * width + j;
}

fn index_to_coord(width: usize, index: usize) -> (usize, usize) {
    return (index % width, index / width);
}

fn set_grid(g: &mut Grid, width: usize, j: usize, i: usize, v: bool) {
    g[coord_to_index(width, j, i)] = v;
}

fn get_grid(g: &Grid, width: usize, j: usize, i: usize) -> bool {
    return g[coord_to_index(width, j, i)];
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

fn in_map(width: usize, height: usize, j: i16, i: i16) -> bool {
    return j >= 0 && j < width as i16 && i >= 0 && i < height as i16;
}

fn count_neighbors(width: usize, height: usize, grid: &Grid, j: usize, i: usize) -> u8 {
    let mut cnt = 0;
    for (o_j, o_i) in &OFFSETS {
        let dj: i16 = j as i16 + o_j;
        let di: i16 = i as i16 + o_i;
        if in_map(width, height, dj, di) {
            if get_grid(grid, width, dj as usize, di as usize) {
                cnt += 1;
            }
        }
    }

    return cnt;
}

fn one_step(
    grid: &mut Grid,
    buffer: &mut Grid,
    width: usize,
    height: usize,
    neighbor_of_changed_cell: &mut Grid,
) {
    for i in 0..height {
        for j in 0..width {
            let idx = coord_to_index(width, j, i);
            if neighbor_of_changed_cell[idx] {
                buffer[idx] = dead_or_alive(grid[idx], count_neighbors(width, height, grid, j, i));
                neighbor_of_changed_cell[idx] = false;
            }
        }
    }

    for i in 0..height {
        for j in 0..width {
            let idx = coord_to_index(width, j, i);
            if buffer[idx] != grid[idx] {
                for (o_j, o_i) in &OFFSETS {
                    let dj: i16 = j as i16 + o_j;
                    let di: i16 = i as i16 + o_i;
                    if in_map(width, height, dj, di) {
                        set_grid(
                            neighbor_of_changed_cell,
                            width,
                            dj as usize,
                            di as usize,
                            true,
                        );
                    }
                }
                neighbor_of_changed_cell[idx] = true;
            }
            grid[idx] = buffer[idx];
        }
    }
}

fn make_empty_grid(height: usize, width: usize) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    for _i in 0..height {
        for _j in 0..width {
            res.push(0);
        }
    }
    return res;
}

fn make_empty_grid_bool(height: usize, width: usize, value: bool) -> Grid {
    let mut res: Grid = Vec::new();
    for _i in 0..height {
        for _j in 0..width {
            res.push(value);
        }
    }
    return res;
}

fn make_rand_grid(height: usize, width: usize) -> Grid {
    let mut res: Grid = Vec::new();
    for _i in 0..height {
        for _j in 0..width {
            res.push(gen_range(0, 101) < 50);
        }
    }
    return res;
}

fn map_range(from_range: (f32, f32), to_range: (f32, f32), s: f32) -> f32 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

fn aspect_ratio() -> f32 {
    return screen_width() / screen_height()
}

fn make_and_set_camera(aspect_ratio: f32) -> Camera2D {
    let camera = Camera2D::from_display_rect(Rect {
        x: 0 as f32,
        y: 0 as f32,
        w: aspect_ratio * 200 as f32,
        h: 200 as f32,
    });
    set_camera(camera);
    camera
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let height = 200 as usize;
    let width = 200 as usize;

    println!("{} {}", height, width);

    let mut hot = make_empty_grid(height as usize, width as usize);

    let mut g = make_rand_grid(height as usize, width as usize);
    let mut buffer = make_empty_grid_bool(height as usize, width as usize, false);
    let mut neighbor_of_updated_cell = make_empty_grid_bool(height as usize, width as usize, true);

    let mut color = Color::new(0.00, 0.00, 0.00, 1.00);

    let texture =
        load_texture_from_image(&Image::gen_image_color(width as u16, height as u16, BLACK));

    let start = SystemTime::now();

    let mut count_step: u32 = 0;

    let total_cells = height * width;

    let mut img: Image = Image::gen_image_color(width as u16, height as u16, BLACK);
    

    loop {
        make_and_set_camera(aspect_ratio());

        let step = 1;

        for _sub in 0..step {
            one_step(
                &mut g,
                &mut buffer,
                width,
                height,
                &mut neighbor_of_updated_cell,
            );
            count_step += 1;
            let elapsed = SystemTime::now().duration_since(start).unwrap().as_secs();
            let speed = count_step as f32 / elapsed as f32;

            let mut cnt_changed = 0;
            for i in 0..height {
                for j in 0..width {
                    if get_grid(&neighbor_of_updated_cell, width, j, i) {
                        cnt_changed += 1;
                    }
                }
            }

            if (count_step % 100) == 0 {
                println!(
                    "{} - {} - changed look up {} / {} (skipped {}% with lookup) fps {}",
                    count_step,
                    speed as u16,
                    cnt_changed,
                    total_cells,
                    100 - ((cnt_changed as f32 / total_cells as f32) * 100 as f32) as u16,
                    get_fps()
                );
            }
        }

        for i in 0..height as usize {
            for j in 0..width as usize {
                let idx = coord_to_index(width, j, i);
                if g[idx] {
                    hot[idx] = 255;
                    img.set_pixel(j as u32, i as u32, WHITE);
                } else {
                    if hot[idx] > 100 {
                        hot[idx] -= 1;
                    }
                    if hot[idx] > 0 {
                        color.b = map_range(
                            (0 as f32, 255 as f32),
                            (0.0 as f32, 1.0 as f32),
                            hot[idx] as f32,
                        );
                        img.set_pixel(j as u32, i as u32, color);
                    }
                }
            }
        }

        update_texture(texture, &img);
        set_texture_filter(texture, macroquad::texture::FilterMode::Nearest);
        draw_texture(texture, 0 as f32, 0 as f32, WHITE);
        next_frame().await
    }
}
