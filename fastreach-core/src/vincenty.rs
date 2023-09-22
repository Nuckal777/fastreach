use geo::{Coord, HaversineDestination, Point};

/// takes a (lon, lat) and returns a Vec<(lon, lat)>.
/// distance in meters.
#[must_use]
pub fn spherical_circle(point: Coord, vertecies: usize, distance: f64) -> Vec<Coord> {
    let step = 360.0 / vertecies as f64;
    let mut points = Vec::<Coord>::with_capacity(vertecies);
    for i in 0..vertecies {
        let angle = step * i as f64;
        points.push(
            Point::from(point)
                .haversine_destination(angle, distance)
                .into(),
        );
    }
    points
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;
    use geo::{HaversineDestination, Point};
    #[test]
    fn spherical_walk() {
        let start = (13.734_617_f64, 51.050_94_f64);
        let dest = Point::from(start).haversine_destination(0.0, 500.0);
        assert_abs_diff_eq!(dest.x(), 13.734_617, epsilon = 0.00001);
        assert_abs_diff_eq!(dest.y(), 51.055_437, epsilon = 0.00001);
    }
}
