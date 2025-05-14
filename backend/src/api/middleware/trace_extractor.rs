// INFO: copy of https://github.com/davidB/tracing-opentelemetry-instrumentation-sdk/blob/main/axum-tracing-opentelemetry/src/middleware/trace_extractor.rs which for some reason does not work standalone

//
//! `OpenTelemetry` tracing middleware.
//!
//! This returns a [`OtelAxumLayer`] configured to use [`OpenTelemetry`'s conventional span field
//! names][otel].
//!
//! # Span fields
//!
//! Try to provide some of the field define at
//! [semantic-conventions/.../http-spans.md](https://github.com/open-telemetry/semantic-conventions/blob/v1.25.0/docs/http/http-spans.md)
//! (Please report or provide fix for missing one)
//!
//! # Example
//!
//! ```
//! use axum::{Router, routing::get, http::Request};
//! use axum_tracing_opentelemetry::middleware::OtelAxumLayer;
//! use std::net::SocketAddr;
//! use tower::ServiceBuilder;
//!
//! let app = Router::new()
//!     .route("/", get(|| async {}))
//!     .layer(OtelAxumLayer::default());
//!
//! # async {
//! let addr = &"0.0.0.0:3000".parse::<SocketAddr>().unwrap();
//! let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
//! axum::serve(listener, app.into_make_service())
//!     .await
//!     .expect("server failed");
//! # };
//! ```
//!

use axum::extract::MatchedPath;
use http::{Request, Response};
use pin_project_lite::pin_project;
use std::{
    error::Error,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower::{Layer, Service};
use tracing::Span;
use tracing_opentelemetry_instrumentation_sdk::http as otel_http;


pub type Filter = fn(&str) -> bool;

/// layer/middleware for axum:
///
/// - propagate `OpenTelemetry` context (`trace_id`,...) to server
/// - create a Span for `OpenTelemetry` (and tracing) on call
///
/// `OpenTelemetry` context are extracted from tracing's span.
#[derive(Default, Debug, Clone)]
pub struct OtelAxumLayer {
    filter: Option<Filter>,
}

// add a builder like api
#[allow(dead_code)]
impl OtelAxumLayer {
    #[must_use]
    pub fn filter(self, filter: Filter) -> Self {
        OtelAxumLayer {
            filter: Some(filter),
        }
    }
}

impl<S> Layer<S> for OtelAxumLayer {
    /// The wrapped service
    type Service = OtelAxumService<S>;
    fn layer(&self, inner: S) -> Self::Service {
        OtelAxumService {
            inner,
            filter: self.filter,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OtelAxumService<S> {
    inner: S,
    filter: Option<Filter>,
}

impl<S, B, B2> Service<Request<B>> for OtelAxumService<S>
where
    S: Service<Request<B>, Response = Response<B2>> + Clone + Send + 'static,
    S::Error: Error + 'static, //fmt::Display + 'static,
    S::Future: Send + 'static,
    B: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    // #[allow(clippy::type_complexity)]
    // type Future = futures_core::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        use tracing_opentelemetry::OpenTelemetrySpanExt;
        let req = req;
        let span = if self.filter.is_none_or(|f| f(req.uri().path())) {
            let span = otel_http::http_server::make_span_from_request(&req);
            let route = http_route(&req);
            let method = otel_http::http_method(req.method());
            // let client_ip = parse_x_forwarded_for(req.headers())
            //     .or_else(|| {
            //         req.extensions()
            //             .get::<ConnectInfo<SocketAddr>>()
            //             .map(|ConnectInfo(client_ip)| Cow::from(client_ip.to_string()))
            //     })
            //     .unwrap_or_default();
            span.record("http.route", route);
            span.record("otel.name", format!("{method} {route}").trim());
            // span.record("trace_id", find_trace_id_from_tracing(&span));
            // span.record("client.address", client_ip);
            span.set_parent(otel_http::extract_context(req.headers()));
            span
        } else {
            tracing::Span::none()
        };
        let future = {
            let _enter = span.enter();
            self.inner.call(req)
        };
        ResponseFuture {
            inner: future,
            span,
        }
    }
}

pin_project! {
    /// Response future for [`Trace`].
    ///
    /// [`Trace`]: super::Trace
    pub struct ResponseFuture<F> {
        #[pin]
        pub(crate) inner: F,
        pub(crate) span: Span,
        // pub(crate) start: Instant,
    }
}

impl<Fut, ResBody, E> Future for ResponseFuture<Fut>
where
    Fut: Future<Output = Result<Response<ResBody>, E>>,
    E: std::error::Error + 'static,
{
    type Output = Result<Response<ResBody>, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let _guard = this.span.enter();
        let result = futures_util::ready!(this.inner.poll(cx));
        otel_http::http_server::update_span_from_response_or_error(this.span, &result);
        Poll::Ready(result)
    }
}

#[inline]
fn http_route<B>(req: &Request<B>) -> &str {
    req.extensions()
        .get::<MatchedPath>()
        .map_or_else(|| "", |mp| mp.as_str())
}
