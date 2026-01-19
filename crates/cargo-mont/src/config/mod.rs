use cargo_metadata::MetadataCommand;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct MontConfig {
    pub project: ProjectConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProjectConfig {
    pub name: String,
    pub bin_target: String,
    pub trunk_target: String,
}

impl MontConfig {
    pub fn load() -> anyhow::Result<Self> {
        let metadata = MetadataCommand::new()
            .exec()
            .map_err(|e| anyhow::anyhow!("Failed to load cargo metadata: {}", e))?;

        // Find the root package or the current workspace member
        // For now, we assume a simple structure or look for [package.metadata.mont]
        // This is a placeholder for actual complex detection logic

        let root_package = metadata.root_package().context("No root package found")?;

        // Try to parse from metadata
        let _metadata_value = &root_package.metadata;

        // We'll return a default for now if parsing fails, but use the package name
        Ok(Self {
            project: ProjectConfig {
                name: root_package.name.clone(),
                bin_target: root_package.name.clone(),
                trunk_target: "index.html".to_string(),
            },
        })
    }
}

use anyhow::Context;
