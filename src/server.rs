use axum::Router;
use axum::http::{Request, Response};
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;
use tower::{Layer, Service};

#[cfg(not(feature = "bundle"))]
use std::path::PathBuf;
#[cfg(not(feature = "bundle"))]
use tower_http::services::{ServeDir, ServeFile};

#[cfg(feature = "bundle")]
use crate::embedded;
#[cfg(feature = "bundle")]
use axum::extract::Path;
#[cfg(feature = "bundle")]
use axum::http::StatusCode;
#[cfg(feature = "bundle")]
use axum::http::header;
#[cfg(feature = "bundle")]
use axum::response::IntoResponse;
#[cfg(feature = "bundle")]
use axum::routing::get;

#[derive(Clone)]
struct RequestLoggerLayer;

impl<S> Layer<S> for RequestLoggerLayer {
    type Service = RequestLogger<S>;

    fn layer(&self, service: S) -> Self::Service {
        RequestLogger { inner: service }
    }
}

#[derive(Clone)]
struct RequestLogger<S> {
    inner: S,
}

impl<S, B, B2> Service<Request<B>> for RequestLogger<S>
where
    S: Service<Request<B>, Response = Response<B2>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    B: Send + 'static,
    B2: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let mut svc = self.inner.clone();

        let method = req.method().clone();
        let path = req.uri().path().to_string();
        let start_time = Instant::now();

        Box::pin(async move {
            let response = svc.call(req).await?;

            let status = response.status();
            let duration = start_time.elapsed();

            log::info!("{} {} {} - {:?}", method, path, status.as_u16(), duration);

            Ok(response)
        })
    }
}

pub async fn start_server(args: &crate::args::Args) -> Result<(), Box<dyn std::error::Error>> {
    let addr = match parse_listen_addr(&args.listen) {
        Ok(addr) => addr,
        Err(e) => {
            log::error!("Failed to parse listen address: {}", e);
            return Err(e.into());
        }
    };

    let app = build_app(args);
    let listener = tokio::net::TcpListener::bind(addr).await?;

    #[cfg(feature = "bundle")]
    log::info!("Server started at {} with bundled assets", addr);

    #[cfg(not(feature = "bundle"))]
    log::info!(
        "Server started at {}, serving from {}",
        addr,
        PathBuf::from(&args.serve_dir).display()
    );

    axum::serve(listener, app).await?;

    Ok(())
}

fn build_app(args: &crate::args::Args) -> Router {
    #[cfg(feature = "bundle")]
    {
        let app = Router::new().route("/", get(embedded_index));

        // Choose the appropriate handler based on args.index
        let app = if args.index {
            app.route("/*path", get(embedded_handler_with_index_fallback))
        } else {
            app.route("/*path", get(embedded_handler_without_fallback))
        };

        app.layer(RequestLoggerLayer)
    }

    #[cfg(not(feature = "bundle"))]
    {
        let serve_dir_path = PathBuf::from(&args.serve_dir);

        let app = if args.index {
            Router::new().fallback_service(
                ServeDir::new(&serve_dir_path)
                    .not_found_service(ServeFile::new(serve_dir_path.join("index.html")))
                    .not_found_service(ServeFile::new(serve_dir_path.join("404.html"))),
            )
        } else {
            Router::new().fallback_service(ServeDir::new(&serve_dir_path))
        };

        app.layer(RequestLoggerLayer)
    }
}

#[cfg(feature = "bundle")]
async fn embedded_index() -> impl IntoResponse {
    match embedded::get_embedded_file("index.html") {
        Some(data) => ([(header::CONTENT_TYPE, "text/html")], data).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            [(header::CONTENT_TYPE, "text/plain")],
            "Not Found",
        )
            .into_response(),
    }
}

#[cfg(feature = "bundle")]
async fn embedded_handler_with_index_fallback(Path(path): Path<String>) -> impl IntoResponse {
    match embedded::get_embedded_file(&path) {
        Some(data) => {
            let mime_type = embedded::get_embedded_file_type(&path);
            ([(header::CONTENT_TYPE, mime_type)], data).into_response()
        }
        None => {
            // Try to serve index.html if requested file not found (SPA typical behavior)
            match embedded::get_embedded_file("index.html") {
                Some(data) => ([(header::CONTENT_TYPE, "text/html")], data).into_response(),
                None => (
                    StatusCode::NOT_FOUND,
                    [(header::CONTENT_TYPE, "text/plain")],
                    "Not Found",
                )
                    .into_response(),
            }
        }
    }
}

#[cfg(feature = "bundle")]
async fn embedded_handler_without_fallback(Path(path): Path<String>) -> impl IntoResponse {
    match embedded::get_embedded_file(&path) {
        Some(data) => {
            let mime_type = embedded::get_embedded_file_type(&path);
            ([(header::CONTENT_TYPE, mime_type)], data).into_response()
        }
        None => (
            StatusCode::NOT_FOUND,
            [(header::CONTENT_TYPE, "text/plain")],
            "Not Found",
        )
            .into_response(),
    }
}

fn parse_listen_addr(addr: &str) -> Result<SocketAddr, Box<dyn std::error::Error>> {
    if addr.starts_with(':') {
        let port = addr[1..].parse::<u16>()?;
        Ok(([0, 0, 0, 0], port).into())
    } else {
        Ok(addr.parse()?)
    }
}
