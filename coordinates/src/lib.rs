#![allow(mixed_script_confusables)]

trait Float: Copy + std::ops::Add<Output = Self> + std::ops::Mul<Output = Self> {
    fn square(&self) -> Self;
    fn atan(&self) -> Self;
    fn atan2(&self, other: Self) -> Self;
    fn cos(&self) -> Self;
    fn sin(&self) -> Self;
    fn sqrt(&self) -> Self;
}

impl Float for f32 {
    fn square(&self) -> Self {
        f32::powf(*self, 2.0)
    }

    fn atan(&self) -> Self {
        f32::atan(*self)
    }

    fn atan2(&self, other: Self) -> Self {
        f32::atan2(*self, other)
    }

    fn cos(&self) -> Self {
        f32::cos(*self)
    }

    fn sin(&self) -> Self {
        f32::sin(*self)
    }

    fn sqrt(&self) -> Self {
        f32::sqrt(*self)
    }
}

impl Float for f64 {
    fn square(&self) -> Self {
        f64::powf(*self, 2.0)
    }

    fn atan(&self) -> Self {
        f64::atan(*self)
    }

    fn atan2(&self, other: Self) -> Self {
        f64::atan2(*self, other)
    }

    fn cos(&self) -> Self {
        f64::cos(*self)
    }

    fn sin(&self) -> Self {
        f64::sin(*self)
    }

    fn sqrt(&self) -> Self {
        f64::sqrt(*self)
    }
}

#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T: Float> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq)]
struct Polar<T> {
    r: T,
    φ: T,
}

impl<T: Float> Polar<T> {
    fn new(r: T, φ: T) -> Self {
        Self { r, φ }
    }
}

impl<T: Float> From<Polar<T>> for Point<T> {
    fn from(Polar { r, φ }: Polar<T>) -> Self {
        let x = r * φ.cos();
        let y = r * φ.sin();
        Self::new(x, y)
    }
}

impl<T: Float> From<Point<T>> for Polar<T> {
    fn from(Point { x, y }: Point<T>) -> Self {
        let r = (x.square() + y.square()).sqrt();
        let φ = y.atan2(x);
        Self::new(r, φ)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_point_f64() {
        let point = Point::new(3.0, 4.0);
        let polar: Polar<_> = point.into();
        assert_eq!(polar, Polar::new(5.0, 0.9272952180016122));
    }

    #[test]
    fn from_polar_f64() {
        let polar = Polar::new(5.0, 0.9272952180016122);
        let point: Point<_> = polar.into();
        assert_eq!(point, Point::new(3.0000000000000004, 3.9999999999999996));
    }

    #[test]
    fn from_point_f32() {
        let point = Point::new(3.0_f32, 4.0_f32);
        let polar: Polar<_> = point.into();
        assert_eq!(polar, Polar::new(5.0, 0.9272952180016122));
    }

    #[test]
    fn from_polar_f32() {
        let polar = Polar::new(5.0_f32, 0.9272952180016122_f32);
        let point: Point<_> = polar.into();
        assert_eq!(point, Point::new(3.0000000000000004, 3.9999999999999996));
    }
}
