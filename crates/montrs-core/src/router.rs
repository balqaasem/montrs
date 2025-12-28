use std::collections::HashMap;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::AppConfig;

#[async_trait]
pub trait Loader<C: AppConfig>: Send + Sync + 'static {
    async fn call(&self, ctx: LoaderCtx<C>) -> Result<LoaderResponse, crate::signals::Signal<String>>; // Simplified error for now
}

#[async_trait]
pub trait Action<C: AppConfig>: Send + Sync + 'static {
    async fn call(&self, input: serde_json::Value, ctx: ActionCtx<C>) -> Result<ActionResponse, crate::signals::Signal<String>>;
}

pub struct LoaderCtx<C: AppConfig> {
    pub config: C,
}

pub struct ActionCtx<C: AppConfig> {
    pub config: C,
}

#[derive(Serialize, Deserialize)]
pub struct LoaderResponse {
    pub data: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct ActionResponse {
    pub data: serde_json::Value,
}

pub struct Router<C: AppConfig> {
    loaders: HashMap<String, Box<dyn Loader<C>>>,
    actions: HashMap<String, Box<dyn Action<C>>>,
}

impl<C: AppConfig> Router<C> {
    pub fn new() -> Self {
        Self {
            loaders: HashMap::new(),
            actions: HashMap::new(),
        }
    }

    pub fn register_loader(&mut self, path: &str, loader: Box<dyn Loader<C>>) {
        self.loaders.insert(path.to_string(), loader);
    }

    pub fn register_action(&mut self, path: &str, action: Box<dyn Action<C>>) {
        self.actions.insert(path.to_string(), action);
    }
}
