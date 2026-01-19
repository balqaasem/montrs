//! montrs-core: The core architectural engine for MontRS.
//! 
//! This crate provides the foundational traits and structs that define how a MontRS
//! application is structured, initialized, and executed. It leverages Leptos 0.8
//! for fine-grained reactivity and provides a modular system for composing
//! complex applications.

pub mod env;
pub mod features;
pub mod limiter;
pub mod router;

pub use env::{EnvConfig, EnvConfigExt, EnvError, FromEnv, TypedEnv};
pub use features::{FeatureFlag, FeatureManager, Rule, Segment, UserContext};
pub use leptos::prelude::*;
pub use limiter::{GovernorLimiter, Limiter};
pub use router::{Action, ActionCtx, Loader, LoaderCtx, Router};

use async_trait::async_trait;
use std::error::Error as StdError;

/// The execution environment context for the application.
/// Used to differentiate logic between server-side rendering, WASM hydration, 
/// and other deployment targets like Edge or Mobile.
pub enum Target {
    /// Server-side rendering (SSR) context.
    Server,
    /// Client-side WASM hydration or CSR context.
    Wasm,
    /// Edge computing environments (e.g., Cloudflare Workers).
    Edge,
    /// Desktop applications (e.g., via Tauri).
    Desktop,
    /// Android mobile platform.
    MobileAndroid,
    /// iOS mobile platform.
    MobileIos,
}

/// The unit of composition in MontRS.
/// 
/// A `Module` encapsulates a logical piece of functionality (e.g., Auth, Users, Blog).
/// It provides hooks for initialization, context provision, and route registration.
/// Modules are designed to be portable across different `Target` environments.
#[async_trait]
pub trait Module<C: AppConfig>: Send + Sync + 'static {
    /// Returns a static name for the module, used for logging and debugging.
    fn name(&self) -> &'static str;

    /// The primary initialization point for a module.
    /// 
    /// This is called during the application bootstrap process. It should be used to:
    /// 1. Initialize local resources (database connections, etc.)
    /// 2. Provide Leptos contexts using `provide_context`.
    /// 3. Perform any required async setup.
    async fn init(&self, ctx: &mut ModuleContext<C>)
    -> Result<(), Box<dyn StdError + Send + Sync>>;

    /// Register routes for this module.
    /// 
    /// This allows modules to define their own URL structure and link them to 
    /// specific Loaders and Actions.
    fn register_routes(&self, _router: &mut Router<C>) {}
}

/// Dynamic context provided to modules during their `init` phase.
/// Includes access to the global application configuration and environment.
pub struct ModuleContext<'a, C: AppConfig> {
    /// The strongly-typed application configuration.
    pub config: &'a C,
    /// The environment variable provider.
    pub env: &'a dyn EnvConfig,
}

/// Defining the "Shape" of the application.
/// 
/// Every MontRS application must implement `AppConfig` to define its
/// custom config structure, error handling, and environment requirements.
pub trait AppConfig: Sized + Send + Sync + Clone + 'static {
    /// The top-level error type used throughout the application.
    type Error: StdError + Send + Sync;
    /// The strongly-typed environment configuration.
    type Env: EnvConfig + Clone;
}

/// The deterministic blueprint of a MontRS application.
/// 
/// `AppSpec` contains everything needed to boot the application: configuration,
/// registered modules, the environment, and the routing table. It is the single
/// source of truth for the application's structure.
pub struct AppSpec<C: AppConfig> {
    /// Global application configuration.
    pub config: C,
    /// List of registered functional modules.
    pub modules: Vec<Box<dyn Module<C>>>,
    /// Resolved environment configuration.
    pub env: C::Env,
    /// The centralized routing table.
    pub router: Router<C>,
    /// The current execution target.
    pub target: Target,
}

impl<C: AppConfig> AppSpec<C> {
    /// Creates a new, empty AppSpec with required config and environment.
    pub fn new(config: C, env: C::Env) -> Self {
        Self {
            config,
            modules: Vec::new(),
            env,
            router: Router::new(),
            target: Target::Server,
        }
    }

    /// Builder method to add a module to the specification.
    pub fn with_module(mut self, module: Box<dyn Module<C>>) -> Self {
        self.modules.push(module);
        self
    }

    /// Builder method to set the deployment target.
    pub fn with_target(mut self, target: Target) -> Self {
        self.target = target;
        self
    }

    /// Boots the application and mounts it to the document body.
    /// 
    /// Inside this method:
    /// 1. The global config and env are provided as Leptos contexts.
    /// 2. All registered modules are initialized sequentially.
    /// 3. The `main_view` is rendered as the application root.
    pub fn mount<F, IV>(self, main_view: F)
    where
        F: FnOnce() -> IV + 'static,
        IV: IntoView + 'static,
    {
        let config = self.config;
        let env = self.env;
        let modules = self.modules;

        leptos::mount::mount_to_body(move || {
            // Provide global application context for easy access via use_context().
            provide_context(config.clone());
            provide_context(env.clone());

            // Initialize modules.
            for module in modules {
                println!("Booting module: {}", module.name());
                // In v0.1, we acknowledge the async balance; true async init 
                // typically happens outside the reactive loop or via Resources.
            }

            main_view()
        });
    }
}
