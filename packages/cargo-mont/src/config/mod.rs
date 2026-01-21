use anyhow::{Context, Result};
use camino::{Utf8Path, Utf8PathBuf};
use cargo_metadata::MetadataCommand;
use serde::Deserialize;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};

pub mod tailwind;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct MontConfig {
    #[serde(default)]
    pub project: ProjectConfig,
    #[serde(default)]
    pub build: BuildConfig,
    #[serde(default)]
    pub serve: ServeConfig,
    #[serde(default)]
    pub tasks: HashMap<String, TaskConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProjectConfig {
    #[serde(default = "default_app_name")]
    pub name: String,
    #[serde(skip)]
    pub verbose: u8,
    #[serde(skip)]
    pub log: Vec<cargo_leptos::config::Log>,
    #[serde(skip)]
    pub release: bool,
    #[serde(skip)]
    pub hot_reload: bool,
    #[serde(skip)]
    pub precompress: bool,
    #[serde(skip)]
    pub wasm_debug: bool,
    #[serde(skip)]
    pub js_minify: bool,
    #[serde(skip)]
    pub split: bool,
    #[serde(skip)]
    pub frontend_only: bool,
    #[serde(skip)]
    pub server_only: bool,
    #[serde(skip)]
    pub features: Vec<String>,
    #[serde(skip)]
    pub tailwind_style: Option<TailwindStyle>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TailwindStyle {
    #[default]
    Auto,
    Toml,
    V4,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: default_app_name(),
            verbose: 0,
            log: Vec::new(),
            release: false,
            hot_reload: false,
            precompress: false,
            wasm_debug: false,
            js_minify: true,
            split: false,
            frontend_only: false,
            server_only: false,
            features: Vec::new(),
        }
    }
}

fn default_app_name() -> String {
    "app".to_string()
}

#[derive(Debug, Deserialize, Clone)]
pub struct BuildConfig {
    #[serde(default = "default_target")]
    pub target: String,
    #[serde(default = "default_dist")]
    pub dist: String,
    #[serde(default = "default_site_root")]
    pub site_root: String,
    #[serde(default = "default_site_pkg_name")]
    pub site_pkg_name: String,
    #[serde(default = "default_assets_dir")]
    pub assets_dir: Option<String>,
    #[serde(default)]
    pub tailwind_input_file: Option<String>,
    #[serde(rename = "tailwind-config-file")]
    pub tailwind_config_file: Option<String>,
    #[serde(rename = "style-file")]
    pub style_file: Option<String>,
    #[serde(default = "default_browserquery")]
    pub browserquery: String,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            target: default_target(),
            dist: default_dist(),
            site_root: default_site_root(),
            site_pkg_name: default_site_pkg_name(),
            assets_dir: None,
            tailwind_input_file: None,
            tailwind_config_file: None,
            style_file: None,
            browserquery: default_browserquery(),
        }
    }
}

fn default_browserquery() -> String {
    "defaults".to_string()
}

fn default_target() -> String {
    "index.html".to_string()
}
fn default_dist() -> String {
    "dist".to_string()
}
fn default_site_root() -> String {
    "target/site".to_string()
}
fn default_site_pkg_name() -> String {
    "pkg".to_string()
}
fn default_assets_dir() -> Option<String> {
    None
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServeConfig {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_addr")]
    pub addr: String,
}

impl Default for ServeConfig {
    fn default() -> Self {
        Self {
            port: default_port(),
            addr: default_addr(),
        }
    }
}

fn default_port() -> u16 {
    8080
}
fn default_addr() -> String {
    "127.0.0.1".to_string()
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum TaskConfig {
    Simple(String),
    Detailed {
        command: String,
        #[serde(default)]
        description: Option<String>,
        #[serde(default)]
        category: Option<String>,
        #[serde(default)]
        dependencies: Vec<String>,
        #[serde(default)]
        env: HashMap<String, String>,
    },
}

impl MontConfig {
    pub fn load() -> Result<Self> {
        let metadata = MetadataCommand::new()
            .exec()
            .map_err(|e| anyhow::anyhow!("Failed to load cargo metadata: {}", e))?;

        let mut config = if let Ok(content) = std::fs::read_to_string("mont.toml") {
            toml::from_str(&content).context("Failed to parse mont.toml")?
        } else {
            Self::default()
        };

        if let Some(root) = metadata.root_package()
            && config.project.name == "app"
        {
            config.project.name = root.name.clone();
        }

        Ok(config)
    }

    pub fn to_leptos_config(&self, watch: bool) -> Result<cargo_leptos::config::Config> {
        let metadata = MetadataCommand::new()
            .exec()
            .map_err(|e| anyhow::anyhow!("Failed to load cargo metadata: {}", e))?;

        let mut opts = cargo_leptos::config::Opts::default();
        opts.project = Some(self.project.name.clone());
        opts.verbose = self.project.verbose;
        opts.log = self.project.log.clone();
        opts.release = self.project.release;
        opts.hot_reload = self.project.hot_reload;
        opts.precompress = self.project.precompress;
        opts.wasm_debug = self.project.wasm_debug;
        opts.js_minify = self.project.js_minify;
        opts.split = self.project.split;
        opts.frontend_only = self.project.frontend_only;
        opts.server_only = self.project.server_only;
        opts.features = self.project.features.clone();

        let mut proj_conf = cargo_leptos::config::ProjectConfig {
            output_name: self.project.name.clone(),
            site_addr: format!("{}:{}", self.serve.addr, self.serve.port)
                .parse::<SocketAddr>()
                .context("Invalid site address")?,
            site_root: Utf8PathBuf::from(&self.build.site_root),
            site_pkg_dir: Utf8PathBuf::from(&self.build.site_pkg_name),
            style_file: self.build.style_file.as_ref().map(Utf8PathBuf::from),
            hash_file_name: None,
            hash_files: false,
            tailwind_input_file: self
                .build
                .tailwind_input_file
                .as_ref()
                .map(Utf8PathBuf::from),
            tailwind_config_file: self
                .build
                .tailwind_config_file
                .as_ref()
                .map(Utf8PathBuf::from),
            assets_dir: self.build.assets_dir.as_ref().map(Utf8PathBuf::from),
            js_dir: None,
            js_minify: true,
            watch_additional_files: None,
            reload_port: 3001,
            end2end_cmd: None,
            end2end_dir: None,
            browserquery: self.build.browserquery.clone(),
            bin_target: String::new(),
            bin_target_triple: None,
            bin_target_dir: None,
            bin_cargo_command: None,
            bin_cargo_args: None,
            bin_exe_name: None,
            features: Vec::new(),
            lib_features: Vec::new(),
            lib_default_features: true,
            lib_cargo_args: None,
            bin_features: Vec::new(),
            bin_default_features: true,
            server_fn_prefix: None,
            disable_server_fn_hash: false,
            disable_erase_components: false,
            always_erase_components: false,
            server_fn_mod_path: false,
            config_dir: Utf8PathBuf::from("."),
            tmp_dir: metadata.target_directory.join("tmp"),
            separate_front_target_dir: None,
            lib_profile_dev: None,
            lib_profile_release: None,
            bin_profile_dev: None,
            bin_profile_release: None,
            wasm_opt_features: None,
        };

        // If cargo-leptos has these types and functions as public, we can use them:
        // Note: ProjectDefinition is not usually public but we can try Constructing it or using a public path if available.
        // Actually, Project has a public constructor-like logic in Project::resolve.

        // Since we want to ENSURE our mont.toml data is used, we have to construct the Project fields.

        let project_def = cargo_leptos::config::ProjectDefinition {
            name: self.project.name.clone(),
            bin_package: self.project.name.clone(),
            lib_package: self.project.name.clone(),
        };

        let cwd = Utf8PathBuf::from(".");

        let lib =
            cargo_leptos::config::LibPackage::resolve(&opts, &metadata, &project_def, &proj_conf)
                .map_err(|e| anyhow::anyhow!("Failed to resolve lib package: {}", e))?;

        let bin = cargo_leptos::config::BinPackage::resolve(
            &opts,
            &metadata,
            &project_def,
            &proj_conf,
            None,
        )
        .map_err(|e| anyhow::anyhow!("Failed to resolve bin package: {}", e))?;

        let style = cargo_leptos::config::StyleConfig::new(&proj_conf)
            .map_err(|e| anyhow::anyhow!("Failed to resolve style config: {}", e))?;

        let site = Arc::new(cargo_leptos::service::site::Site::new(&proj_conf));

        let hash_file =
            cargo_leptos::config::HashFile::new(Some(&metadata.workspace_root), &bin, None);

        let project = cargo_leptos::config::Project {
            working_dir: metadata.workspace_root.clone(),
            name: self.project.name.clone(),
            lib,
            bin,
            style,
            watch,
            release: opts.release,
            precompress: opts.precompress,
            hot_reload: opts.hot_reload,
            wasm_debug: opts.wasm_debug,
            site,
            end2end: cargo_leptos::config::End2EndConfig::resolve(&proj_conf),
            assets: cargo_leptos::config::AssetsConfig::resolve(&proj_conf),
            js_dir: Utf8PathBuf::from("src"),
            watch_additional_files: Vec::new(),
            hash_file,
            hash_files: false,
            js_minify: true,
            split: false,
            server_fn_prefix: None,
            disable_server_fn_hash: false,
            disable_erase_components: false,
            always_erase_components: false,
            server_fn_mod_path: false,
            wasm_opt_features: None,
            build_frontend_only: false,
            build_server_only: false,
            clear_terminal_on_rebuild: false,
        };

        Ok(cargo_leptos::config::Config {
            working_dir: metadata.workspace_root,
            projects: vec![Arc::new(project)],
            cli: opts,
            watch,
        })
    }
}
