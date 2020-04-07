use bracket_lib::{line_2d, line_2d_bresenham}

pub fn line_to(orig: Point, dest: Point, bresenham: bool) -> Vec<Point> {
    let points;
    if bresenham { points = line_2d_bresenham(orig, dest) }
}
