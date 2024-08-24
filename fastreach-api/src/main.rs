use std::{fs::File, sync::Arc};

use chrono::{Duration, NaiveDateTime};
use fastreach_core::{
    cascade,
    graph::{Graph, IsochroneDijsktra},
};
use geo::{ChamberlainDuquetteArea, Polygon};
use lazy_static::lazy_static;
use memmap2::Mmap;
use thiserror::Error;
use warp::{http::StatusCode, reply, Filter};

mod filters;

const GRAPH_DEFAULT: &str = "graph.bin";
const STATIC_DEFAULT: &str = "static";
const MAX_MINUTES_DEFAULT: i64 = 120;
const PARALLEL_DEFAULT: usize = 2;

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

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    InternalServerError(String),
}

struct IsochroneHandler<'a> {
    graph: Graph<'a>,
    max_minutes: i64,
}

impl<'a> IsochroneHandler<'a> {
    fn handle_isochrone(&self, body: &IsochroneBody) -> Result<IsochroneReply, HandlerError> {
        if body.minutes < 0 || body.minutes > self.max_minutes {
            return Err(HandlerError::BadRequest("minutes out of range".to_owned()));
        }
        let id = str::parse::<u64>(&body.id)
            .map_err(|_| HandlerError::BadRequest("cannot parse id".to_owned()))?;
        let start_idx = self
            .graph
            .ids
            .get(&id)
            .ok_or(HandlerError::BadRequest("station not found".to_owned()))?;
        let start_time = NaiveDateTime::from_timestamp_millis(body.start)
            .ok_or(HandlerError::BadRequest("invalid start time".to_owned()))?;
        let mut algo = IsochroneDijsktra::new(&self.graph);
        let reached = algo
            .nodes_within(*start_idx, start_time, Duration::minutes(body.minutes))
            .map_err(|_| HandlerError::InternalServerError("failed dijsktra".to_owned()))?;

        let polys: Vec<Polygon<f32>> = reached.into_iter().map(|n| n.to_poly()).collect();
        let merged = cascade::union_polys(polys);

        Ok(IsochroneReply {
            area: merged.chamberlain_duquette_unsigned_area() / 1_000_000.0,
            diameter: cascade::diameter(&merged) / 1000.0,
            geometry: geojson::GeoJson::from(&merged),
        })
    }
}

#[tokio::main]
async fn main() {
    let max_minutes = match std::env::var("FASTREACH_MAX_MINUTES") {
        Ok(val) => str::parse(&val).unwrap_or(MAX_MINUTES_DEFAULT),
        Err(_) => MAX_MINUTES_DEFAULT,
    };
    let parallel = match std::env::var("FASTREACH_PARALLEL") {
        Ok(val) => str::parse(&val).unwrap_or(PARALLEL_DEFAULT),
        Err(_) => PARALLEL_DEFAULT,
    };
    let static_path =
        std::env::var("FASTREACH_STATIC").unwrap_or_else(|_| STATIC_DEFAULT.to_owned());

    let graph = Graph::from_slice(&GRAPH_DATA).expect("failed to parse graph");
    let node_count = graph.nodes.len();
    let semaphore = Arc::new(tokio::sync::Semaphore::new(parallel));
    let iso_handler = Arc::new(IsochroneHandler { graph, max_minutes });
    let api = warp::post()
        .and(warp::path!("api" / "v1" / "isochrone"))
        .and(warp::body::json::<IsochroneBody>())
        .then(move |body: IsochroneBody| {
            let local_handler = iso_handler.clone();
            let local_semaphore = semaphore.clone();
            async move {
                let _permit = local_semaphore
                    .acquire()
                    .await
                    .expect("semaphore closed unexpectedly");
                match local_handler.handle_isochrone(&body) {
                    Ok(reply) => reply::with_status(reply::json(&reply), StatusCode::OK),
                    Err(HandlerError::BadRequest(msg)) => {
                        reply::with_status(reply::json(&msg), StatusCode::BAD_REQUEST)
                    }
                    Err(HandlerError::InternalServerError(msg)) => {
                        reply::with_status(reply::json(&msg), StatusCode::INTERNAL_SERVER_ERROR)
                    }
                }
            }
        });

    let (_addr, serve) = warp::serve(api.or(filters::static_content(static_path)))
        .bind_with_graceful_shutdown(([0, 0, 0, 0], 8080), async move {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to listen to shutdown signal");
        });

    println!("Serving {node_count} nodes on 0.0.0.0:8080");
    serve.await;

    println!("Bye");
}
