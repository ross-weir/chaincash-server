use axum::{routing::get, Router};

/// Serves the ChainCash payment server on the given listener forever.
pub async fn serve_blocking(listener: std::net::TcpListener) -> Result<(), crate::Error> {
    let app = Router::new().route("/healthcheck", get(|| async { "ok" }));

    axum::Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
