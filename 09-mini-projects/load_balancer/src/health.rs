use crate::config::AppConfig;
use arc_swap::ArcSwap;
use http_body_util::Empty;
use hyper::body::Bytes;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::client::legacy::Client;
use std::sync::Arc;
use std::time::Duration;

pub async fn start_health_checks(config_arc: Arc<ArcSwap<AppConfig>>, interval: Duration) {
    let client: Client<HttpConnector, Empty<Bytes>> =
        Client::builder(hyper_util::rt::TokioExecutor::new()).build_http();

    loop {
        tokio::time::sleep(interval).await;

        // 1. Snapshot the current config
        let current_conf = config_arc.load();
        let mut healthy_list = Vec::new();

        // 2. Concurrently check all backends
        for backend in &current_conf.all_backends {
            if check_backend(&client, backend).await {
                healthy_list.push(backend.clone());
            }
        }

        // 3. Update ArcSwap if the list changed
        if healthy_list != current_conf.live_backends {
            println!(
                "🏥 Health Check: {}/{} backends live",
                healthy_list.len(),
                current_conf.all_backends.len()
            );

            let mut new_conf = (**current_conf).clone();
            new_conf.live_backends = healthy_list;
            config_arc.store(Arc::new(new_conf));
        }
    }
}

async fn check_backend(client: &Client<HttpConnector, Empty<Bytes>>, url: &str) -> bool {
    let Ok(uri) = url.parse::<hyper::Uri>() else {
        return false;
    };

    // We use a short timeout so a dead backend doesn't hang the checker
    match tokio::time::timeout(Duration::from_secs(2), client.get(uri)).await {
        Ok(Ok(res)) => res.status().is_success(),
        _ => false,
    }
}
