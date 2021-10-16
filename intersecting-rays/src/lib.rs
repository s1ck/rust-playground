#[derive(PartialEq, Debug)]
struct Point(f32, f32);

#[derive(Debug)]
struct Line(Point, Point);

impl Line {
    fn intersection(&self, other: &Line) -> Option<Point> {
        fn determinant(a: &Point, b: &Point) -> f32 {
            a.0 * b.1 - a.1 * b.0
        }

        let x_diff = Point(self.0 .0 - self.1 .0, other.0 .0 - other.1 .0);
        let y_diff = Point(self.0 .1 - self.1 .1, other.0 .1 - other.1 .1);

        let det = determinant(&x_diff, &y_diff);

        if det == 0_f32 {
            return None;
        }

        let d = Point(
            determinant(&self.0, &self.1),
            determinant(&other.0, &other.1),
        );

        let x = determinant(&d, &x_diff) / det;
        let y = determinant(&d, &y_diff) / det;

        Some(Point(x, y))
    }
}

#[test]
fn test_intersection() {
    let line1 = Line(Point(0.0, 0.0), Point(4.0, 4.0));
    let line2 = Line(Point(0.0, 4.0), Point(4.0, 0.0));

    assert_eq!(line1.intersection(&line2).unwrap(), Point(2.0, 2.0))
}
