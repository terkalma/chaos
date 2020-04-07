pub mod numeric {

  pub type Point = (f32, f32);
  pub type Map = fn(x: f32, y: f32) -> f32;

  pub fn euler(f: Map, g: Map, h: f32, x_0: f32, y_0: f32, n: usize) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::with_capacity(n);

    result.push((x_0, y_0));

    let mut p = (x_0, y_0);

    for _i in 1..n {
      let x_i = p.0 + h * f(p.0, p.1);
      let y_i = p.1 + h * g(p.0, p.1);

      result.push((x_i, y_i));

      p = (x_i, y_i);
    }

    result
  }

  pub fn leap_frog(f: Map, g: Map, h: f32, x_0: f32, y_0: f32, n: usize) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::with_capacity(n);

    result.push((x_0, y_0));

    let mut p = (x_0, y_0);

    for _i in 1..n {
      let x_i = p.0 + h * f(p.0, p.1);
      let y_i = p.1 + h * g(x_i, p.1);

      result.push((x_i, y_i));

      p = (x_i, y_i);
    }

    result
  }

  pub fn runge_kutta(f: Map, g: Map, h: f32, x_0: f32, y_0: f32, n: usize) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::with_capacity(n);

    result.push((x_0, y_0));

    let mut p = (x_0, y_0);

    for _i in 1..n {
      let k1x = h * f(p.0, p.1);
      let k1y = h * g(p.0, p.1);
      let k2x = h * f(p.0 + k1x / 2.0, p.1 + k1y);
      let k2y = h * g(p.0 + k1x / 2.0, p.1 + k1y);

      let x_i = p.0 + k2x;
      let y_i = p.1 + k2y;

      result.push((x_i, y_i));

      p = (x_i, y_i);
    }

    result
  }
}
