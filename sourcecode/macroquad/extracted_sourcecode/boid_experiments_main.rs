use std::ops::Add;

extern crate macroquad;

struct Boid {
    position: macroquad::math::Vec2,
    velocity: macroquad::math::Vec2,
    acc: macroquad::math::Vec2,
    color: macroquad::color::Color,
}

const MAXVELOCITY: f32 = 5.0;
const MAXACCELERATION: f32 = 3.0;

impl Boid {
    fn apply(&mut self, dt: f32) {
        self.acc = self.acc.clamp_length_max(MAXACCELERATION);
        self.velocity += self.acc * dt;
        self.position += self.velocity * dt;
        let w = macroquad::window::screen_width();
        let h = macroquad::window::screen_height();
        self.position.x = (self.position.x / w).fract() * w;
        if self.position.x < 0.0 {
            self.position.x += w;
        }
        self.position.y = (self.position.y / h).fract() * h;
        if self.position.y < 0.0 {
            self.position.y += h;
        }

        self.velocity = self.velocity.clamp_length_max(MAXVELOCITY);
    }

    fn draw(&self) {
        let drawing_radius = 10.0;
        let norm_vel = self.velocity.normalize();
        let v1 = self.position + norm_vel * drawing_radius;
        let ang = norm_vel.y.atan2(norm_vel.x);
        let ang2 = ang + std::f32::consts::PI * 0.6;
        let ang3 = ang - std::f32::consts::PI * 0.6;
        let v2 =
            self.position + macroquad::math::Vec2::new(ang2.cos(), ang2.sin()) * drawing_radius;
        let v3 =
            self.position + macroquad::math::Vec2::new(ang3.cos(), ang3.sin()) * drawing_radius;

        macroquad::shapes::draw_triangle(v1, v2, v3, self.color);
        macroquad::shapes::draw_line(
            self.position.x,
            self.position.y,
            v1.x,
            v1.y,
            2.0,
            macroquad::color::RED,
        );
    }

    fn new_random() -> Boid {
        let x_pos = macroquad::rand::gen_range(0.0, macroquad::window::screen_width());
        let y_pos = macroquad::rand::gen_range(0.0, macroquad::window::screen_height());
        let blue = macroquad::rand::gen_range(128, 255);
        let rg = macroquad::rand::gen_range(0, blue);
        let boid = Boid {
            position: macroquad::math::Vec2::new(x_pos, y_pos),
            velocity: macroquad::math::Vec2::new(
                macroquad::rand::gen_range(-1.0, 1.0),
                macroquad::rand::gen_range(-1.0, 1.0),
            )
            .normalize(),
            acc: macroquad::math::Vec2::ZERO,
            color: macroquad::color::Color::from_rgba(
                rg,
                rg,
                macroquad::rand::gen_range(128, 255),
                255,
            ),
        };
        return boid;
    }
}

struct Boidgroup {
    group: Vec<Boid>,
}

impl Boidgroup {
    fn populate_random(size: u8) -> Boidgroup {
        let mut group = Vec::new();
        for _i in 0..size {
            group.push(Boid::new_random());
        }

        return Boidgroup { group };
    }

    fn draw(&self) {
        for boid in self.group.iter() {
            boid.draw();
        }
    }

    fn apply(&mut self, timestep: f32, perception_radius: f32, alignment: f32, cohesion: f32) {
        for boid in self.group.iter_mut() {
            boid.apply(macroquad::time::get_frame_time() * timestep);
        }
        self.align(perception_radius, alignment, cohesion);
    }

    fn align(&mut self, perception_radius: f32, alignment: f32, cohesion: f32) {
        for i in 0..self.group.len() {
            let mut total = 0;
            let mut average_velocity = macroquad::math::Vec2::ZERO;
            let mut average_position = macroquad::math::Vec2::ZERO;

            for j in 0..self.group.len() {
                if i == j {
                    continue;
                }
                let boid1 = &self.group[i];
                let boid2 = &self.group[j];
                if boid1.position.distance(boid2.position) < perception_radius {
                    total += 1;
                    average_velocity += boid2.velocity;
                    average_position += boid2.position;
                }
            }

            if total > 1 {
                average_velocity *= 1.0 / (total as f32);
                average_position *= 1.0 / (total as f32);

                let mut boid1 = &mut self.group[i];
                average_position -= boid1.position;

                //steering behavior
                boid1.acc = (average_velocity - boid1.velocity).normalize() * alignment
                    - average_position.normalize() * cohesion;
            }
        }
    }
}

use macroquad::hash;
#[macroquad::main("Boids")]
async fn main() {
    let mut group = Boidgroup::populate_random(255);
    let mut cohesion = 0.47;
    let mut alignment = -0.14;
    let mut perception_radius = 100.0;
    let mut time_step = 50.0;
    loop {
        macroquad::ui::widgets::Window::new(
            hash!(),
            macroquad::math::vec2(0.0, 0.0),
            macroquad::math::vec2(420.0, 100.0),
        )
        .label("Boid Controls")
        .ui(&mut macroquad::ui::root_ui(), |ui| {
            ui.slider(hash!(), "alignment", -1.0..1.0, &mut alignment);
            ui.slider(hash!(), "cohesion", -1.0..1.0, &mut cohesion);
            ui.slider(
                hash!(),
                "perception radius",
                0.0..100.0,
                &mut perception_radius,
            );

            if ui.button(None, "randomize") {
                group = Boidgroup::populate_random(255);
            }

            ui.slider(hash!(), "time", 0.0..100.0, &mut time_step);
        });

        macroquad::window::clear_background(macroquad::color::BLACK);
        group.draw();
        group.apply(time_step, perception_radius, alignment, -cohesion);
        macroquad::window::next_frame().await
    }
}
