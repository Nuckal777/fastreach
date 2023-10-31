use std::fs::File;

use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use fastreach_core::{
    cascade,
    graph::{self, Graph, IsochroneDijsktra},
};
use geo::{GeodesicArea, Polygon};
use memmap2::Mmap;

const ERFURT_HBF: u64 = 13_973_471_588_854_917_578;

fn main() {
    let file = File::open("graph.bin").expect("failed to open graph data");
    let mapping = unsafe { Mmap::map(&file).expect("failed mmap") };
    let graph = Graph::from_slice(&mapping).expect("failed to parse data");
    let start = std::time::Instant::now();
    let station_idx = graph.ids.get(&ERFURT_HBF).unwrap();
    let mut algo = IsochroneDijsktra::new(&graph);
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

    let deduped_tree = graph::dedup_by_coverage(graph::dedup_by_coords(&reached));
    let polys: Vec<Polygon<f64>> = deduped_tree.into_iter().map(|n| n.to_poly()).collect();
    let merged = cascade::union_polys(polys);
    let area = merged.geodesic_area_signed() / 1_000_000.0;
    let diameter = cascade::diameter(&merged) / 1_000.0;
    let end = std::time::Instant::now();
    println!("duration: {} ms", (end - start).as_millis());
    println!("area: {area} km2");
    println!("diameter: {diameter} km");
}
