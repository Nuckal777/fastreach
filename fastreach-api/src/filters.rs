use warp::{reply, Filter};

fn precompressed_br(
    base_path: &str,
    file: &str,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let with_br = file.to_owned() + ".br";
    let prefix = warp::get()
        .and(warp::path(file.to_owned()))
        .and(warp::path::end());
    let brotli = prefix
        .clone()
        .and(warp::filters::header::header::<String>("accept-encoding"))
        .and_then(|encoding: String| async move {
            for chunk in encoding.split(',') {
                if chunk.trim() == "br" {
                    return Ok(());
                }
            }
            Err(warp::reject())
        })
        .untuple_one()
        .and(
            warp::filters::fs::file(std::path::PathBuf::from(base_path).join(with_br))
                .with(reply::with::header("content-encoding", "br")),
        );
    let uncompressed = prefix.and(warp::filters::fs::file(
        std::path::PathBuf::from(base_path).join(file),
    ));
    brotli.or(uncompressed)
}

pub(crate) fn static_content(
    base_path: String,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    precompressed_br(&base_path, "nodes-v1.json").or(warp::filters::fs::dir(base_path))
}
