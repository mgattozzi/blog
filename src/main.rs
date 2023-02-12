use axum::http::StatusCode;
use axum::response::Responsed;
use axum::routing::get;
use axum::Router;
use phf::phf_map;
use phf::Map;
use std::net::SocketAddr;

// Macros
macro_rules! file {
    ($input:expr) => {
        include_bytes!(concat!(env!("OUT_DIR"), "/", $input))
    };
}

// Website Pages
const HOME: &[u8] = file!("home");
const ABOUT: &[u8] = file!("about");
const CONTACT: &[u8] = file!("contact");
const THE_EDGE: &[u8] = file!("the-edge");
const WEIRD: &[u8] = file!("weird-exprs");
const ORPHAN: &[u8] = file!("orphan-rules");
const RUNTIME: &[u8] = file!("rusts-runtime");
const OXIDIZING: &[u8] = file!("oxidize-interview");
const RUST_WASM: &[u8] = file!("rust-wasm");
const PAGE_404: &[u8] = file!("page-404");

static LOOKUP: Map<&'static str, &'static [u8]> = phf_map! {
        "/" => HOME,
        "/about" => ABOUT,
        "/contact" => CONTACT,
        "/the-edge" => THE_EDGE,
        "/weird-expressions-and-where-to-find-them" => WEIRD,
        "/orphan-rules" => ORPHAN,
        "/rusts-runtime" => RUNTIME,
        "/rust-wasm" => RUST_WASM,
        "/oxidizing-the-technical-interview" => OXIDIZING,
};

#[tokio::main]
async fn main() {
  let router = Router::new()
    .route("/", get(home))
    .route("/about", get())
    .route("/contact", get())
    .route("/the-edge", get())
    .route("/weird-expressions-and-where-to-find-them", get())
    .route("/orphan-rules", get())
    .route("/rusts-runtime", get())
    .route("/rust-wasm", get())
    .route("/oxidizing-the-technical-interview", get())

}

fn make_200_res(body: &[u8]) -> Response {
    Response::from_status(StatusCode::OK)
        .with_header("Content-Encoding", "gzip")
        // One Day - 60s/min * 60 min/hr * 24 hr/day
        .with_header("Cache-Control", "max-age=86400")
        .with_body(body)
}

fn make_404_res() -> Response {
    Response::from_status(StatusCode::NOT_FOUND)
        .with_header("Content-Encoding", "gzip")
        // One Day - 60s/min * 60 min/hr * 24 hr/day
        .with_header("Cache-Control", "max-age=86400")
        .with_body(PAGE_404)
}
