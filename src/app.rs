//! ChainCash payment server creation and serving.
use axum::{routing::get, Router};

fn make_app() -> Router {
    Router::new().route("/healthcheck", get(|| async { "ok" }))
}

/// Serves the ChainCash payment server on the given listener forever.
///
/// # Example
///
/// ```
/// # async fn run() {
/// use std::net::TcpListener;
///
/// let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
///
/// chaincash_server::serve_blocking(listener).await.unwrap();
/// # }
/// ```
pub async fn serve_blocking(listener: std::net::TcpListener) -> Result<(), crate::Error> {
    axum::Server::from_tcp(listener)?
        .serve(make_app().into_make_service())
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use hyper::{Request, StatusCode};
    use tower::ServiceExt;

    use super::*;

    #[tokio::test]
    async fn test_healthcheck() {
        let response = make_app()
            .oneshot(
                Request::get("/healthcheck")
                    .body(Default::default())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
