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

#[derive(serde_derive::Deserialize)]
struct IsochroneBody {
    name: String,
    start: i64,
    minutes: i64,
}

#[derive(serde_derive::Serialize)]
struct IsochroneReply {
    area: f64,
    diameter: f64,
    geometry: geojson::Value,
}

#[tokio::main]
async fn main() {
    let graph = Arc::new(Graph::from_slice(&GRAPH_DATA).expect("failed to parse graph"));
    let handler = warp::post()
        .and(warp::path!("api" / "v1" / "isochrone"))
        .and(warp::body::json::<IsochroneBody>())
        .map(move |body: IsochroneBody| {
            let Some(start_idx) = graph.names.get(&body.name) else {
                return warp::reply::with_status(
                    warp::reply::json(&"station not found".to_owned()),
                    warp::http::StatusCode::BAD_REQUEST,
                );
            };
            let start_station = &graph.nodes[*start_idx];
            let algo = IsochroneDijsktra::new(&graph);
            let Some(start_time) = NaiveDateTime::from_timestamp_millis(body.start) else {
                return warp::reply::with_status(
                    warp::reply::json(&"invalid start time".to_owned()),
                    warp::http::StatusCode::BAD_REQUEST,
                );
            };
            let Ok(reached) =
                algo.nodes_within(start_station, start_time, Duration::minutes(body.minutes))
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
                    spherical_circle((r.node.lat() as f64, r.node.lon() as f64), 8, distance)
                })
                .map(|verts| {
                    let mut coords: Vec<Coord<f64>> =
                        verts.iter().map(|t| geo::Coord::from(*t)).collect();
                    coords.push(coords[0]);
                    let line_string = LineString::new(coords);
                    Polygon::new(line_string, vec![])
                })
                .collect();
            let merged = cascade::union(polys);
            let iso_reply = IsochroneReply {
                area: merged.geodesic_area_signed() / 1_000_000.0,
                diameter: cascade::diameter(&merged) / 1000.0,
                geometry: geojson::Value::from(&merged),
            };
            warp::reply::with_status(warp::reply::json(&iso_reply), warp::http::StatusCode::OK)
        });

    warp::serve(handler).run(([0, 0, 0, 0], 8080)).await;
}
