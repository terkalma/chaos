mod logistic;

use std::collections::VecDeque;
use nannou::prelude::*;
pub use crate::logistic::equation;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    v: VecDeque<f32>,
    x_0: f32,
    a: f32
}

fn model(_app: &App) -> Model {
    let a: f32 = 2.5;
    let x_0: f32 = 0.2;

    Model {
        a,
        x_0,
        v: equation::cycle(a, x_0)
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.a < 4.0 {
        model.a = model.a + 0.001;
    }

    model.v = equation::cycle(model.a, model.x_0);
}

fn view(app: &App, model: &Model, frame: &Frame){
    let draw = app.draw();
    let x = model.a * 300.0 - 1000.0;

    for y in model.v.iter() {
        draw.rect()
            .x_y(x, y * 500.0 - 300.0)
            .w_h(1.0, 1.0)
            .color(RED);
    }

    draw.to_frame(app, &frame).unwrap();

}