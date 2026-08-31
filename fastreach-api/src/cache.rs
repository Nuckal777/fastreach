use std::io::{BufReader, BufWriter};

use brotli::enc::BrotliEncoderParams;
use fastreach_core::graph::Graph;

const CACHE_DIR: &str = "cache";

#[derive(serde_derive::Serialize)]
struct NodeInfo {
    id: String, // JS number type cannot handle large integers
    coords: [f32; 2],
    name: String,
}

pub fn build_nodes_cache<'a>(graphs: &[Graph<'a>]) -> Result<(), Box<dyn std::error::Error>> {
    for (i, graph) in graphs.iter().enumerate() {
        let filename = get_path_for_nodes(i);
        let file = std::fs::File::create_new(&filename);
        match file {
            Ok(file) => {
                println!("Generating nodes cache at {filename}.");
                let nodes: Vec<NodeInfo> = graph
                    .nodes
                    .iter()
                    .map(|n| NodeInfo {
                        id: n.id().to_string(),
                        coords: [n.lon(), n.lat()],
                        name: n.name().to_string(),
                    })
                    .collect();
                serde_json::to_writer(BufWriter::new(file), &nodes)?;

                let filename_compressed = filename.clone() + ".br";
                println!(
                    "Generating compressed nodes cache at {filename_compressed}. This may take a bit."
                );
                let mut uncompressed = BufReader::new(std::fs::File::open(&filename)?);
                let mut compressed =
                    BufWriter::new(std::fs::File::create_new(filename_compressed)?);
                brotli::BrotliCompress(
                    &mut uncompressed,
                    &mut compressed,
                    &BrotliEncoderParams::default(),
                )?;
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                println!(
                    "Nodes cache at {filename} exists. Consider deleting, if graph data changed."
                );
                continue;
            }
            Err(e) => return Err(Box::new(e)),
        }
    }
    Ok(())
}

pub fn get_path_for_nodes(idx: usize) -> String {
    format!("{CACHE_DIR}/{idx}.json")
}
