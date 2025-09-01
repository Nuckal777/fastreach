use geo::{Coord, Destination, GeoFloat, Haversine, Point};
use num_traits::FromPrimitive;

/// takes a (lon, lat) and returns a Vec<(lon, lat)>.
/// distance in meters.
/// # Panics
/// When T cannot be cast to f32.
#[must_use]
pub fn spherical_circle<T: GeoFloat + FromPrimitive>(
    point: Point<T>,
    vertecies: usize,
    distance: T,
) -> Vec<Coord<T>> {
    let step = num_traits::cast::<f32, T>(360.0).unwrap() / num_traits::cast(vertecies).unwrap();
    let mut points = Vec::<Coord<T>>::with_capacity(vertecies);
    for i in 0..vertecies {
        let angle = step * num_traits::cast(i).unwrap();
        points.push(Haversine.destination(point, angle, distance).into());
    }
    points
}
