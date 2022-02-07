extern crate macroquad;

mod inverse_cinematics {

    fn manhatten_distance(a: [f32; 2], b: [f32; 2]) -> f32 {
        return (a[0] - b[0]).abs() + (a[1] - b[1]).abs();
    }

    pub fn inverse_cinematics(
        position: [f32; 2],
        target: [f32; 2],
        forarm_len: f32,
        upper_arm_len: f32,
        normal: [f32; 2], //in 2D there are two solutions for inverse cinematics... which one do you chose? false: solution 1, true: solution 2.
    ) -> ([f32; 2], [f32; 2], bool) {
        let delta_x = target[0] - position[0];
        let delta_y = target[1] - position[1];
        let distance = (delta_x * delta_x + delta_y * delta_y).sqrt();
        if distance > forarm_len + upper_arm_len {
            return (
                [
                    position[0] + delta_x / distance * upper_arm_len,
                    position[1] + delta_y / distance * upper_arm_len,
                ],
                [
                    position[0] + delta_x / distance * (forarm_len + upper_arm_len),
                    position[1] + delta_y / distance * (forarm_len + upper_arm_len),
                ],
                false,
            );
        }

        //solve inverse Kinematics....
        let distance1 = (upper_arm_len * upper_arm_len - forarm_len * forarm_len
            + distance * distance)
            / (2.0 * distance);

        let height = (upper_arm_len * upper_arm_len - distance1 * distance1).sqrt();

        if distance1.is_nan() || height.is_nan() {
            return (
                [
                    position[0] + delta_x / distance * upper_arm_len,
                    position[1] + delta_y / distance * upper_arm_len,
                ],
                [
                    position[0] + delta_x / distance * (upper_arm_len - forarm_len),
                    position[1] + delta_y / distance * (upper_arm_len - forarm_len),
                ],
                false,
            );
        }

        let mut orthogonal_delta = [-delta_y / distance, delta_x / distance];
        let orthogonal_delta2 = [delta_y / distance, -delta_x / distance];
        if manhatten_distance(orthogonal_delta2, normal)
            > manhatten_distance(orthogonal_delta, normal)
        {
            orthogonal_delta = orthogonal_delta2;
        }

        let joint_x = position[0] + (delta_x / distance) * distance1 + orthogonal_delta[0] * height;
        let joint_y = position[1] + (delta_y / distance) * distance1 + orthogonal_delta[1] * height;
        return ([joint_x, joint_y], [target[0], target[1]], true);
    }
}

fn veclenght(v: &[f32; 2]) -> f32 {
    return (v[0] * v[0] + v[1] * v[1]).sqrt();
}

fn delta(a: &[f32; 2], b: &[f32; 2]) -> [f32; 2] {
    return [a[0] - b[0], a[1] - b[1]];
}

fn normalize(v: &mut [f32; 2]) {
    let l = veclenght(v);
    v[0] /= l;
    v[1] /= l;
}

fn add_vectors(v1: &[f32; 2], v2: &[f32; 2]) -> [f32; 2] {
    return [v1[0] + v2[0], v1[1] + v2[1]];
}

fn scale(a: f32, v: &[f32; 2]) -> [f32; 2] {
    return [a * v[0], a * v[1]];
}

fn normalize_3d(a: [f32; 3]) -> [f32; 3] {
    let l = a[0] * a[0] + a[1] * a[1] + a[2] * a[2];
    return [a[0] / l, a[1] / l, a[2] / l];
}

fn random_color() -> macroquad::color::Color {
    let r1 = macroquad::rand::gen_range(0.0, 1.0);
    let g = macroquad::rand::gen_range(0.0, 1.0);
    let b = macroquad::rand::gen_range(0.0, 1.0);
    let colors = normalize_3d([r1, g, b]);
    return macroquad::color::Color {
        a: 1.0,
        r: colors[0],
        g: colors[1],
        b: colors[2],
    };
}

fn outside_screen(pos: [f32; 2]) -> bool {
    let width = macroquad::window::screen_width();
    let height = macroquad::window::screen_height();
    return pos[0] < 0f32 || pos[1] < 0f32 || pos[0] > width || pos[1] > height;
}

fn random_on_screen() -> [f32; 2] {
    let width = macroquad::window::screen_width();
    let height = macroquad::window::screen_height();
    return [
        macroquad::rand::gen_range(0.0, width),
        macroquad::rand::gen_range(0.0, height),
    ];
}

mod walker {

    use crate::{
        add_vectors, delta, draw_robot_arm, normalize, outside_screen, random_color,
        random_on_screen, scale, veclenght,
    };

    #[derive(Default)]
    pub struct Walker1 {
        //normalized
        velocity: [f32; 2],
        position: [f32; 2],
        feet_offsets: [[f32; 2]; 2],
        feet_positions: [[f32; 2]; 2],
        feet_interpolation_target: [(bool, [f32; 2]); 2],
        for_arm_lenght: f32,
        upper_arm_len: f32,
        radius: f32,
        speed: f32,
        color: macroquad::color::Color,
    }

    pub struct WalkerGroup {
        walkers: Vec<Walker1>,
    }

    impl WalkerGroup {
        pub fn randomly_generate(
            num: usize,
            spawn_rect: [f32; 4],
            scales: [f32; 2],
        ) -> WalkerGroup {
            let mut walkers = vec![];
            for _i in 0..num {
                walkers.push(Walker1::new_random(spawn_rect, scales))
            }
            return WalkerGroup { walkers: walkers };
        }

        pub fn draw_all(&self) {
            for walker in self.walkers.iter() {
                walker.draw();
            }
        }

        pub fn update_all(&mut self, dt: f32) {
            for walker in self.walkers.iter_mut() {
                walker.move_forward(dt);
                if outside_screen(walker.position) {
                    walker.reset_pos(random_on_screen());
                }
            }
        }
    }

    impl Walker1 {
        pub fn move_forward(&mut self, dt: f32) {
            self.position[0] += self.velocity[0] * dt * self.speed;
            self.position[1] += self.velocity[1] * dt * self.speed;
            //calculate distance between feet and central body ...
            for (i, foot_pos) in self.feet_positions.iter_mut().enumerate() {
                let distance = veclenght(&delta(foot_pos, &self.position));
                if self.feet_interpolation_target[i].0 {
                    //move the feet forward in desired direction...
                    *foot_pos = add_vectors(&foot_pos, &scale(2.0, &self.velocity));
                    //if we are close enough stop moving the foot:
                    let interpolation_target = self.feet_interpolation_target[i].1;
                    if 0.1
                        > (veclenght(&delta(&interpolation_target, &foot_pos)) - self.radius).abs()
                    {
                        self.feet_interpolation_target[i].0 = false;
                    }
                }
                //check wether distance to walker position has grown to large:
                else if distance > (self.for_arm_lenght + self.upper_arm_len) {
                    self.feet_interpolation_target[i].0 = true;
                    let armscale = self.for_arm_lenght + self.upper_arm_len;
                    let next_target = add_vectors(&self.position, &scale(armscale, &self.velocity));
                    //this can be removed.. but it looks better...
                    /*
                    let ortho = [self.velocity[1], -self.velocity[0]];
                    if i == 0{
                        self.feet_interpolation_target[i].1 = add_vectors(&next_target, &scale(-self.radius*1.5, &ortho));
                    }
                    if i == 1{
                        self.feet_interpolation_target[i].1 = add_vectors(&next_target, &scale(self.radius*1.5, &ortho));
                    }*/
                    self.feet_interpolation_target[i].1 = next_target;
                }
            }
        }

        fn reset_pos(&mut self, pos: [f32; 2]) {
            self.position = pos;
            self.init_foot_positions();
            self.feet_interpolation_target[0].0 = false;
            self.feet_interpolation_target[1].0 = false;
        }

        fn init_foot_positions(&mut self) {
            normalize(&mut self.velocity);
            //vector orthogonal to velocity:
            let ortho = [self.velocity[1], -self.velocity[0]];
            let foot_1_offset = scale(self.radius, &ortho);
            let foot_2_offset = scale(-self.radius, &ortho);
            let armscale = (self.for_arm_lenght + self.upper_arm_len) * 0.5;
            let foot_1_pos = add_vectors(&foot_1_offset, &scale(-armscale, &self.velocity));
            let foot2_pos = add_vectors(&foot_2_offset, &scale(armscale, &self.velocity));
            self.feet_offsets = [foot_1_offset, foot_2_offset];
            self.feet_positions = [
                add_vectors(&foot_1_pos, &self.position),
                add_vectors(&foot2_pos, &self.position),
            ];
        }

        pub fn new_random(spawn_rect: [f32; 4], scales: [f32; 2]) -> Walker1 {
            let pos_x = macroquad::rand::gen_range(spawn_rect[0], spawn_rect[0] + spawn_rect[2]);
            let pos_y = macroquad::rand::gen_range(spawn_rect[1], spawn_rect[1] + spawn_rect[3]);
            let velox = macroquad::rand::gen_range(-1.0, 1.0);
            let veloy = macroquad::rand::gen_range(-1.0, 1.0);
            let scale_factor = macroquad::rand::gen_range(scales[0], scales[1]);
            let new_walker = Walker1::new(
                [pos_x, pos_y],
                [velox, veloy],
                20.0 * scale_factor,
                20.0 * scale_factor + scale_factor,
                19. * scale_factor,
                20.0 * scale_factor,
                random_color(),
            );
            return new_walker;
        }

        pub fn draw(&self) {
            macroquad::shapes::draw_circle(
                self.position[0],
                self.position[1],
                self.radius,
                self.color,
            );

            draw_robot_arm(
                add_vectors(&self.position, &self.feet_offsets[0]),
                self.feet_positions[0],
                self.for_arm_lenght,
                self.upper_arm_len,
                [-self.velocity[1], self.velocity[0]],
                self.color,
            );

            draw_robot_arm(
                add_vectors(&self.position, &self.feet_offsets[1]),
                self.feet_positions[1],
                self.for_arm_lenght,
                self.upper_arm_len,
                [self.velocity[1], -self.velocity[0]],
                self.color,
            );
        }

        pub fn new(
            pos: [f32; 2],
            velocity: [f32; 2],
            radius: f32,
            forarm_len: f32,
            upper_arm_len: f32,
            speed: f32,
            color: macroquad::color::Color,
        ) -> Walker1 {
            let mut walker1 = Walker1::default();
            walker1.position = pos;
            walker1.velocity = velocity;
            walker1.radius = radius;
            walker1.for_arm_lenght = forarm_len;
            walker1.upper_arm_len = upper_arm_len;
            walker1.init_foot_positions();
            walker1.speed = speed;
            walker1.color = color;
            return walker1;
        }
    }
}

fn draw_robot_arm(
    position: [f32; 2],
    target: [f32; 2],
    forarm_len: f32,
    upper_arm_len: f32,
    solution: [f32; 2],
    color: macroquad::color::Color,
) {
    let ([joint_x, joint_y], [hand_x, hand_y], _works) = inverse_cinematics::inverse_cinematics(
        position,
        target,
        forarm_len,
        upper_arm_len,
        solution,
    );
    macroquad::shapes::draw_line(position[0], position[1], joint_x, joint_y, 5.0, color);
    macroquad::shapes::draw_line(joint_x, joint_y, hand_x, hand_y, 3.0, color);
}

fn animated_connecting_rod(
    position: [f32; 2],
    rotating_axule_offset: [f32; 2],
    rotating_axule_radius: f32,
    time: f32,
    time_multiplier: f32,
    forarm_len: f32,
    upper_arm_len: f32,
    solution: [f32; 2],
    phase: f32,
) -> ([f32; 2], [f32; 2], bool) {
    return inverse_cinematics::inverse_cinematics(
        position,
        [
            position[0]
                + rotating_axule_offset[0]
                + rotating_axule_radius * (time * time_multiplier + phase).cos(),
            position[1]
                + rotating_axule_offset[1]
                + rotating_axule_radius * (time * time_multiplier + phase).sin(),
        ],
        forarm_len,
        upper_arm_len,
        solution,
    );
}

fn draw_animated_connecting_rod(
    position: [f32; 2],
    rotating_axule_offset: [f32; 2],
    rotating_axule_radius: f32,
    time: f32,
    time_multiplier: f32,
    forarm_len: f32,
    upper_arm_len: f32,
    solution: [f32; 2],
    phase: f32,
) {
    let ([joint_x, joint_y], [hand_x, hand_y], _works) = animated_connecting_rod(
        position,
        rotating_axule_offset,
        rotating_axule_radius,
        time,
        time_multiplier,
        forarm_len,
        upper_arm_len,
        solution,
        phase,
    );

    macroquad::shapes::draw_line(
        position[0],
        position[1],
        joint_x,
        joint_y,
        3.0,
        macroquad::prelude::WHITE,
    );
    macroquad::shapes::draw_line(
        joint_x,
        joint_y,
        hand_x,
        hand_y,
        1.0,
        macroquad::prelude::WHITE,
    );
}

#[macroquad::main("Animation Tests")]
async fn main() {
    let mut walk = walker::Walker1::new(
        [100.0, 50.0],
        [1.0, 0.0],
        20.0,
        20.0,
        20.0,
        20.0,
        macroquad::prelude::ORANGE,
    );
    let mut walk_group = walker::WalkerGroup::randomly_generate(
        50,
        [
            0.0,
            0.0,
            macroquad::window::screen_width(),
            macroquad::window::screen_height(),
        ],
        [0.5, 2.0],
    );
    loop {
        let (mouse_x, mouse_y) = macroquad::input::mouse_position();
        macroquad::window::clear_background(macroquad::color::BLACK);
        walk_group.draw_all();
        walk_group.update_all(macroquad::time::get_frame_time());
        draw_robot_arm(
            [100.0; 2],
            [mouse_x, mouse_y],
            60.0,
            30.0,
            [0.0, 1.0],
            macroquad::prelude::RED,
        );
        draw_robot_arm(
            [100.0; 2],
            [mouse_x, mouse_y],
            60.0,
            30.0,
            [0.0, -1.0],
            macroquad::prelude::BLUE,
        );
        draw_animated_connecting_rod(
            [250.0; 2],
            [25.0, 25.0],
            10.0,
            macroquad::time::get_time() as f32,
            10.0,
            40.0,
            20.0,
            [1.0, 0.0],
            0.0,
        );
        draw_animated_connecting_rod(
            [250.0; 2],
            [25.0, -25.0],
            10.0,
            macroquad::time::get_time() as f32,
            -10.0,
            40.0,
            20.0,
            [-1.0, 0.0],
            3.14 / 10.0 * 0.5,
        );
        walk.move_forward(macroquad::time::get_frame_time());
        draw_animated_connecting_rod(
            [290.0; 2],
            [25.0, 25.0],
            10.0,
            macroquad::time::get_time() as f32,
            10.0,
            40.0,
            20.0,
            [0.0, 1.0],
            0.0,
        );
        draw_animated_connecting_rod(
            [290.0; 2],
            [-25.0, -25.0],
            10.0,
            macroquad::time::get_time() as f32,
            -10.0,
            40.0,
            20.0,
            [0.0, -1.0],
            3.14 / 10.0 * 0.5,
        );

        walk.draw();

        macroquad::window::next_frame().await;
    }
}
