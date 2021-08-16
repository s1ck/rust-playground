#[derive(Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq)]
struct Polar {
    r: f64,
    phi: f64,
}

impl Polar {
    fn new(r: f64, phi: f64) -> Self {
        Self { r, phi }
    }
}

impl From<Polar> for Point {
    fn from(p: Polar) -> Self {
        let x = p.r * p.phi.cos();
        let y = p.r * p.phi.sin();
        Self::new(x, y)
    }
}

impl From<Point> for Polar {
    fn from(p: Point) -> Self {
        let r = (p.x.square() + p.y.square()).sqrt();
        let phi = p.y.atan2(p.x);
        Self::new(r, phi)
    }
}

trait Square {
    fn square(&self) -> Self;
}

impl Square for f64 {
    fn square(&self) -> Self {
        self.powf(2.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_point() {
        let point = Point::new(3.0, 4.0);
        let polar: Polar = point.into();
        assert_eq!(polar, Polar::new(5.0, 0.9272952180016122));
    }

    #[test]
    fn from_polar() {
        let polar = Polar::new(5.0, 0.9272952180016122);
        let point: Point = polar.into();
        assert_eq!(point, Point::new(3.0000000000000004, 3.9999999999999996));
    }
}
