use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use montrs_core::{EnvConfig, AppSpec, AppConfig};
use montrs_core::env::{FromEnv, EnvError};

pub struct TestEnv {
    vars: Arc<RwLock<HashMap<String, String>>>,
}

impl TestEnv {
    pub fn new() -> Self {
        Self {
            vars: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn set(&self, key: &str, value: &str) {
        let mut vars = self.vars.write().unwrap();
        vars.insert(key.to_string(), value.to_string());
    }
}

impl EnvConfig for TestEnv {
    fn get_var(&self, key: &str) -> Result<String, EnvError> {
        let vars = self.vars.read().unwrap();
        vars.get(key)
            .cloned()
            .ok_or_else(|| EnvError::MissingKey(key.to_string()))
    }
}

pub struct TestRuntime<C: AppConfig> {
    pub spec: AppSpec<C>,
}

impl<C: AppConfig> TestRuntime<C> {
    pub fn new(spec: AppSpec<C>) -> Self {
        Self { spec }
    }

    pub async fn execute<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&AppSpec<C>) -> R,
    {
        // In a real implementation, this would set up the reactive context,
        // potentially a tokio task local for the runtime, etc.
        f(&self.spec)
    }
}
