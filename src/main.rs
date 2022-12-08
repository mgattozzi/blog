use fastly::http::StatusCode;
use fastly::Error;
use fastly::Request;
use fastly::Response;
use phf::phf_map;
use phf::Map;

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

// Images
const THE_EDGE_SVG_1: &[u8] = file!("the-edge-1.svg");

static LOOKUP: Map<&'static str, &'static [u8]> = phf_map! {
        "/" => HOME,
        "/about" => ABOUT,
        "/contact" => CONTACT,
        "/the-edge" => THE_EDGE,
        "/the-edge-1.svg" => THE_EDGE_SVG_1,
        "/weird-expressions-and-where-to-find-them" => WEIRD,
        "/orphan-rules" => ORPHAN,
        "/rusts-runtime" => RUNTIME,
        "/rust-wasm" => RUST_WASM,
        "/oxidizing-the-technical-interview" => OXIDIZING,
};

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    let path = req.get_path();
    Ok(LOOKUP
        .get(path)
        .map(|body| {
            if path.ends_with("svg") {
                make_svg_res(body)
            } else {
                make_200_res(body)
            }
        })
        .unwrap_or_else(make_404_res))
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

fn make_svg_res(image: &[u8]) -> Response {
    Response::from_status(StatusCode::OK)
        .with_header("Content-Type", "image/svg+xml")
        // One Day - 60s/min * 60 min/hr * 24 hr/day
        .with_header("Cache-Control", "max-age=86400")
        .with_body(image)
}
