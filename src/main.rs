trait MetricSpace {
  type Distance;
  fn distance(&self, other: &Self) -> Self::Distance;
}

struct Point {
  x: f64,
  y: f64,
}

impl MetricSpace for Point {
  type Distance = f64;

  /// Compute the euclidean distance between two points in 2D space.
  fn distance(&self, other: &Self) -> Self::Distance {
    ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
  }
}

fn main() {
  println!("Hello, world!");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn distance_between_separate_points() {
    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 3.0, y: 4.0 };
    assert_eq!(p1.distance(&p2), 5.0);
  }

  #[test]
  fn distance_at_same_point_is_zero() {
    let p1 = Point { x: 3.0, y: 3.0 };
    let p2 = Point { x: 3.0, y: 3.0 };
    assert_eq!(p1.distance(&p2), 0.0);
  }

  #[test]
  fn starting_point_doesnt_matter() {
    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 3.0, y: 4.0 };
    assert_eq!(p2.distance(&p1), p1.distance(&p2));
  }
}
