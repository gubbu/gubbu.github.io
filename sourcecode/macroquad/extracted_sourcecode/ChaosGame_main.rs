use macroquad::hash;

const N: usize = 7000;

//one rule has the following format: (propability, new_x = [x_factor, y_factor, constant], new_y (same format as new_x))
type RuleSet = [(f32, [f32; 3], [f32; 3])];

const CORAL_RULESET: &RuleSet = &[
    (0.4, [0.31f32, -0.53, 0.89], [-0.46f32, -0.29, 1.10]),
    (0.15, [0.31, -0.08, 0.22], [0.15, -0.45, 0.34]),
    (0.45, [0.0, 0.55, 0.01], [0.69, -0.2, 0.38]),
];

const SIERPINSKI_TRI_RULESET: &RuleSet = &[
    (1.0 / 3.0, [0.5, 0.0, 0.0], [0.0, 0.5, 0.0]), //average with vertex [0f32; 2]
    (1.0 / 3.0, [0.5, 0.0, 0.5 * 1.0], [0.0, 0.5, 0.0]), //average with vertex [1.0, 0.0],
    (
        1.0 / 3.0,
        [0.5, 0.0, 0.5 * 0.5],
        [0.0, 0.5, 0.5 * 0.5 * 1.7320508075688772],
    ), //average with vertex [0.5, 0.5 * 3f32.sqrt()]
];

const FERN_RULESET: &RuleSet = &[
    (0.02, [0.0, 0.0, 0.5], [0.0, 0.27, 0.0]),
    (0.15, [-0.14, 0.26, 0.57], [0.25, 0.22, -0.04]),
    (0.13, [0.17, -0.21, 0.41], [0.22, 0.18, 0.09]),
    (0.7, [0.78, 0.03, 0.11], [-0.03, 0.74, 0.27]),
];

//Barnsley fern https://en.wikipedia.org/wiki/Barnsley_fern
//Note that the complete fern is within the range −2.1820 < x < 2.6558 and 0 ≤ y < 9.9983
const RESCALE_Y: f32 = 1.0 / 9.9983;
const BARNSLEY: &RuleSet = &[
    (0.01, [0.0, 0.0, 0.0], [0.0, 0.16, 0.0]),
    (0.85, [0.85, 0.04, 0.0], [-0.04, 0.85, 1.6 * RESCALE_Y]),
    (0.07, [0.2, -0.26, 0.0], [0.23, 0.22, 1.6 * RESCALE_Y]),
    (0.07, [-0.15, 0.28, 0.0], [0.26, 0.24, 0.44 * RESCALE_Y]),
];

const VICSEK_FRACTAL: &RuleSet = &[
    (0.2, [1.0 / 3.0, 0.0, 0.0], [0.0, 1.0 / 3.0, 0.0]), //point 0.0, 0.0
    (0.2, [1.0 / 3.0, 0.0, 2.0 / 3.0], [0.0, 1.0 / 3.0, 0.0]), //point 1.0, 0.0
    (0.2, [1.0 / 3.0, 0.0, 0.0], [0.0, 1.0 / 3.0, 2.0 / 3.0]), //point 0.0, 1.0
    (
        0.2,
        [1.0 / 3.0, 0.0, 2.0 / 3.0],
        [0.0, 1.0 / 3.0, 2.0 / 3.0],
    ), //point 1.0, 1.0
    (
        0.2,
        [1.0 / 3.0, 0.0, 2.0 / 3.0 * 0.5],
        [0.0, 1.0 / 3.0, 2.0 / 3.0 * 0.5],
    ), //point 0.5, 0.5
];

const SIRPINSKI_CARPET: &RuleSet = &[
    (0.125, [1.0 / 3.0, 0.0, 0.0], [0.0, 1.0 / 3.0, 0.0]), //point 0.0, 0.0
    (0.125, [1.0 / 3.0, 0.0, 2.0 / 3.0], [0.0, 1.0 / 3.0, 0.0]), //point 1.0, 0.0
    (0.125, [1.0 / 3.0, 0.0, 0.0], [0.0, 1.0 / 3.0, 2.0 / 3.0]), //point 0.0, 1.0
    (
        0.125,
        [1.0 / 3.0, 0.0, 2.0 / 3.0],
        [0.0, 1.0 / 3.0, 2.0 / 3.0],
    ), //point 1.0, 1.0
    (
        0.125,
        [1.0 / 3.0, 0.0, 2.0 / 3.0 * 0.5],
        [0.0, 1.0 / 3.0, 0.0], //jump to point 0.5, 0.0
    ),
    (
        0.125,
        [1.0 / 3.0, 0.0, 2.0 / 3.0 * 0.5],
        [0.0, 1.0 / 3.0, 2.0 / 3.0], //jump to point 0.5, 1.0
    ),
    (
        0.125,
        [1.0 / 3.0, 0.0, 0.0],
        [0.0, 1.0 / 3.0, 2.0 / 3.0 * 0.5], //jump to point 0.0, 0.5
    ),
    (
        0.125,
        [1.0 / 3.0, 0.0, 2.0 / 3.0],
        [0.0, 1.0 / 3.0, 2.0 / 3.0 * 0.5], //jump to point 1.0, 0.5
    ),
];

const CHAOS_GAMES: &[(&str, &RuleSet)] = &[
    ("Fern", FERN_RULESET),
    ("Coral", CORAL_RULESET),
    ("Sierpinski Triangle", SIERPINSKI_TRI_RULESET),
    ("Vicsek Fractal", VICSEK_FRACTAL),
    ("Sierpinski Carpet", SIRPINSKI_CARPET),
    ("Barnsley Fern", BARNSLEY),
];

fn apply_ruleset_on_point(ruleset: &RuleSet, point: [f32; 2]) -> [f32; 2] {
    let random_number = macroquad::rand::gen_range(0.0, 1.1);
    let mut sum = 0.0;
    for rule in ruleset {
        sum += rule.0;
        if random_number <= sum {
            let [old_x, old_y] = point;
            let new_point = [
                rule.1[0] * old_x + rule.1[1] * old_y + rule.1[2],
                rule.2[0] * old_x + rule.2[1] * old_y + rule.2[2],
            ];
            return new_point;
        }
    }
    let rule = ruleset[ruleset.len() - 1];
    let [old_x, old_y] = point;
    return [
        rule.1[0] * old_x + rule.1[1] * old_y + rule.1[2],
        rule.2[0] * old_x + rule.2[1] * old_y + rule.2[2],
    ];
}

fn generate_with_ruleset(point_list: &mut [[f32; 2]; N], ruleset: &RuleSet) {
    let mut current_x: f32 = 0f32;
    let mut current_y: f32 = 0f32;
    for i in 0..N {
        let current_point = apply_ruleset_on_point(ruleset, [current_x, current_y]);
        current_x = current_point[0];
        current_y = current_point[1];
        point_list[i] = [current_x, current_y];
    }
}

#[macroquad::main("ChaosGame")]
async fn main() {
    let game_names: Vec<&str> = CHAOS_GAMES.iter().map(|&chaos_game| chaos_game.0).collect();

    let mut point_list = [[0f32; 2]; N];
    generate_with_ruleset(&mut point_list, FERN_RULESET);

    let mut current_selection = 0;
    let mut scale: f32 = 1.0;
    loop {
        macroquad::window::clear_background(macroquad::color::BLACK);
        macroquad::ui::widgets::Window::new(
            hash!(),
            macroquad::math::vec2(0.0, 0.0),
            macroquad::math::vec2(420.0, 75.0),
        )
        .label("Selct Chaos Game")
        .ui(&mut macroquad::ui::root_ui(), |ui| {
            let selected_before = current_selection;

            ui.combo_box(
                hash!(),
                "select chaos game",
                &game_names,
                &mut current_selection,
            );
            ui.slider(hash!(), "scale", 0.0..2.0, &mut scale);
            if selected_before != current_selection {
                println!("selection changed! to {}", CHAOS_GAMES[current_selection].0);
                point_list = [[0.0; 2]; N];
                generate_with_ruleset(&mut point_list, CHAOS_GAMES[current_selection].1);
            }
        });
        let screen_width = macroquad::window::screen_width();
        let screen_height = macroquad::window::screen_height();
        let add_x = screen_width * (1.0 - scale) * 0.5;
        let add_y = screen_height * (1.0 - scale) * 0.5;
        for point in point_list.iter() {
            let scale_x = point[0] * screen_width * scale + add_x;
            let scale_y = point[1] * screen_height * scale + add_y;
            macroquad::shapes::draw_circle(scale_x, scale_y, 1.0, macroquad::color::WHITE);
        }
        macroquad::window::next_frame().await;
    }
}
