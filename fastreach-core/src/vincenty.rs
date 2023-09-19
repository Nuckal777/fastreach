use std::f64::consts;

const EARTH_RADIUS: f64 = 6371.009;

/// takes a (lon,lat) and returns a (lon, lat)
#[must_use]
pub fn spherical_walk(point: (f64, f64), angle: f64, distance: f64) -> (f64, f64) {
    let lon = point.0 / 180.0 * consts::PI;
    let lat = point.1 / 180.0 * consts::PI;
    let rad = angle / 180.0 * consts::PI;

    let s_alpha = lat.cos() * rad.sin();
    let sigma = distance / EARTH_RADIUS;

    let t1 = lat.sin() * sigma.cos() + lat.cos() * sigma.sin() * rad.cos();
    let intermediate = lat.sin() * sigma.sin() - lat.cos() * sigma.cos() * rad.cos();
    let t2 = (s_alpha * s_alpha + intermediate * intermediate).sqrt();
    let phi2 = t1.atan2(t2);

    let lambda_upper = sigma.sin() * rad.sin();
    let lambda_lower = lat.cos() * sigma.cos() - lat.sin() * sigma.sin() * rad.cos();
    let lambda = lambda_upper.atan2(lambda_lower);
    let l2 = lambda + lon;

    let result_lon = l2 * 180.0 / consts::PI;
    let result_lat = phi2 * 180.0 / consts::PI;
    (result_lon, result_lat)
}

/// takes a (lon, lat) and returns a Vec<(lon, lat)>
#[must_use]
pub fn spherical_circle(point: (f64, f64), vertecies: usize, distance: f64) -> Vec<(f64, f64)> {
    let step = 360.0 / vertecies as f64;
    let mut points = Vec::<(f64, f64)>::with_capacity(vertecies);
    for i in 0..vertecies {
        let angle = step * i as f64;
        points.push(spherical_walk(point, angle, distance));
    }
    points
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    #[test]
    fn spherical_walk() {
        let start = (51.050_94_f64, 13.734_617_f64);
        let (lat, lon) = super::spherical_walk(start, 0.0, 0.5);
        assert_relative_eq!(lon, 13.734_617);
        assert_relative_eq!(lat, 51.055_437);
    }
}
