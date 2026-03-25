//! Prometheus Adapter

use std::net::SocketAddr;
use tokio::net::TcpListener;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::header::CONTENT_TYPE;

use crate::application::PrometheusExporter;
use crate::domain::Registry;

/// Start Prometheus metrics endpoint
pub async fn start_prometheus_server(
    addr: SocketAddr,
    registry: Registry,
) -> Result<(), Box<dyn std::error::Error>> {
    let exporter = PrometheusExporter::new();
    let registry = std::sync::Arc::new(registry);

    let make_svc = make_service_fn(move |_conn| {
        let registry = registry.clone();
        let exporter = PrometheusExporter::new();
        async move {
            Ok::<_, hyper::Error>(service_fn(move |_req: Request<Body>| {
                let registry = registry.clone();
                let exporter = exporter.clone();
                async move {
                    let metrics = exporter.export(&registry).unwrap_or_default();
                    Ok::<_, hyper::Error>(Response::builder()
                        .header(CONTENT_TYPE, "text/plain; version=0.0.4")
                        .body(Body::from(metrics))
                        .unwrap())
                }
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);
    server.await?;

    Ok(())
}
