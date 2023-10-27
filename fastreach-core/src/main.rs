use std::fs::File;

use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use fastreach_core::{
    cascade,
    graph::{self, Graph, IsochroneDijsktra},
    vincenty, MOVE_SPEED,
};
use geo::{Coord, CoordsIter, GeodesicArea, LineString, Polygon};
use memmap2::Mmap;

const ERFURT_HBF: u64 = 13_973_471_588_854_917_578;

fn main() {
    let file = File::open("graph.bin").expect("failed to open graph data");
    let mapping = unsafe { Mmap::map(&file).expect("failed mmap") };
    let graph = Graph::from_slice(&mapping).expect("failed to parse data");
    let start = std::time::Instant::now();
    let station_idx = graph.ids.get(&ERFURT_HBF).unwrap();
    let algo = IsochroneDijsktra::new(&graph);
    let reached = algo
        .nodes_within(
            *station_idx,
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2023, 10, 18).unwrap(),
                NaiveTime::from_hms_opt(10, 15, 30).unwrap(),
            ),
            Duration::minutes(180),
        )
        .expect("failed dijsktra");

    let deduped_nodes = graph::dedup_by_coverage(graph::dedup_by_coords(&reached));

    let polys: Vec<geo::Polygon<f64>> = deduped_nodes
        .into_iter()
        .map(|r| {
            let distance = MOVE_SPEED * r.duration.num_minutes() as f64;
            vincenty::spherical_circle(
                Coord::from((r.node.lon() as f64, r.node.lat() as f64)),
                8,
                distance,
            )
        })
        .map(|mut verts| {
            verts.push(verts[0]);
            let line_string = LineString::new(verts);
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
