const DIVISIONS: usize = 10;
const NUMBER_DISKS: usize = 40;
const SIMULATION_SPEED: f64 = 30.0;

struct Disk {
    velocity: [f64; 2],
    position: [f64; 2],
    radius: f64,
    mass: f64,
}

//square: [x, y, size]
fn point_in_square(square: [f64; 3], point: [f64; 2]) -> bool {
    square[0] < point[0]
        && square[1] < point[1]
        && square[0] + square[2] > point[0]
        && square[1] + square[2] > point[1]
}
// [x, y, width, height]
fn square_collision(square1: [f64; 3], square2: [f64; 3]) -> bool {
    for (xf, yf) in [(0, 0), (0, 1), (1, 0), (1, 1)].iter() {
        let xf = *xf as f64;
        let yf = *yf as f64;
        let point1 = [square1[0] + xf * square1[2], square1[1] + yf * square1[2]];
        let point2 = [square2[0] + xf * square2[2], square2[1] + yf * square2[2]];
        if point_in_square(square2, point1) || point_in_square(square1, point2) {
            return true;
        }
    }
    return false;
}

impl Disk {
    fn my_square(&self) -> [f64; 3] {
        [
            self.position[0] - self.radius,
            self.position[1] - self.radius,
            self.radius * 2.0,
        ]
    }

    fn resolve_collision(&mut self, other: &mut Disk) {
        let inbetween = [
            self.position[0] - other.position[0],
            self.position[1] - other.position[1],
        ];
        let inbetween_lenght = (inbetween[0] * inbetween[0] + inbetween[1] * inbetween[1]).sqrt();
        let inbetween_normalized = [
            inbetween[0] / inbetween_lenght,
            inbetween[1] / inbetween_lenght,
        ];
        let minimum_distance = other.radius + self.radius;
        let overlapping_distance = minimum_distance - inbetween_lenght;

        other.position[0] -= inbetween_normalized[0] * overlapping_distance * 0.5;
        other.position[1] -= inbetween_normalized[1] * overlapping_distance * 0.5;

        self.position[0] += inbetween_normalized[0] * overlapping_distance * 0.5;
        self.position[1] += inbetween_normalized[1] * overlapping_distance * 0.5;
    }

    fn elastic_collision(&mut self, other: &mut Disk) {
        let self_velocity = self.velocity;
        //source: https://www.khanacademy.org/science/physics/linear-momentum/elastic-and-inelastic-collisions/a/what-are-elastic-and-inelastic-collisions
        self.velocity[0] = (self.mass - other.mass) / (self.mass + other.mass) * self.velocity[0]
            + (2.0 * other.mass) / (self.mass + other.mass) * other.velocity[0];
        self.velocity[1] = (self.mass - other.mass) / (self.mass + other.mass) * self.velocity[1]
            + (2.0 * other.mass) / (self.mass + other.mass) * other.velocity[1];

        other.velocity[0] = (other.mass - self.mass) / (self.mass + other.mass) * other.velocity[0]
            + (2.0 * self.mass) / (self.mass + other.mass) * self_velocity[0];
        other.velocity[1] = (other.mass - self.mass) / (self.mass + other.mass) * other.velocity[1]
            + (2.0 * self.mass) / (self.mass + other.mass) * self_velocity[1];
    }

    fn collision(&self, other: &Disk) -> bool {
        //first: check if those squares collide:
        let my_square = self.my_square();
        let other_square = other.my_square();
        if square_collision(my_square, other_square) {
            let difference_vec = [
                self.position[0] - other.position[0],
                self.position[1] - other.position[1],
            ];
            let distance_squared =
                difference_vec[0] * difference_vec[0] + difference_vec[1] * difference_vec[1];
            let minimum_distance = self.radius + other.radius;
            let minimum_distance_squared = minimum_distance * minimum_distance;
            return minimum_distance_squared > distance_squared;
        }
        return false;
    }

    //the simulation happens inside a contained rectangular space. the upper left corner of the simulation space is at (0.0, 0.0)
    fn border_bounds(&mut self, width: f64, height: f64) {
        //left vertical wall:
        if self.position[0] - self.radius < 0.0 {
            self.velocity = [-self.velocity[0], self.velocity[1]];
            self.position[0] = 0.0 + self.radius;
        }
        //right vertical wall:
        else if self.position[0] + self.radius > width {
            self.velocity = [-self.velocity[0], self.velocity[1]];
            self.position[0] = width - self.radius;
        }
        //upper horizontal wall
        if self.position[1] - self.radius < 0.0 {
            self.velocity = [self.velocity[0], -self.velocity[1]];
            self.position[1] = 0.0 + self.radius;
        }
        //lower horizontal wall
        else if self.position[1] + self.radius > height {
            self.velocity = [self.velocity[0], -self.velocity[1]];
            self.position[1] = height - self.radius;
        }
    }

    fn apply_velocity(&mut self, delta_time: f64) {
        self.position[0] += self.velocity[0] * delta_time;
        self.position[1] += self.velocity[1] * delta_time;
    }

    fn render(&self) {
        macroquad::shapes::draw_circle(
            self.position[0] as f32,
            self.position[1] as f32,
            self.radius as f32,
            macroquad::color::WHITE,
        );
    }
}

fn generate_disks_randomly(seed: u64) -> Vec<Disk> {
    macroquad::rand::srand(seed);
    let mut positions = [(0, 0); DIVISIONS * DIVISIONS];
    let mut i = 0;
    for y in 0..DIVISIONS {
        for x in 0..DIVISIONS {
            positions[i] = (x, y);
            i += 1;
        }
    }
    //shuffle the positions lists:
    for _i in 0..positions.len() {
        let i1 = macroquad::rand::gen_range(0, positions.len());
        let i2 = macroquad::rand::gen_range(0, positions.len());
        if i1 != i2 {
            positions.swap(i1, i2);
        }
    }
    let width = macroquad::window::screen_width() / DIVISIONS as f32;
    let height = macroquad::window::screen_height() / DIVISIONS as f32;
    let square_size = width.min(height);
    let minimum_radius = square_size * 0.1;
    let mut disks = Vec::with_capacity(NUMBER_DISKS);
    //generate the random disks:
    for i in 0..NUMBER_DISKS {
        let (x, y) = positions[i];
        //convert it to center coordinates
        let x = x as f32 * square_size + square_size * 0.5;
        let y = y as f32 * square_size + square_size * 0.5;
        let x = x as f64;
        let y = y as f64;
        let radius = macroquad::rand::gen_range(minimum_radius, square_size * 0.5) as f64;
        let vx = macroquad::rand::gen_range(-1.0, 1.0) * SIMULATION_SPEED;
        let vy = macroquad::rand::gen_range(-1.0, 1.0) * SIMULATION_SPEED;
        let mass = std::f64::consts::PI * radius * radius;
        disks.push(Disk {
            velocity: [vx, vy],
            position: [x, y],
            radius,
            mass,
        });
    }
    return disks;
}

#[macroquad::main("BasicShapes")]
async fn main() {
    assert!(DIVISIONS * DIVISIONS > NUMBER_DISKS);
    let mut seed_count = 0;
    let mut disks = generate_disks_randomly(seed_count);

    loop {
        macroquad::window::clear_background(macroquad::color::BLACK);
        let delta_time = macroquad::time::get_frame_time() as f64;
        let width = macroquad::window::screen_width() as f64;
        let height = macroquad::window::screen_height() as f64;
        //maybe split_at_mut would be a better solution to get 2 mutable references.... but i try this as an experiment:
        for i in 0..NUMBER_DISKS {
            let mut mutable_iterator = disks.iter_mut().skip(i);
            if let Some(disk1) = mutable_iterator.next() {
                disk1.apply_velocity(delta_time);
                disk1.border_bounds(width, height);
                disk1.render();
                while let Some(disk2) = mutable_iterator.next() {
                    if disk1.collision(disk2) {
                        disk1.resolve_collision(disk2);
                        disk1.elastic_collision(disk2);
                    }
                }
            }
        }

        if macroquad::input::is_mouse_button_down(macroquad::input::MouseButton::Left) {
            disks = generate_disks_randomly(seed_count);
            seed_count = seed_count.wrapping_add(1);
        }

        //calculate overall impulse and kinetic energy:
        let mut impulse_total = 0.0;
        let mut kinetic_total = 0.0;
        for i in disks.iter() {
            let mass = i.mass;
            let [x, y] = i.velocity;
            let speed_squared = x * x + y * y;
            let speed = speed_squared.sqrt();
            impulse_total += mass * speed;
            kinetic_total += 0.5 * mass * speed_squared;
        }
        macroquad::text::draw_text(
            &format!(
                "impulse: {:.3}, E_kin: {:.10}",
                impulse_total, kinetic_total
            ),
            0.0,
            0.0,
            20.0,
            macroquad::color::Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 0.5,
            },
        );
        macroquad::window::next_frame().await
    }
}
