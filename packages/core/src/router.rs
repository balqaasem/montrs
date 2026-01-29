//! montrs-core/src/router.rs: Explicit routing primitives inspired by Remix.
//! This file defines Loaders for data fetching and Actions for data mutation,
//! ensuring clear boundaries between reads and writes.

use crate::AppConfig;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Trait for data loading components. Loaders are responsible for fetching data
/// for a specific route. They are read-only and idempotent.
#[async_trait]
pub trait Loader<C: AppConfig>: Send + Sync + 'static {
    async fn call(
        &self,
        ctx: LoaderCtx<C>,
    ) -> Result<LoaderResponse, Box<dyn std::error::Error + Send + Sync>>;

    /// Returns a description of what this loader fetches.
    fn description(&self) -> &'static str {
        ""
    }

    /// Returns the expected JSON schema for the input data (e.g., query params).
    fn input_schema(&self) -> Option<serde_json::Value> {
        None
    }

    /// Returns the expected JSON schema for the output data.
    fn output_schema(&self) -> Option<serde_json::Value> {
        None
    }
}

/// Trait for data mutation components. Actions are responsible for handling
/// state-changing operations (form submissions, API mutations).
#[async_trait]
pub trait Action<C: AppConfig>: Send + Sync + 'static {
    async fn call(
        &self,
        input: serde_json::Value,
        ctx: ActionCtx<C>,
    ) -> Result<ActionResponse, Box<dyn std::error::Error + Send + Sync>>;

    /// Returns a description of what this action does.
    fn description(&self) -> &'static str {
        ""
    }

    /// Returns the expected JSON schema for the input data.
    fn input_schema(&self) -> Option<serde_json::Value> {
        None
    }

    /// Returns the expected JSON schema for the output data.
    fn output_schema(&self) -> Option<serde_json::Value> {
        None
    }
}

/// Context passed to a Loader, providing access to the application configuration.
pub struct LoaderCtx<C: AppConfig> {
    pub config: C,
}

/// Context passed to an Action, providing access to the application configuration.
pub struct ActionCtx<C: AppConfig> {
    pub config: C,
}

/// Standard response format for a Loader.
#[derive(Serialize, Deserialize)]
pub struct LoaderResponse {
    pub data: serde_json::Value,
}

/// Standard response format for an Action.
#[derive(Serialize, Deserialize)]
pub struct ActionResponse {
    pub data: serde_json::Value,
}

/// The Application Router which maintains a mapping of paths to Loaders and Actions.
pub struct Router<C: AppConfig> {
    loaders: HashMap<String, Box<dyn Loader<C>>>,
    actions: HashMap<String, Box<dyn Action<C>>>,
}

/// A machine-readable specification of the router, useful for agents and documentation.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RouterSpec {
    pub loaders: HashMap<String, LoaderMetadata>,
    pub actions: HashMap<String, ActionMetadata>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoaderMetadata {
    pub description: String,
    pub input_schema: Option<serde_json::Value>,
    pub output_schema: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActionMetadata {
    pub description: String,
    pub input_schema: Option<serde_json::Value>,
    pub output_schema: Option<serde_json::Value>,
}

impl<C: AppConfig> Router<C> {
    /// Generates a serializable specification of the current router state.
    pub fn spec(&self) -> RouterSpec {
        let mut loaders = HashMap::new();
        for (path, loader) in &self.loaders {
            loaders.insert(
                path.clone(),
                LoaderMetadata {
                    description: loader.description().to_string(),
                    input_schema: loader.input_schema(),
                    output_schema: loader.output_schema(),
                },
            );
        }

        let mut actions = HashMap::new();
        for (path, action) in &self.actions {
            actions.insert(
                path.clone(),
                ActionMetadata {
                    description: action.description().to_string(),
                    input_schema: action.input_schema(),
                    output_schema: action.output_schema(),
                },
            );
        }

        RouterSpec { loaders, actions }
    }
}

impl<C: AppConfig> Default for Router<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: AppConfig> Router<C> {
    /// Initializes a new, empty router.
    pub fn new() -> Self {
        Self {
            loaders: HashMap::new(),
            actions: HashMap::new(),
        }
    }

    /// Registers a loader for a specific path.
    pub fn register_loader(&mut self, path: &str, loader: Box<dyn Loader<C>>) {
        self.loaders.insert(path.to_string(), loader);
    }

    /// Registers an action for a specific path.
    pub fn register_action(&mut self, path: &str, action: Box<dyn Action<C>>) {
        self.actions.insert(path.to_string(), action);
    }
}
