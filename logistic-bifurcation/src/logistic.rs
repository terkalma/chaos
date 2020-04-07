pub mod equation {
  use std::collections::VecDeque;

  pub fn next(a: f32, x_n_1: f32) -> f32 {
    a * x_n_1 * (1.0 - x_n_1)
  }

  pub fn cycle(a: f32, x_0: f32) -> VecDeque<f32> {
    let capacity = 2_usize.pow(5);
    let mut x = next(a, x_0);

    let mut v: VecDeque<f32> = VecDeque::with_capacity(capacity);

    for i in 0..(capacity * 2) {
      if i > capacity {
        v.pop_front();
      }

      v.push_back(x);
      x = next(a, x);
    }

    v
  }
}

