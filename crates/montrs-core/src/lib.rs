pub mod signals;
pub mod router;
pub mod env;

pub use signals::Signal;
pub use router::{Router, Loader, Action, LoaderCtx, ActionCtx};
pub use env::{EnvConfig, TypedEnv};

use std::error::Error;
use async_trait::async_trait;

pub enum Target {
    Server,
    Wasm,
    Edge,
    Desktop,
    MobileAndroid,
    MobileIos,
}

#[async_trait]
pub trait Module<C: AppConfig>: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    
    async fn init(&self, ctx: &mut ModuleContext<C>) -> Result<(), Box<dyn Error + Send + Sync>>;
    
    fn register_routes(&self, _router: &mut Router<C>) {}
}

pub struct ModuleContext<'a, C: AppConfig> {
    pub config: &'a C,
    pub env: &'a dyn EnvConfig,
}

pub trait AppConfig: Sized + Send + Sync + 'static {
    type Error: Error + Send + Sync;
    type Env: EnvConfig;
}

pub struct AppSpec<C: AppConfig> {
    pub config: C,
    pub modules: Vec<Box<dyn Module<C>>>,
    pub env: C::Env,
    pub router: Router<C>,
    pub target: Target,
}

impl<C: AppConfig> AppSpec<C> {
    pub fn new(config: C, env: C::Env) -> Self {
        Self {
            config,
            modules: Vec::new(),
            env,
            router: Router::new(),
            target: Target::Server,
        }
    }

    pub fn with_module(mut self, module: Box<dyn Module<C>>) -> Self {
        self.modules.push(module);
        self
    }

    pub fn with_target(mut self, target: Target) -> Self {
        self.target = target;
        self
    }
}
