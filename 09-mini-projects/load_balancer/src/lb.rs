use crate::config::AppConfig;
use crate::plugins::Plugin;
use arc_swap::ArcSwap;
use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper::{Request, Response, StatusCode};
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::client::legacy::Client;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// The Core Load Balancer Engine Structure
pub struct LoadBalancer {
    pub config: Arc<ArcSwap<AppConfig>>,
    pub client: Client<HttpConnector, Incoming>,
    pub plugins: Vec<Box<dyn Plugin>>,
    pub rr_counter: AtomicUsize,
}

impl LoadBalancer {
    /// Primary entry point for all the incoming requests
    pub async fn proxy(
        &self,
        mut req: Request<Incoming>,
    ) -> Result<Response<Full<Bytes>>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        // Run plugins before forwarding the request
        for plugin in &self.plugins {
            plugin.on_request(&mut req);
        }

        // Access the current state of healthy backends
        let curr_config = self.config.load();
        let live_backends = &curr_config.live_backends;

        // Safety: Check if we have any live backends
        if live_backends.is_empty() {
            return Ok(self.service_unavailable());
        };

        // Simple Round-Robin Selection
        let idx = self.rr_counter.fetch_add(1, Ordering::Relaxed) % live_backends.len();
        let target_base_url = &live_backends[idx];
        let path_and_query = req
            .uri()
            .path_and_query()
            .map(|pq| pq.as_str())
            .unwrap_or("");
        let new_uri_string = format!("{}{}", target_base_url, path_and_query);

        match new_uri_string.parse::<hyper::Uri>() {
            Ok(new_uri) => {
                *req.uri_mut() = new_uri;

                // the "Host" header and "X-Forwarded-For" related logic
                if let Some(host) = req.uri().host() {
                    let host_value = hyper::header::HeaderValue::from_str(host).unwrap();
                    req.headers_mut().insert(hyper::header::HOST, host_value);
                }

                // Forward the request to the selected backend
                let backend_resp = match self.client.request(req).await {
                    Ok(r) => r,
                    Err(e) => return Err(Box::new(e)),
                };

                // Buffer the backend body into memory so we can return a concrete body type
                let status = backend_resp.status();
                let headers = backend_resp.headers().clone();
                let collected = backend_resp.into_body().collect().await.map_err(|e| {
                    Box::new(e) as Box<dyn std::error::Error + Send + Sync + 'static>
                })?;

                let full = Full::from(collected.to_bytes());

                let mut builder = Response::builder().status(status);
                // copy headers (if any) from whatever the backend provided
                if let Some(h) = builder.headers_mut() {
                    for (k, v) in headers.iter() {
                        h.insert(k.clone(), v.clone());
                    }
                }

                Ok(builder.body(full).unwrap())
            }
            Err(_) => Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Full::from(Bytes::new()))
                .unwrap()),
        }
    }

    /// Helper for returning a 503 when no backends are healthy
    fn service_unavailable(&self) -> Response<Full<Bytes>> {
        Response::builder()
            .status(StatusCode::SERVICE_UNAVAILABLE)
            .body(Full::from(Bytes::new()))
            .unwrap()
    }
}
