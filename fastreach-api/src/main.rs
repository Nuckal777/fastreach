use std::{fs::File, sync::Arc};

use chrono::{Duration, NaiveDateTime};
use fastreach_core::{
    cascade,
    graph::{dedup_by_coords, dedup_by_coverage, Graph, IsochroneDijsktra},
};
use geo::{Polygon, ChamberlainDuquetteArea};
use lazy_static::lazy_static;
use memmap2::Mmap;
use warp::Filter;

const MAX_MINUTES_DEFAULT: i64 = 120;
const GRAPH_DEFAULT: &str = "graph.bin";

lazy_static! {
    static ref GRAPH_DATA: Mmap = {
        let path = std::env::var("FASTREACH_GRAPH").unwrap_or_else(|_| GRAPH_DEFAULT.to_owned());
        let file = File::open(path).expect("failed to open graph data");
        unsafe { Mmap::map(&file).expect("failed mmap") }
    };
}

#[derive(serde_derive::Deserialize)]
struct IsochroneBody {
    // JS cannot deal with large integers in JSON
    id: String,
    start: i64,
    minutes: i64,
}

#[derive(serde_derive::Serialize)]
struct IsochroneReply {
    area: f32,
    diameter: f32,
    geometry: geojson::GeoJson,
}

#[tokio::main]
async fn main() {
    let max_minutes = match std::env::var("FASTREACH_MAX_MINUTES") {
        Ok(val) => str::parse(&val).unwrap_or(MAX_MINUTES_DEFAULT),
        Err(_) => MAX_MINUTES_DEFAULT,
    };
    let graph = Arc::new(Graph::from_slice(&GRAPH_DATA).expect("failed to parse graph"));
    let node_count = graph.nodes.len();
    let handler = warp::post()
        .and(warp::path!("api" / "v1" / "isochrone"))
        .and(warp::body::json::<IsochroneBody>())
        .map(move |body: IsochroneBody| {
            if body.minutes < 0 || body.minutes > max_minutes {
                return warp::reply::with_status(
                    warp::reply::json(&"minutes out of range".to_owned()),
                    warp::http::StatusCode::BAD_REQUEST,
                );
            }
            let Ok(id) = str::parse::<u64>(&body.id) else {
                return warp::reply::with_status(
                    warp::reply::json(&"cannot parse id".to_owned()),
                    warp::http::StatusCode::BAD_REQUEST,
                );
            };
            let Some(start_idx) = graph.ids.get(&id) else {
                return warp::reply::with_status(
                    warp::reply::json(&"station not found".to_owned()),
                    warp::http::StatusCode::BAD_REQUEST,
                );
            };
            let Some(start_time) = NaiveDateTime::from_timestamp_millis(body.start) else {
                return warp::reply::with_status(
                    warp::reply::json(&"invalid start time".to_owned()),
                    warp::http::StatusCode::BAD_REQUEST,
                );
            };
            let mut algo = IsochroneDijsktra::new(&graph);
            let Ok(reached) =
                algo.nodes_within(*start_idx, start_time, Duration::minutes(body.minutes))
            else {
                return warp::reply::with_status(
                    warp::reply::json(&"failed dijsktra".to_owned()),
                    warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                );
            };

            // depending on the specific hardware it may be faster to not dedup
            let deduped_tree = dedup_by_coverage(dedup_by_coords(&reached));
            let polys: Vec<Polygon<f32>> = deduped_tree.into_iter().map(|n| n.to_poly()).collect();
            let merged = cascade::union_polys(polys);

            let iso_reply = IsochroneReply {
                area: merged.chamberlain_duquette_signed_area() / 1_000_000.0,
                diameter: cascade::diameter(&merged) / 1000.0,
                geometry: geojson::GeoJson::from(&merged),
            };
            warp::reply::with_status(warp::reply::json(&iso_reply), warp::http::StatusCode::OK)
        });

    let (_addr, serve) =
        warp::serve(handler).bind_with_graceful_shutdown(([0, 0, 0, 0], 8080), async move {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to listen to shutdown signal");
        });

    println!("Serving {node_count} nodes on 0.0.0.0:8080");
    serve.await;

    println!("Bye");
}
