mod routes;
mod model;
mod observability;

use std::future::ready;
use crate::routes::{graphql_handler, graphql_playground, health};
use axum::{Router, Server, routing::get, extract::Extension, middleware};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use crate::model::QueryRoot;
use crate::observability::metrics::{create_prometheus_recoder, track_metrics};


#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();
    let prometheus_recorder = create_prometheus_recoder();
    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .route("/health", get(health))
        .route("/metrics", get(move || ready(prometheus_recorder.render())))
        .route_layer(middleware::from_fn(track_metrics))
        .layer(Extension(schema));
    Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}

