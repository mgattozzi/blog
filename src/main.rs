use axum::extract::Path;
use axum::http::StatusCode;
use axum::http::Uri;
use axum::response::IntoResponse;
use axum::response::Redirect;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use tracing::event;
use tracing::Level;

// Macros
macro_rules! static_file {
    ($input:expr) => {
        include_bytes!(concat!(env!("OUT_DIR"), "/", $input))
    };
}

// Website Pages
const HOME: &[u8] = static_file!("home");
const ABOUT: &[u8] = static_file!("about");
const CONTACT: &[u8] = static_file!("contact");
const ORPHAN: &[u8] = static_file!("orphan-rules");
const OXIDIZING: &[u8] = static_file!("oxidize-interview");
const PAGE_404: &[u8] = static_file!("page-404");
const RUNTIME: &[u8] = static_file!("rusts-runtime");
const RUST_WASM: &[u8] = static_file!("rust-wasm");
const THE_EDGE: &[u8] = static_file!("the-edge");
const WEIRD: &[u8] = static_file!("weird-exprs");

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let router = Router::new()
        .route("/", get(home))
        .route("/about", get(about))
        .route("/contact", get(contact))
        // Redirect Old Slugs for SEO
        .route("/orphan-rules", get(redirect_old))
        .route("/oxidizing-the-technical-interview", get(redirect_old))
        .route("/rust-wasm", get(redirect_old))
        .route("/rusts-runtime", get(redirect_old))
        .route(
            "/weird-expressions-and-where-to-find-them",
            get(redirect_old),
        )
        // Serve all Blog Posts
        .route("/posts/*post", get(serve_post))
        .fallback(fallback);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    event!(Level::INFO, "listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn redirect_old(uri: Uri) -> Redirect {
    Redirect::permanent(&("/posts".to_string() + uri.path()))
}

async fn home() -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            ("Content-Encoding", "gzip"),
            ("Content-Type", "text/html; charset=utf-8"),
            ("Cache-Control", "max-age=86400"),
        ],
        HOME,
    )
}

async fn contact() -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            ("Content-Encoding", "gzip"),
            ("Content-Type", "text/html; charset=utf-8"),
            ("Cache-Control", "max-age=86400"),
        ],
        CONTACT,
    )
}

async fn about() -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            ("Content-Encoding", "gzip"),
            ("Content-Type", "text/html; charset=utf-8"),
            ("Cache-Control", "max-age=86400"),
        ],
        ABOUT,
    )
}

async fn serve_post(Path(post): Path<String>) -> impl IntoResponse {
    let body = match post.as_str() {
        "orphan-rules" => ORPHAN,
        "oxidizing-the-technical-interview" => OXIDIZING,
        "rust-wasm" => RUST_WASM,
        "rusts-runtime" => RUNTIME,
        "weird-expressions-and-where-to-find-them" => WEIRD,
        "the-edge" => THE_EDGE,
        _ => {
            return (
                StatusCode::NOT_FOUND,
                [
                    ("Content-Encoding", "gzip"),
                    ("Content-Type", "text/html; charset=utf-8"),
                    ("Cache-Control", "max-age=86400"),
                ],
                PAGE_404,
            )
        }
    };

    (
        StatusCode::OK,
        [
            ("Content-Encoding", "gzip"),
            ("Content-Type", "text/html; charset=utf-8"),
            ("Cache-Control", "max-age=86400"),
        ],
        body,
    )
}

async fn fallback() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        [
            ("Content-Encoding", "gzip"),
            ("Content-Type", "text/html; charset=utf-8"),
            ("Cache-Control", "max-age=86400"),
        ],
        PAGE_404,
    )
}
