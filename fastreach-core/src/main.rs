use std::fs::File;

use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use fastreach_core::{
    cascade,
    graph::{Graph, IsochroneDijsktra},
};
use geo::{ChamberlainDuquetteArea, Polygon};
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
    let within_end = std::time::Instant::now();
    println!("within duration: {} ms", (within_end - start).as_millis());

    let polys: Vec<Polygon<f32>> = reached.into_iter().map(|n| n.to_poly()).collect();
    let merged = cascade::union_polys(polys);
    let area = merged.chamberlain_duquette_unsigned_area() / 1_000_000.0;
    let diameter = cascade::diameter(&merged) / 1_000.0;
    let end = std::time::Instant::now();
    println!("duration: {} ms", (end - start).as_millis());
    println!("area: {area} km2");
    println!("diameter: {diameter} km");
}
