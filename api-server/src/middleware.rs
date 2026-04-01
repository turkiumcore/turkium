use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use tracing::info;

pub async fn logging_middleware(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();

    info!("[{}] {}", method, uri);

    next.run(req).await
}
