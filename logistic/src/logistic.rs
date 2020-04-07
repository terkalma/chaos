pub mod equation {
  pub fn next(a: f32, x_n_1: f32) -> f32 {
    a * x_n_1 * (1.0 - x_n_1)
  }
}

