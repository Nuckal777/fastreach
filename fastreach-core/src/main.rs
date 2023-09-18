use std::fs::File;

use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use fastreach_core::{
    cascade,
    graph::{Graph, IsochroneDijsktra, self},
    vincenty, MOVE_SPEED,
};
use geo::{Coord, GeodesicArea, LineString, Polygon};
use memmap2::Mmap;

fn main() {
    let file = File::open("graph.bin").expect("failed to open graph data");
    let mapping = unsafe { Mmap::map(&file).expect("failed mmap") };
    let graph = Graph::from_slice(&mapping).expect("failed to parse data");
    let start = std::time::Instant::now();
    let station_idx = graph.names.get("Erfurt Hbf").unwrap();
    let station = &graph.nodes[*station_idx];
    let algo = IsochroneDijsktra::new(&graph);
    let reached = algo
        .nodes_within(
            station,
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2022, 7, 23).unwrap(),
                NaiveTime::from_hms_opt(10, 15, 30).unwrap(),
            ),
            Duration::minutes(90),
        )
        .expect("failed dijsktra");

    let deduped_nodes = graph::dedup_by_coords(&reached);

    let polys: Vec<geo::Polygon<f64>> = deduped_nodes
        .into_iter()
        .map(|r| {
            let distance = MOVE_SPEED * r.duration.num_minutes() as f64;
            vincenty::spherical_circle(
                (r.node.lat() as f64, r.node.lon() as f64),
                8,
                distance,
            )
        })
        .map(|verts| {
            let mut coords: Vec<Coord<f64>> = verts.iter().map(|t| geo::Coord::from(*t)).collect();
            coords.push(coords[0]);
            let line_string = LineString::new(coords);
            Polygon::new(line_string, vec![])
        })
        .collect();
    let merged = cascade::union(polys);
    let area = merged.geodesic_area_signed() / 1_000_000.0;
    let diameter = cascade::diameter(&merged) / 1_000.0;
    let end = std::time::Instant::now();
    println!("duration: {} ms", (end - start).as_millis());
    println!("area: {area} km2");
    println!("diameter: {diameter} km");
}
