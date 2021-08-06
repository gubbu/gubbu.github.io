extern crate macroquad;

const MAPWIDTH: usize = 20;
const MAPHEIGHT: usize = 20;

fn run_dda(
    map: [[bool; MAPWIDTH]; MAPHEIGHT],
    start_x: f32,
    start_y: f32,
    end_x: f32,
    end_y: f32,
) -> Option<([f32; 2], [usize; 2])> {
    let dx = end_x - start_x;
    let dy = end_y - start_y;
    let ray_len = (dx * dx + dy * dy).sqrt();
    let dx_norm = dx / ray_len;
    let dy_norm = dy / ray_len;
    let step_lenght_x = (1.0 + dy / dx * dy / dx).sqrt();
    let step_lenght_y = (1.0 + dx / dy * dx / dy).sqrt();
    let mut step_x;
    let mut step_y;
    let whole_step_x;
    let whole_step_y;
    if dx < 0.0 {
        step_x = start_x.fract() * step_lenght_x;
        whole_step_x = -1.0;
    } else {
        step_x = (1.0 - start_x.fract()) * step_lenght_x;
        whole_step_x = 1.0;
    };
    if dy < 0.0 {
        step_y = start_y.fract() * step_lenght_y;
        whole_step_y = -1.0;
    } else {
        step_y = (1.0 - start_y.fract()) * step_lenght_y;
        whole_step_y = 1.0;
    };
    let mut covered_distance = 0.0;
    let max_distance = 100.0;
    let mut map_check_x = start_x;
    let mut map_check_y = start_y;
    while covered_distance < max_distance {
        if step_x < step_y {
            map_check_x += whole_step_x;
            covered_distance = step_x;
            step_x += step_lenght_x;
        } else {
            map_check_y += whole_step_y;
            covered_distance = step_y;
            step_y += step_lenght_y;
        }

        //bounds check
        if map_check_x >= 0.0
            && map_check_x < MAPWIDTH as f32
            && map_check_y >= 0.0
            && map_check_y < MAPHEIGHT as f32
            && map[map_check_y as usize][map_check_x as usize]
        {
            return Some((
                [
                    start_x + dx_norm * covered_distance,
                    start_y + dy_norm * covered_distance,
                ],
                [map_check_x as usize, map_check_y as usize],
            ));
        }
    }
    None
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut obstacle_map = [[false; MAPWIDTH]; MAPHEIGHT];
    let start = (
        macroquad::window::screen_width() * 0.5,
        macroquad::window::screen_height() * 0.5,
    );
    loop {
        macroquad::window::clear_background(macroquad::color::WHITE);
        let box_width = macroquad::window::screen_width() / (MAPWIDTH as f32);
        let box_height: f32 = macroquad::window::screen_height() / (MAPHEIGHT as f32);
        let (mouse_x, mouse_y) = macroquad::input::mouse_position();
        let tile_coordinate_x = (mouse_x / box_width).trunc();
        let tile_coordinate_y = (mouse_y / box_height).trunc();
        //draw map
        for (y, row) in obstacle_map.iter().enumerate() {
            for (x, obstacle) in row.iter().enumerate() {
                let x_f32 = x as f32 * box_width;
                let y_f32 = y as f32 * box_height;
                if *obstacle {
                    macroquad::shapes::draw_rectangle(
                        x_f32,
                        y_f32,
                        box_width,
                        box_height,
                        macroquad::color::BLUE,
                    );
                }
            }
        }
        macroquad::shapes::draw_rectangle(
            tile_coordinate_x * box_width,
            tile_coordinate_y * box_height,
            box_width,
            box_height,
            macroquad::color::Color {
                r: 0.0,
                g: 1.0,
                b: 0.0,
                a: 0.5,
            },
        );
        macroquad::shapes::draw_line(
            start.0,
            start.1,
            mouse_x,
            mouse_y,
            2.0,
            macroquad::color::DARKBLUE,
        );
        if macroquad::input::is_mouse_button_down(macroquad::input::MouseButton::Left) {
            let x_index = tile_coordinate_x.abs() as usize;
            let y_index = tile_coordinate_y.abs() as usize;
            if x_index < MAPWIDTH && y_index < MAPHEIGHT {
                obstacle_map[y_index][x_index] = true;
            }
        } else if macroquad::input::is_mouse_button_down(macroquad::input::MouseButton::Right) {
            let x_index = tile_coordinate_x.abs() as usize;
            let y_index = tile_coordinate_y.abs() as usize;
            if x_index < MAPWIDTH && y_index < MAPHEIGHT {
                obstacle_map[y_index][x_index] = false;
            }
        }

        if let Some(([x, y], [xi, yi])) = run_dda(
            obstacle_map,
            start.0 / box_width,
            start.1 / box_height,
            mouse_x / box_width,
            mouse_y / box_height,
        ) {
            macroquad::shapes::draw_circle(
                x * box_width,
                y * box_height,
                10.0,
                macroquad::color::Color {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.5,
                },
            );
            let box_x_cord = xi as f32 * box_width;
            let box_y_cord = yi as f32 * box_height;
            macroquad::shapes::draw_rectangle(
                box_x_cord,
                box_y_cord,
                box_width,
                box_height,
                macroquad::color::Color {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.5,
                },
            );
        }
        macroquad::window::next_frame().await
    }
}
