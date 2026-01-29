use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use anyhow::Result;
use chrono::{DateTime, Utc};

pub mod guides;
pub mod error_parser;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LlmSnapshot {
    pub project_name: String,
    pub timestamp: DateTime<Utc>,
    pub framework_version: String,
    pub structure: Vec<FileEntry>,
    pub modules: Vec<ModuleSummary>,
    pub routes: Vec<RouteSummary>,
    pub documentation_snippets: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileEntry {
    pub path: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModuleSummary {
    pub name: String,
    pub description: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RouteSummary {
    pub path: String,
    pub kind: String, // "Loader" or "Action"
    pub description: String,
    pub input_schema: Option<serde_json::Value>,
    pub output_schema: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorRecord {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub version: u32,
    pub status: ErrorStatus,
    pub detail: ProjectError,
    pub history: Vec<ErrorVersion>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ErrorStatus {
    Active,
    Resolved,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorVersion {
    pub version: u32,
    pub timestamp: DateTime<Utc>,
    pub message: String,
    pub diff: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectError {
    pub file: String,
    pub line: u32,
    pub column: u32,
    pub message: String,
    pub code_context: String,
    pub level: String, // Error, Warning
    pub ai_metadata: Option<AiErrorMetadata>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AiErrorMetadata {
    pub error_code: String,
    pub explanation: String,
    pub suggested_fixes: Vec<String>,
    pub rustc_error: Option<String>,
}

pub struct LlmManager {
    root_path: PathBuf,
}

impl LlmManager {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root_path: root.into() }
    }

    pub fn llm_dir(&self) -> PathBuf {
        self.root_path.join(".llm")
    }

    pub fn errorfiles_dir(&self) -> PathBuf {
        self.llm_dir().join("errorfiles")
    }

    pub fn ensure_dir(&self) -> Result<()> {
        let dir = self.llm_dir();
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        let error_dir = self.errorfiles_dir();
        if !error_dir.exists() {
            fs::create_dir_all(&error_dir)?;
        }
        Ok(())
    }

    pub fn write_snapshot(&self, snapshot: &LlmSnapshot, format: &str) -> Result<()> {
        self.ensure_dir()?;
        let content = match format {
            "yaml" => serde_yaml::to_string(snapshot)?,
            "txt" => format!("{:#?}", snapshot),
            _ => serde_json::to_string_pretty(snapshot)?,
        };
        let ext = if format == "yaml" { "yaml" } else if format == "txt" { "txt" } else { "json" };
        fs::write(self.llm_dir().join(format!("llm.{}", ext)), content)?;
        Ok(())
    }

    pub fn write_error_record(&self, record: &ErrorRecord) -> Result<()> {
        self.ensure_dir()?;
        let version_dir = self.errorfiles_dir().join(format!("v{}", record.version));
        if !version_dir.exists() {
            fs::create_dir_all(&version_dir)?;
        }
        
        let content = serde_json::to_string_pretty(record)?;
        fs::write(version_dir.join(format!("{}.json", record.id)), content)?;
        Ok(())
    }

    pub fn report_project_error(&self, error: ProjectError) -> Result<String> {
        // Check if a similar active error already exists
        if let Ok(active_errors) = self.list_active_errors() {
            for existing in active_errors {
                if existing.detail.file == error.file && 
                   existing.detail.line == error.line && 
                   existing.detail.message == error.message {
                    return Ok(existing.id);
                }
            }
        }

        let id = uuid::Uuid::new_v4().to_string();
        let record = ErrorRecord {
            id: id.clone(),
            timestamp: Utc::now(),
            version: 1,
            status: ErrorStatus::Active,
            detail: error,
            history: Vec::new(),
        };
        self.write_error_record(&record)?;
        Ok(id)
    }

    pub fn list_active_errors(&self) -> Result<Vec<ErrorRecord>> {
        let mut active = Vec::new();
        let error_dir = self.errorfiles_dir();
        if !error_dir.exists() {
            return Ok(active);
        }

        for entry in fs::read_dir(error_dir)?.flatten() {
            if entry.path().is_dir() {
                for file in fs::read_dir(entry.path())?.flatten() {
                    if file.path().extension().and_then(|s| s.to_str()) == Some("json") {
                        if let Ok(content) = fs::read_to_string(file.path()) {
                            if let Ok(record) = serde_json::from_str::<ErrorRecord>(&content) {
                                if let ErrorStatus::Active = record.status {
                                    active.push(record);
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(active)
    }

    pub fn report_error(&self, message: String) -> Result<()> {
        self.report_project_error(ProjectError {
            file: "unknown".to_string(),
            line: 0,
            column: 0,
            message,
            code_context: "".to_string(),
            level: "Error".to_string(),
            ai_metadata: None,
        })?;
        Ok(())
    }

    pub fn generate_diff(&self) -> Option<String> {
        // Try to use git diff if available
        std::process::Command::new("git")
            .args(["diff", "HEAD"])
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    let s = String::from_utf8_lossy(&output.stdout).to_string();
                    if s.is_empty() { None } else { Some(s) }
                } else {
                    None
                }
            })
    }

    pub fn auto_resolve_active_errors(&self, fix_message: String, diff: Option<String>) -> Result<()> {
        let walker = walkdir::WalkDir::new(self.errorfiles_dir()).into_iter();
        for entry in walker.filter_map(|e| e.ok()) {
            if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                let content = fs::read_to_string(entry.path())?;
                if let Ok(record) = serde_json::from_str::<ErrorRecord>(&content) {
                    if let ErrorStatus::Active = record.status {
                        self.resolve_error(&record.id, fix_message.clone(), diff.clone())?;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn resolve_error(&self, id: &str, fix_message: String, diff: Option<String>) -> Result<()> {
        let mut record = self.find_error(id)?;
        if let ErrorStatus::Active = record.status {
            record.status = ErrorStatus::Resolved;
            let history_version = record.version;
            record.version += 1;
            record.history.push(ErrorVersion {
                version: history_version,
                timestamp: Utc::now(),
                message: fix_message,
                diff,
            });
            self.write_error_record(&record)?;
        }
        Ok(())
    }

    fn find_error(&self, id: &str) -> Result<ErrorRecord> {
        let error_dir = self.errorfiles_dir();
        for entry in fs::read_dir(error_dir)?.flatten() {
            if entry.path().is_dir() {
                let file_path = entry.path().join(format!("{}.json", id));
                if file_path.exists() {
                    let content = fs::read_to_string(file_path)?;
                    return Ok(serde_json::from_str(&content)?);
                }
            }
        }
        anyhow::bail!("Error record not found: {}", id)
    }

    pub fn generate_tools_spec(&self) -> Result<serde_json::Value> {
        println!("AI-First: Generating tools spec...");
        let mut tools = vec![
            serde_json::json!({
                "name": "montrs_build",
                "description": "Builds the MontRS project.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "release": { "type": "boolean", "description": "Build in release mode" }
                    }
                }
            }),
            serde_json::json!({
                "name": "montrs_spec",
                "description": "Generates a project specification.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "format": { "type": "string", "enum": ["json", "yaml", "txt"] }
                    }
                }
            }),
            serde_json::json!({
                "name": "montrs_fmt",
                "description": "Formats the project's Rust and view! code.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "check": { "type": "boolean", "description": "Check if files are formatted without modifying them" },
                        "path": { "type": "string", "description": "Path to format (default: .)" }
                    }
                }
            })
        ];

        // Scan for package-specific tools/capabilities from READMEs and source comments
        let packages_dir = self.root_path.join("packages");
        if packages_dir.exists() {
            if let Ok(entries) = fs::read_dir(&packages_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        let pkg_name = path.file_name().unwrap().to_string_lossy();
                        
                        // 1. Scan README for high-level capabilities
                        let readme_path = path.join("README.md");
                        if readme_path.exists() {
                            if let Ok(content) = fs::read_to_string(&readme_path) {
                                if content.contains("AI Usage Guide") || content.contains("Key Features") || content.contains("Key Components") || content.contains("AI-First") {
                                    tools.push(serde_json::json!({
                                        "name": format!("montrs_pkg_{}", pkg_name),
                                        "description": format!("Capability provided by package {}. Refer to its README for details.", pkg_name),
                                        "parameters": { "type": "object", "properties": {} }
                                    }));
                                }
                            }
                        }

                        // 2. Scan source for explicit @ai-tool markers
                        let src_dir = path.join("src");
                        if src_dir.exists() {
                            let walker = walkdir::WalkDir::new(&src_dir).into_iter();
                            for entry in walker.filter_map(|e| e.ok()) {
                                if entry.path().is_file() && entry.path().extension().and_then(|s| s.to_str()) == Some("rs") {
                                    if let Ok(content) = fs::read_to_string(entry.path()) {
                                        for line in content.lines() {
                                            if line.contains("@ai-tool") {
                                                // Simple extraction: // @ai-tool: name="tool_name" desc="description"
                                                if let Some(tool_meta) = self.parse_ai_tool_marker(line) {
                                                    // Avoid duplicates
                                                    let name = tool_meta["name"].as_str().unwrap_or_default();
                                                    if !tools.iter().any(|t| t["name"] == name) {
                                                        tools.push(tool_meta);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            // If packages_dir doesn't exist, we might be in a package itself or a flat structure
            // Just scan current src if it exists
            let src_dir = self.root_path.join("src");
            if src_dir.exists() {
                let walker = walkdir::WalkDir::new(&src_dir).into_iter();
                for entry in walker.filter_map(|e| e.ok()) {
                    if entry.path().is_file() && entry.path().extension().and_then(|s| s.to_str()) == Some("rs") {
                        if let Ok(content) = fs::read_to_string(entry.path()) {
                            for line in content.lines() {
                                if line.contains("@ai-tool") {
                                    if let Some(tool_meta) = self.parse_ai_tool_marker(line) {
                                        let name = tool_meta["name"].as_str().unwrap_or_default();
                                        if !tools.iter().any(|t| t["name"] == name) {
                                            tools.push(tool_meta);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(serde_json::json!({ "tools": tools }))
    }

    fn parse_ai_tool_marker(&self, line: &str) -> Option<serde_json::Value> {
        // Expected format: @ai-tool: name="name" desc="description"
        let re = regex::Regex::new(r#"@ai-tool:\s+name="(?P<name>[^"]+)"\s+desc="(?P<desc>[^"]+)""#).ok()?;
        let caps = re.captures(line)?;
        
        Some(serde_json::json!({
            "name": caps.name("name")?.as_str(),
            "description": caps.name("desc")?.as_str(),
            "parameters": { "type": "object", "properties": {} }
        }))
    }

    fn get_file_description(&self, path: &std::path::Path) -> Option<String> {
        let content = fs::read_to_string(path).ok()?;
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("//!") || line.starts_with("///") {
                let desc = line.trim_start_matches("//!").trim_start_matches("///").trim();
                if !desc.is_empty() {
                    return Some(desc.to_string());
                }
            }
        }
        None
    }

    fn discover_modules_heuristically(&self) -> (Vec<ModuleSummary>, Vec<RouteSummary>) {
        let mut modules = Vec::new();
        let mut routes = Vec::new();

        // Scan root src, packages/*/src, and templates/*/src
        let mut scan_dirs = vec![self.root_path.join("src")];
        
        for dir_name in &["packages", "templates"] {
            let base_dir = self.root_path.join(dir_name);
            if base_dir.exists() {
                if let Ok(entries) = fs::read_dir(base_dir) {
                    for entry in entries.flatten() {
                        if entry.path().is_dir() {
                            let src = entry.path().join("src");
                            if src.exists() {
                                println!("AI-First: Scanning for modules in {:?}", src);
                                scan_dirs.push(src);
                            }
                            let tests = entry.path().join("tests");
                            if tests.exists() {
                                println!("AI-First: Scanning for modules in {:?}", tests);
                                scan_dirs.push(tests);
                            }
                        }
                    }
                }
            }
        }

        let module_re = regex::Regex::new(r"impl\s+Module(?:<[^>]+>)?\s+for\s+(\w+)").unwrap();
        let loader_re = regex::Regex::new(r"impl\s+Loader(?:<[^>]+>)?\s+for\s+(\w+)").unwrap();
        let action_re = regex::Regex::new(r"impl\s+Action(?:<[^>]+>)?\s+for\s+(\w+)").unwrap();

        for src_dir in scan_dirs {
            if !src_dir.exists() { continue; }
            let walker = walkdir::WalkDir::new(&src_dir).into_iter();
            for entry in walker.filter_map(|e| e.ok()) {
                if entry.path().is_file() && entry.path().extension().and_then(|s| s.to_str()) == Some("rs") {
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        println!("AI-First: Checking file {:?}", entry.path());
                        // Discover Modules
                        for caps in module_re.captures_iter(&content) {
                            let name = caps[1].to_string();
                            println!("AI-First: Found module implementation: {}", name);
                            if !modules.iter().any(|m: &ModuleSummary| m.name == name) {
                                modules.push(ModuleSummary {
                                    name,
                                    description: self.get_file_description(entry.path()).unwrap_or_else(|| "Discovered module".to_string()),
                                    metadata: HashMap::new(),
                                });
                            }
                        }

                        // Discover Loaders
                        for caps in loader_re.captures_iter(&content) {
                            let name = caps[1].to_string();
                            println!("AI-First: Found loader implementation: {}", name);
                            routes.push(RouteSummary {
                                path: format!("(impl) {}", name),
                                kind: "Loader".to_string(),
                                description: format!("Heuristically discovered Loader: {}", name),
                                input_schema: None,
                                output_schema: None,
                            });
                        }

                        // Discover Actions
                        for caps in action_re.captures_iter(&content) {
                            let name = caps[1].to_string();
                            println!("AI-First: Found action implementation: {}", name);
                            routes.push(RouteSummary {
                                path: format!("(impl) {}", name),
                                kind: "Action".to_string(),
                                description: format!("Heuristically discovered Action: {}", name),
                                input_schema: None,
                                output_schema: None,
                            });
                        }
                    }
                }
            }
        }

        (modules, routes)
    }

    pub fn write_tools_spec(&self) -> Result<()> {
        let tools = self.generate_tools_spec()?;
        let content = serde_json::to_string_pretty(&tools)?;
        let path = self.llm_dir().join("tools.json");
        fs::write(path, content)?;
        Ok(())
    }

    /// Generates a comprehensive snapshot of the codebase.
    pub fn generate_snapshot(&self, project_name: String) -> Result<LlmSnapshot> {
        self.generate_snapshot_with_spec(project_name, None)
    }

    pub fn generate_snapshot_with_spec(&self, project_name: String, spec: Option<montrs_core::AppSpecExport>) -> Result<LlmSnapshot> {
        let mut structure = Vec::new();
        let walker = ignore::WalkBuilder::new(&self.root_path)
            .hidden(false)
            .git_ignore(true)
            .filter_entry(|entry| {
                let name = entry.file_name().to_string_lossy();
                name != ".git" && name != "target" && name != ".llm"
            })
            .build();

        for entry in walker {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    let relative_path = path.strip_prefix(&self.root_path)?
                        .to_string_lossy()
                        .into_owned();
                    let description = if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                        self.get_file_description(path)
                    } else {
                        None
                    };

                    structure.push(FileEntry {
                        path: relative_path,
                        description,
                    });
                }
            }
        }

        let mut documentation_snippets = HashMap::new();
        documentation_snippets.insert("architecture".to_string(), guides::ARCHITECTURE_GUIDE.to_string());
        documentation_snippets.insert("debugging".to_string(), guides::DEBUGGING_GUIDE.to_string());

        let (modules, routes) = if let Some(s) = spec {
            let mut modules = Vec::new();
            let mut routes = Vec::new();

            for module_spec in s.modules {
                modules.push(ModuleSummary {
                    name: module_spec.name,
                    description: module_spec.description,
                    metadata: HashMap::new(),
                });
            }
            
            for (path, loader) in s.router.loaders {
                routes.push(RouteSummary {
                    path,
                    kind: "Loader".to_string(),
                    description: loader.description,
                    input_schema: loader.input_schema,
                    output_schema: loader.output_schema,
                });
            }
            for (path, action) in s.router.actions {
                routes.push(RouteSummary {
                    path,
                    kind: "Action".to_string(),
                    description: action.description,
                    input_schema: action.input_schema,
                    output_schema: action.output_schema,
                });
            }
            (modules, routes)
        } else {
            self.discover_modules_heuristically()
        };

        Ok(LlmSnapshot {
            project_name,
            timestamp: Utc::now(),
            framework_version: env!("CARGO_PKG_VERSION").to_string(),
            structure,
            modules,
            routes,
            documentation_snippets,
        })
    }
}
