use tower_http::{
  classify::{ServerErrorsAsFailures, SharedClassifier},
  trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logger() {
  tracing_subscriber::registry()
    .with(
      tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        // axum logs rejections from built-in extractors with the `axum::rejection`
        // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
        "server=debug,tower_http=debug,axum::rejection=trace".into()
      }),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();
}

pub fn create_logger_middleware() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
  TraceLayer::new_for_http()
    .make_span_with(DefaultMakeSpan::new().include_headers(true))
    .on_request(DefaultOnRequest::new().level(tracing::Level::INFO))
    .on_response(DefaultOnResponse::new().level(tracing::Level::INFO))
}
