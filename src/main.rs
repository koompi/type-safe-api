use crate::api::Ctx;
use axum::{
    http::{HeaderValue, Method},
    routing::get,
};
use mongodb::Client;
use rspc::{selection, Config};
use rspc::{ErrorCode, MiddlewareContext, Type};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::cors::CorsLayer;

mod api;
mod utils;

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub struct User {
    pub name: String,
    pub email: String,
}

async fn router() -> axum::Router {
    let client = mongodb::Client::with_uri_str("mongodb://127.0.0.1:27017")
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
        .middleware(|mw| {
            mw.middleware(|mw| async move {
                let state = (mw.req.clone(), mw.ctx.clone(), mw.input.clone());
                Ok(mw.with_state(state))
            })
            .resp(|state, result| async move {
                println!(
                    "[LOG] req='{:?}' ctx='{:?}'  input='{:?}' result='{:?}'",
                    state.0, state.1, state.2, result
                );
                Ok(result)
            })
        })
        .query("name", |t| t(|ctx, input: ()| "brilliant"))
        .query("users", |t| {
            t(|ctx, input: User| {
                let col = ctx.db.database("rspc").collection::<User>("users");

                input
            })
        })
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
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET]),
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
