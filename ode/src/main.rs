mod ode;

use nannou::prelude::*;
pub use crate::ode::numeric::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    points: Vec<Point>
}

fn f(_x: f32, y: f32) -> f32 {
    y
}

fn g(x: f32, y: f32) -> f32 {
    - 0.5 * (x * x - 1.0) * y - x
}

fn van_der_pol() -> Vec<Point> {
    euler(f, g, 0.5, 2.0, 2.0, 10000)
}

fn model(_app: &App) -> Model {

    Model {
        points: van_der_pol()
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {

}

fn view(app: &App, model: &Model, frame: &Frame){
    let draw = app.draw();

    for x in model.points.iter() {
        draw.rect()
            .x_y(x.0 * 100.0, x.1 * 100.0)
            .w_h(1.0, 1.0)
            .color(RED);
    }

    draw.to_frame(app, &frame).unwrap();
}