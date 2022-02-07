extern crate macroquad;

const WIDTH_HEIGHT: usize = 60;
const WIDTH_HEIGHT_FLOAT: f32 = WIDTH_HEIGHT as f32;
const SECONDS_PER_TICK: f64 = 0.15;
const FOREGROUND_COLOR: macroquad::color::Color = macroquad::color::WHITE;
const FOREGROUND_COLOR2: macroquad::color::Color = macroquad::color::LIGHTGRAY;

struct SnowFlakeSim {
    snow_bars: [usize; WIDTH_HEIGHT],
    current_flakes: Vec<(usize, usize)>,
}

impl SnowFlakeSim {
    fn new() -> Self {
        SnowFlakeSim {
            snow_bars: [0; WIDTH_HEIGHT],
            current_flakes: vec![],
        }
    }

    fn spawn_flakes(&mut self, spawn_chance_1_to: u32) {
        for i in 0..WIDTH_HEIGHT {
            let random_number = macroquad::rand::gen_range(1, spawn_chance_1_to + 1);
            if random_number % spawn_chance_1_to == 1 {
                self.current_flakes.push((i, 0));
            }
        }
    }

    fn tick(&mut self) {
        //simulate the flakes:
        let mut elements_to_remove = vec![];
        for (i, (flake_x, flake_y)) in self.current_flakes.iter_mut().enumerate() {
            *flake_y += 1;
            if WIDTH_HEIGHT - *flake_y <= self.snow_bars[*flake_x] {
                self.snow_bars[*flake_x] += 1;
                elements_to_remove.push(i);
            }
        }
        let mut count_removed_elements = 0;
        for index in elements_to_remove.iter() {
            self.current_flakes.remove(index - count_removed_elements);
            count_removed_elements += 1;
        }
        println!("lenght flakes: {}", self.current_flakes.len());
    }

    fn maximum_reached(&self) -> bool {
        for bar in self.snow_bars.iter() {
            if *bar > WIDTH_HEIGHT {
                return true;
            }
        }
        return false;
    }

    fn reset_bars(&mut self) {
        self.snow_bars = [0; WIDTH_HEIGHT];
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut paused = false;
    let mut simulation = SnowFlakeSim::new();
    let mut last_time = macroquad::time::get_time();
    //prerun the sim at the beginning
    for _i in 0..WIDTH_HEIGHT + 10 {
        simulation.spawn_flakes(10);
        simulation.tick();
    }
    loop {
        let elapsed = macroquad::time::get_time() - last_time;

        //redrawing the screen:
        macroquad::window::clear_background(macroquad::color::BLACK);

        let block_width = macroquad::window::screen_width() / WIDTH_HEIGHT_FLOAT;
        let block_height = macroquad::window::screen_height() / WIDTH_HEIGHT_FLOAT;
        //draw all the snow flakes.
        for (flake_x, flake_y) in simulation.current_flakes.iter() {
            let flake_x = *flake_x as f32 * block_width;
            //problem: the flakes get imedietly moved down at spawn, meaning the y=0 row is empty.
            //solution: substract 1
            let flake_y = (*flake_y as f32 - 1.0) * block_height;
            macroquad::shapes::draw_rectangle(
                flake_x,
                flake_y,
                block_width,
                block_height,
                FOREGROUND_COLOR,
            );
        }
        //draw all the bars:
        for (index, value) in simulation.snow_bars.iter().enumerate() {
            let bar_x = index as f32 * block_width;
            let bar_height = *value as f32 * block_height;
            macroquad::shapes::draw_rectangle(
                bar_x,
                macroquad::window::screen_height() - bar_height,
                block_width,
                bar_height,
                FOREGROUND_COLOR2,
            );
        }

        if elapsed > SECONDS_PER_TICK && !paused {
            //logic of the simulation:
            last_time = macroquad::time::get_time();
            simulation.spawn_flakes(10);
            simulation.tick();
            if simulation.maximum_reached() {
                simulation.reset_bars();
            }
        }

        //draw_line(x, 40.0, 100.0, 200.0, 15.0, BLUE);
        //draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        macroquad::text::draw_text(
            &format!("Snowflakes:{}", simulation.current_flakes.len()),
            20.0,
            20.0,
            20.0,
            macroquad::color::RED,
        );
        if paused {
            macroquad::text::draw_text("Paused", 20.0, 40.0, 20.0, macroquad::color::RED);
        }
        if macroquad::input::is_mouse_button_released(macroquad::input::MouseButton::Left) {
            paused = !paused;
        } else if macroquad::input::is_key_pressed(macroquad::input::KeyCode::R) {
            simulation.reset_bars();
            simulation.current_flakes = vec![];
        }
        macroquad::window::next_frame().await
    }
}
