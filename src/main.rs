use nannou::{noise::NoiseFn, noise::Perlin, prelude::*};

const WIDTH: i32 = 500;
const HEIGHT: i32 = 500;
const RADIUS: f32 = 30.0;
const SHIFT: f32 = 5.0;
const STROKE_WEIGHT: f32 = 0.5;
const STROKE_SCALAR: f32 = 1.2;
const NUM_CIRCLES: u32 = 100;
const NOISE_SCALE: f32 = 0.4;
const NOISE_AMPLITUDE: f32 = 20.0;

fn main() {
    nannou::sketch(view)
        .size(WIDTH as u32, HEIGHT as u32)
        .loop_mode(LoopMode::loop_once())
        .run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let perlin = Perlin::new();

    for _c in 0..NUM_CIRCLES {
        let x = random_range(-WIDTH / 2, WIDTH / 2) as f32;
        let y = random_range(-HEIGHT / 2, HEIGHT / 2) as f32;
        let w = random_range(STROKE_WEIGHT, STROKE_WEIGHT * STROKE_SCALAR);

        let noise_x = x * NOISE_SCALE;
        let noise_y = y * NOISE_SCALE;

        let noise_value = perlin.get([noise_x as f64, noise_y as f64]) as f32;
        let noise_offset = NOISE_AMPLITUDE * noise_value;
        let size = RADIUS + noise_offset;

        draw.ellipse()
            .x_y(x, y)
            .radius(size)
            .stroke_weight(w)
            .stroke_color(GOLD)
            .no_fill();

        draw.ellipse()
            .x_y(x + SHIFT, y)
            .radius(size)
            .stroke_weight(w)
            .stroke_color(AZURE)
            .no_fill();

        draw.ellipse()
            .x_y(x - SHIFT, y)
            .radius(size)
            .stroke_weight(w)
            .stroke_color(GREY)
            .no_fill();

        draw.ellipse()
            .x_y(x, y - SHIFT)
            .radius(size)
            .stroke_weight(w)
            .stroke_color(MEDIUMSLATEBLUE)
            .no_fill();
    }

    draw.to_frame(app, &frame).unwrap();
}
