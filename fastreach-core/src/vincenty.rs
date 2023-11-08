use geo::{Coord, GeoFloat, HaversineDestination, Point};
use num_traits::FromPrimitive;

/// takes a (lon, lat) and returns a Vec<(lon, lat)>.
/// distance in meters.
#[must_use]
pub fn spherical_circle<T: GeoFloat + FromPrimitive>(
    point: Coord<T>,
    vertecies: usize,
    distance: T,
) -> Vec<Coord<T>> {
    let step = num_traits::cast::<f32, T>(360.0).unwrap() / num_traits::cast(vertecies).unwrap();
    let mut points = Vec::<Coord<T>>::with_capacity(vertecies);
    for i in 0..vertecies {
        let angle = step * num_traits::cast(i).unwrap();
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
