pub use rspc::RouterBuilder;
use rspc::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub struct User {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone)]
pub struct Ctx {
    pub db: mongodb::Client,
    pub user: Option<User>,
}

pub type Router = rspc::Router<Ctx>;

pub(crate) fn new() -> RouterBuilder<Ctx> {
    Router::new().query("version", |t| t(|_, _: ()| env!("CARGO_PKG_VERSION")))
}
