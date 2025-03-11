use anyhow::anyhow;
use axum::Router;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing::{info, warn};

use get_if_addrs::get_if_addrs;

#[derive(Debug)]
struct HttpServeState {
    path: std::path::PathBuf,
}

fn get_ipv4_addr() -> anyhow::Result<Ipv4Addr> {
    let if_addrs = get_if_addrs()?;
    for if_addr in if_addrs {
        if let get_if_addrs::IfAddr::V4(addr) = if_addr.addr {
            if addr.ip.is_loopback() {
                continue;
            }

            return Ok(addr.ip);
        }
    }

    Err(anyhow!("No IPv4 address found"))
}

pub async fn process_http_serve(path: std::path::PathBuf, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let state = HttpServeState { path: path.clone() };
    let router = Router::new()
        .nest_service("/tower", ServeDir::new(path.clone()))
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    let addr = listener.local_addr()?;
    info!("Starting server on {:?}:{}", addr.ip(), addr.port());
    if let Ok(ip) = get_ipv4_addr() {
        info!("Starting server on {:?}:{}", ip, addr.port());
    }
    axum::serve(listener, router).await?;

    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", p);
    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            format!("File {} note found", p.display()),
        )
    } else {
        // TODO: test p is a directory
        // if it is a directory, list all files/subdirectories
        // as <li><a href="/path/to/file">file name</a></li>
        // <html><body><ul>...</ul></body></html>
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, content)
            }
            Err(e) => {
                warn!("Error reading file: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}
