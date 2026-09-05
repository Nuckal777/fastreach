use std::{
    fs::File,
    path::PathBuf,
    sync::{Arc, LazyLock},
};

use chrono::{DateTime, Duration};
use fastreach_core::{
    cascade,
    graph::{u16_to_date, Graph, IsochroneDijsktra},
};
use geo::{ChamberlainDuquetteArea, Polygon};
use memmap2::Mmap;
use thiserror::Error;
use warp::{http::StatusCode, reply, Filter};

mod cache;
mod filters;

const STATIC_DEFAULT: &str = "static";
const MAX_MINUTES_DEFAULT: i64 = 120;
const PARALLEL_DEFAULT: usize = 2;

static GRAPH_DATAS: LazyLock<Vec<Mmap>> = LazyLock::new(|| {
    let mut result = Vec::<Mmap>::new();
    const DATA_DIR: &str = "data";
    let dir_iter = std::fs::read_dir(DATA_DIR).expect("failed to list data directory");
    let dir_paths: Result<Vec<PathBuf>, std::io::Error> =
        dir_iter.map(|d| d.map(|e| e.path())).collect();
    let mut dir_paths = dir_paths.expect("failed to list entry in data directory");
    dir_paths.sort();
    for entry in dir_paths {
        let file = File::open(entry).expect("failed to open graph");
        let mapping = unsafe { Mmap::map(&file).expect("failed memory mapping") };
        result.push(mapping);
    }
    result
});

#[derive(serde_derive::Deserialize)]
struct IsochroneBody {
    // JS cannot deal with large integers in JSON
    id: String,
    dataset: usize,
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

#[derive(Clone, Debug, serde_derive::Serialize)]
struct DatasetInfo {
    name: String,
    nodes: usize,
    edges: usize,
    from: chrono::NaiveDate,
    to: chrono::NaiveDate,
}

#[derive(serde_derive::Serialize)]
struct DatasetsReply {
    datasets: Vec<DatasetInfo>,
}

struct DatasetsHandler {
    infos: Vec<DatasetInfo>,
}

impl DatasetsHandler {
    fn handle_datasets(&self) -> Result<DatasetsReply, HandlerError> {
        Ok(DatasetsReply {
            datasets: self.infos.clone(),
        })
    }
}

struct IsochroneHandler<'a> {
    graphs: Vec<Graph<'a>>,
    max_minutes: i64,
}

impl IsochroneHandler<'_> {
    fn handle_isochrone(&self, body: &IsochroneBody) -> Result<IsochroneReply, HandlerError> {
        if body.minutes < 0 || body.minutes > self.max_minutes {
            return Err(HandlerError::BadRequest("minutes out of range".to_owned()));
        }
        let id = str::parse::<u64>(&body.id)
            .map_err(|_| HandlerError::BadRequest("cannot parse id".to_owned()))?;
        let graph = match self.graphs.get(body.dataset) {
            Some(g) => g,
            None => return Err(HandlerError::BadRequest("dataset out of range".to_owned())),
        };
        let start_idx = graph
            .ids
            .get(&id)
            .ok_or(HandlerError::BadRequest("station not found".to_owned()))?;
        let start_time = DateTime::from_timestamp_millis(body.start)
            .ok_or(HandlerError::BadRequest("invalid start time".to_owned()))?;
        let mut algo = IsochroneDijsktra::new(graph);
        let reached = algo
            .nodes_within(
                *start_idx,
                start_time.naive_utc(),
                Duration::minutes(body.minutes),
            )
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

    let mut graphs = Vec::<Graph>::new();
    let mut infos = Vec::<DatasetInfo>::new();
    for mapping in GRAPH_DATAS.iter() {
        let graph = Graph::from_slice(mapping).expect("failed to parse grpah");
        let node_count = graph.nodes.len();
        let edge_count: usize = graph.nodes.iter().map(|n| n.outgoing.len()).sum();
        let dataset_name = graph.metadata.name().to_owned();
        let dataset_from = u16_to_date(graph.metadata.from());
        let dataset_to = u16_to_date(graph.metadata.to());
        println!(
            "Loaded {node_count} nodes and {edge_count} edges from {dataset_name} ({dataset_from} - {dataset_to})"
        );
        graphs.push(graph);
        infos.push(DatasetInfo {
            name: dataset_name,
            nodes: node_count,
            edges: edge_count,
            from: dataset_from,
            to: dataset_to,
        });
    }

    cache::build_nodes_cache(&graphs).expect("failed to build node api cache");

    let semaphore = Arc::new(tokio::sync::Semaphore::new(parallel));
    let iso_handler = Arc::new(IsochroneHandler {
        graphs,
        max_minutes,
    });
    let isochrone_api = warp::post()
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

    let datasets_handler = Arc::new(DatasetsHandler { infos });
    let datasets_api = warp::get()
        .and(warp::path!("api" / "v1" / "datasets"))
        .then(move || {
            let local_handler = datasets_handler.clone();
            async move {
                match local_handler.handle_datasets() {
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

    let nodes_api = warp::get()
        .and(warp::path!("api" / "v1" / "nodes" / usize))
        .then(move |idx| async move {
            match tokio::fs::File::open(cache::get_path_for_nodes(idx)).await {
                Ok(f) => {
                    let stream = tokio_util::io::ReaderStream::new(f);
                    Box::new(warp::reply::stream(stream)) as Box<dyn warp::Reply>
                }
                Err(_) => Box::new(warp::reply::with_status(reply(), StatusCode::NOT_FOUND))
                    as Box<dyn warp::Reply>,
            }
        });

    let api = isochrone_api.or(nodes_api).or(datasets_api);
    let serve = warp::serve(api.or(filters::static_content(static_path)))
        .bind(([0, 0, 0, 0], 8080))
        .await
        .graceful(async move {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to listen to shutdown signal");
        })
        .run();

    println!("Serving on 0.0.0.0:8080");
    serve.await;

    println!("Bye");
}
