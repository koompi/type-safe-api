use crate::api::Ctx;
use axum::{
    http::{HeaderValue, Method,},
    routing::get,
};
use mongodb::{Client, bson::DateTime};
use rspc::{selection, Config};
use rspc::{ErrorCode, MiddlewareContext, Type};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::cors::CorsLayer;
use futures::stream::StreamExt;

mod api;
mod utils;

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub struct User {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub struct Todo {
    pub title: String,
}

async fn router() -> axum::Router {
    let client = mongodb::Client::with_uri_str("mongodb+srv://soklay:soklay123@soklay.fmepuar.mongodb.net/")
        .await
        .unwrap();

    let router = api::new()
        .config(
            Config::new()
                // Doing this will automatically export the bindings when the `build` function is called.
                .export_ts_bindings(
                    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("./web/src/utils/bindings.ts"),
                )
                .set_ts_bindings_header("/* eslint-disable */"),
        )
        
        .query("name", |t| t(|ctx, input: ()| "brilliant"))
        .query("users", |t| {
            t(|ctx, input: User| {
                let col = ctx.db.database("rspc").collection::<User>("users");

                input
            })
        })
        .query("todos", |t| {
            t( |ctx, input: ()| async move {
                let col = ctx.db.database("rspc").collection::<Todo>("todos");
                let mut todos = col.find(None, None).await.unwrap();
                let mut todo = Vec::new();
                while let Some(data) = todos.next().await {
                    todo.push(data.unwrap());
                }
                todo
            })
        })
        .mutation("create_todos",  |t| t(|ctx, new_todo: Todo| async move {
            let col = ctx.db.database("rspc").collection::<Todo>("todos");
            col.insert_one(new_todo, None).await.unwrap();
            
        }))
        .build()
        .arced();

    axum::Router::new()
        .route("/", get(|| async { "Welcome to your new rspc app!" }))
        .route("/health", get(|| async { "Ok!" }))
        .nest(
            "/rspc",
            router
                .endpoint(|| Ctx {
                    db: client,
                    user: None,
                })
                .axum(),
        )
        .layer(
            CorsLayer::new()
                .allow_methods(tower_http::cors::Any)
                .allow_methods(tower_http::cors::Any)
                .allow_headers(tower_http::cors::Any)
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST]),
        )
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let addr = "[::]:9000".parse::<SocketAddr>().unwrap(); // This listens on IPv6 and IPv4
    println!("{} listening on http://{}", env!("CARGO_CRATE_NAME"), addr);
    axum::Server::bind(&addr)
        .serve(router().await.into_make_service())
        .with_graceful_shutdown(utils::axum_shutdown_signal())
        .await
        .expect("Error with HTTP server!");
}
