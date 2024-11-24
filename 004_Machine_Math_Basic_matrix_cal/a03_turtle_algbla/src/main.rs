use std::f64::consts::PI;
use turtle::{Color, Turtle};

fn interpolate(start: f64, end: f64, t: f64) -> f64 {
    start + (end - start) * t
}

fn sigmoid(t: f64) -> f64 {
    1.0 / (1.0 + (-t).exp())
}

fn homotopy(x: f64, y: f64, t: f64) -> (f64, f64) {
    const SPACE_WIDTH: f64 = 10.0;

    let norm = (x * x + y * y).sqrt();
    let tau = interpolate(5.0, -5.0, t) + norm / SPACE_WIDTH;
    let alpha = sigmoid(tau);
    (x, y + 0.5 * (2.0 * PI * alpha).sin())
}

fn main() {
    const RUN_TIME: f64 = 3.0;
    const FRAME_RATE: u32 = 60; // Frames per second
    const FRAME_COUNT: usize = (RUN_TIME * FRAME_RATE as f64) as usize;

    let mut turtle = Turtle::new();
    turtle.set_speed("instant"); // Fastest drawing speed

    // Set up window properties
    turtle.drawing_mut().set_background_color("black");
    turtle.set_pen_color(Color::WHITE);
    turtle.pen_up();

    // Define grid size
    let grid_size = 20;
    let step = 20.0; // Spacing between points

    for frame in 0..FRAME_COUNT {
        let t = frame as f64 / FRAME_COUNT as f64;

        // Clear the screen at each frame
        turtle.clear();

        for i in 0..grid_size {
            for j in 0..grid_size {
                let x = -200.0 + i as f64 * step;
                let y = -200.0 + j as f64 * step;

                let (hx, hy) = homotopy(x / 100.0, y / 100.0, t);

                // Map coordinates to screen space and draw the point
                turtle.go_to([hx as f64 * 100.0, hy as f64 * 100.0].into());
                turtle.dot(3.0);
            }
        }

        // Update frame for the animation
        turtle.sleep(1.0 / FRAME_RATE as f64);
    }
}
