mod logistic;

use nannou::prelude::*;
pub use crate::logistic::equation;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    x_n_1: f32,
    x_n: f32,
    a: f32
}

fn model(_app: &App) -> Model {
    let a: f32 = 3.569;
    let x: f32 = 0.2;

    Model {
        a,
        x_n_1: x,
        x_n: equation::next(a, x)
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.x_n_1 = model.x_n;
    model.x_n = equation::next(model.a, model.x_n);
}

fn view(app: &App, model: &Model, frame: &Frame){
    let draw = app.draw();
    let win_rect = app.main_window().rect();

    draw.rect()
        .rgba(0.0, 0.0, 0.0, 0.05)
        .w_h(win_rect.w(), win_rect.h());
    draw.ellipse()
        .x_y(model.x_n_1 * 200.0, model.x_n * 200.0)
        .radius(2.0)
        .color(RED);

    draw.to_frame(app, &frame).unwrap();

}