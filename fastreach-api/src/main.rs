use std::{fs::File, sync::Arc};

use chrono::{Duration, NaiveDateTime};
use fastreach_core::{
    cascade,
    graph::{dedup_by_coords, Graph, IsochroneDijsktra},
    vincenty::spherical_circle,
    MOVE_SPEED,
};
use geo::{Coord, GeodesicArea, LineString, Polygon};
use lazy_static::lazy_static;
use memmap2::Mmap;
use warp::Filter;

lazy_static! {
    static ref GRAPH_DATA: Mmap = {
        let file = File::open("graph.bin").expect("failed to open graph data");
        unsafe { Mmap::map(&file).expect("failed mmap") }
    };
}

const MAX_MINUTES_DEFAULT: i64 = 120;

#[derive(serde_derive::Deserialize)]
struct IsochroneBody {
    // JS cannot deal with large integers in JSON
    id: String,
    start: i64,
    minutes: i64,
}

#[derive(serde_derive::Serialize)]
struct IsochroneReply {
    area: f64,
    diameter: f64,
    geometry: geojson::GeoJson,
}

#[tokio::main]
async fn main() {
    let max_minutes = match std::env::var("FASTREACH_MAX_MINUTES") {
        Ok(val) => str::parse(&val).unwrap_or(MAX_MINUTES_DEFAULT),
        Err(_) => MAX_MINUTES_DEFAULT,
    };
    let graph = Arc::new(Graph::from_slice(&GRAPH_DATA).expect("failed to parse graph"));
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
            let algo = IsochroneDijsktra::new(&graph);
            let Some(start_time) = NaiveDateTime::from_timestamp_millis(body.start) else {
                return warp::reply::with_status(
                    warp::reply::json(&"invalid start time".to_owned()),
                    warp::http::StatusCode::BAD_REQUEST,
                );
            };
            let Ok(reached) =
                algo.nodes_within(*start_idx, start_time, Duration::minutes(body.minutes))
            else {
                return warp::reply::with_status(
                    warp::reply::json(&"failed dijsktra".to_owned()),
                    warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                );
            };

            let deduped_nodes = dedup_by_coords(&reached);

            let polys: Vec<geo::Polygon<f64>> = deduped_nodes
                .into_iter()
                .map(|r| {
                    let distance = MOVE_SPEED * r.duration.num_minutes() as f64;
                    spherical_circle(
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
            let iso_reply = IsochroneReply {
                area: merged.geodesic_area_signed() / 1_000_000.0,
                diameter: cascade::diameter(&merged) / 1000.0,
                geometry: geojson::GeoJson::from(&merged),
            };
            warp::reply::with_status(warp::reply::json(&iso_reply), warp::http::StatusCode::OK)
        });

    warp::serve(handler).run(([0, 0, 0, 0], 8080)).await;
}
