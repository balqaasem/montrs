use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use anyhow::Result;
use chrono::{DateTime, Utc};

pub mod guides;
pub mod error_parser;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AgentSnapshot {
    pub project_name: String,
    pub timestamp: DateTime<Utc>,
    pub framework_version: String,
    pub structure: Vec<FileEntry>,
    pub plates: Vec<PlateSummary>,
    pub routes: Vec<RouteSummary>,
    pub documentation_snippets: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileEntry {
    pub path: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlateSummary {
    pub name: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RouteSummary {
    pub path: String,
    pub kind: String,
    pub description: String,
    pub input_schema: Option<serde_json::Value>,
    pub output_schema: Option<serde_json::Value>,
    pub params_schema: Option<serde_json::Value>,
    pub loader_output_schema: Option<serde_json::Value>,
    pub action_input_schema: Option<serde_json::Value>,
    pub action_output_schema: Option<serde_json::Value>,
    pub metadata: HashMap<String, String>,
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
    pub package: Option<String>,
    pub file: String,
    pub line: u32,
    pub column: u32,
    pub message: String,
    pub code_context: String,
    pub level: String, // Error, Warning
    pub agent_metadata: Option<AgentErrorMetadata>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorTracking {
    pub errors: Vec<ConsolidatedError>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConsolidatedError {
    pub id: String,
    pub package: Option<String>,
    pub file: String,
    pub line: u32,
    pub column: u32,
    pub level: String,
    pub message: String,
    pub status: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AgentErrorMetadata {
    pub error_code: String,
    pub explanation: String,
    pub suggested_fixes: Vec<String>,
    pub rustc_error: Option<String>,
}

pub struct AgentManager {
    root_path: PathBuf,
}

impl AgentManager {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root_path: root.into() }
    }

    pub fn agent_dir(&self) -> PathBuf {
        self.root_path.join(".agent")
    }

    pub fn errorfiles_dir(&self) -> PathBuf {
        self.agent_dir().join("errorfiles")
    }

    pub fn tracking_file(&self) -> PathBuf {
        self.agent_dir().join("error_tracking.json")
    }

    pub fn ensure_dir(&self) -> Result<()> {
        let dir = self.agent_dir();
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        let error_dir = self.errorfiles_dir();
        if !error_dir.exists() {
            fs::create_dir_all(&error_dir)?;
        }
        Ok(())
    }

    pub fn write_snapshot(&self, snapshot: &AgentSnapshot, format: &str) -> Result<()> {
        self.ensure_dir()?;
        let content = match format {
            "yaml" => serde_yaml::to_string(snapshot)?,
            "txt" => format!("{:#?}", snapshot),
            _ => serde_json::to_string_pretty(snapshot)?,
        };
        let ext = if format == "yaml" { "yaml" } else if format == "txt" { "txt" } else { "json" };
        fs::write(self.agent_dir().join(format!("agent.{}", ext)), content)?;
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
        
        // Update consolidated tracking
        self.update_consolidated_tracking(record)?;
        
        Ok(())
    }

    pub fn load_tracking(&self) -> Result<ErrorTracking> {
        let path = self.tracking_file();
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            // Try new format first
            if let Ok(tracking) = serde_json::from_str::<ErrorTracking>(&content) {
                return Ok(tracking);
            }
            // Fallback to manual parsing if needed or try to adapt legacy
            if let Ok(value) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(active_issues) = value.get("active_issues").and_then(|v| v.as_array()) {
                    let mut errors = Vec::new();
                    for issue in active_issues {
                        if let (Some(id), Some(msg)) = (issue.get("id").and_then(|v| v.as_str()), issue.get("message").and_then(|v| v.as_str())) {
                            let status = issue.get("status").and_then(|v| v.as_str()).unwrap_or("Pending").to_string();
                            errors.push(ConsolidatedError {
                                id: id.to_string(),
                                package: issue.get("package").and_then(|v| v.as_str()).map(|s| s.to_string()),
                                file: issue.get("file").and_then(|v| v.as_str()).unwrap_or("unknown").to_string(),
                                line: issue.get("line").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
                                column: issue.get("column").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
                                level: issue.get("type").and_then(|v| v.as_str()).unwrap_or("Error").to_string(),
                                message: msg.to_string(),
                                status,
                                timestamp: issue.get("timestamp")
                                    .and_then(|v| v.as_str())
                                    .and_then(|s| s.parse().ok())
                                    .unwrap_or_else(Utc::now),
                            });
                        }
                    }
                    if !errors.is_empty() {
                        return Ok(ErrorTracking { errors });
                    }
                }
            }
        }
        Ok(ErrorTracking { errors: Vec::new() })
    }

    pub fn save_tracking(&self, tracking: &ErrorTracking) -> Result<()> {
        self.ensure_dir()?;
        let content = serde_json::to_string_pretty(tracking)?;
        fs::write(self.tracking_file(), content)?;
        Ok(())
    }

    fn update_consolidated_tracking(&self, record: &ErrorRecord) -> Result<()> {
        let mut tracking = self.load_tracking().unwrap_or(ErrorTracking { errors: Vec::new() });
        let status = match record.status {
            ErrorStatus::Active => "Pending",
            ErrorStatus::Resolved => "Fixed",
        }.to_string();

        let consolidated = ConsolidatedError {
            id: record.id.clone(),
            package: record.detail.package.clone(),
            file: record.detail.file.clone(),
            line: record.detail.line,
            column: record.detail.column,
            level: record.detail.level.clone(),
            message: record.detail.message.clone(),
            status,
            timestamp: record.timestamp,
        };

        if let Some(pos) = tracking.errors.iter().position(|e| e.id == record.id) {
            tracking.errors[pos] = consolidated;
        } else {
            tracking.errors.push(consolidated);
        }

        self.save_tracking(&tracking)
    }

    /// Reports a new error to the agent, creating or updating errorfile.json.
    pub fn report_project_error(&self, mut error: ProjectError) -> Result<String> {
        // Try to determine package from file path if not provided
        if error.package.is_none() && error.file != "unknown" {
            if let Some(pkg) = self.determine_package(&error.file) {
                error.package = Some(pkg);
            }
        }

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
            package: None,
            file: "unknown".to_string(),
            line: 0,
            column: 0,
            message,
            code_context: "".to_string(),
            level: "Error".to_string(),
            agent_metadata: None,
        })?;
        Ok(())
    }

    fn determine_package(&self, file_path: &str) -> Option<String> {
        let path = std::path::Path::new(file_path);
        // Look for "packages/NAME" or "apps/NAME"
        let components: Vec<_> = path.components().collect();
        for i in 0..components.len() {
            if let Some(c) = components[i].as_os_str().to_str() {
                if (c == "packages" || c == "apps") && i + 1 < components.len() {
                    return components[i+1].as_os_str().to_str().map(|s| s.to_string());
                }
            }
        }
        None
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
        println!("Agent: Generating tools spec...");
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
            }),
            serde_json::json!({
                "name": "montrs_sketch",
                "description": "Generates a single-file 'sketch' of a MontRS component.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "name": { "type": "string", "description": "Name of the sketch file" },
                        "kind": { "type": "string", "enum": ["plate", "route", "app"], "description": "Component type" }
                    },
                    "required": ["name"]
                }
            }),
            serde_json::json!({
                "name": "montrs_expand",
                "description": "Expands a sketch file into a full MontRS workspace structure.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Path to the sketch file" }
                    },
                    "required": ["path"]
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
                                if content.contains("Agent Usage Guide") || content.contains("Key Features") || content.contains("Key Components") || content.contains("Agent") {
                                    tools.push(serde_json::json!({
                                        "name": format!("montrs_pkg_{}", pkg_name),
                                        "description": format!("Capability provided by package {}. Refer to its README for details.", pkg_name),
                                        "parameters": { "type": "object", "properties": {} }
                                    }));
                                }
                            }
                        }

                        // 2. Scan source for explicit @agent-tool markers
                        let src_dir = path.join("src");
                        if src_dir.exists() {
                            let walker = walkdir::WalkDir::new(&src_dir).into_iter();
                            for entry in walker.filter_map(|e| e.ok()) {
                                if entry.path().is_file() && entry.path().extension().and_then(|s| s.to_str()) == Some("rs") {
                                    if let Ok(content) = fs::read_to_string(entry.path()) {
                                        for line in content.lines() {
                                            if line.contains("@agent-tool:") {
                                                // Simple extraction: // @agent-tool: name="tool_name" desc="description"
                                                if let Some(tool_meta) = self.parse_agent_tool_marker(line) {
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
                                if line.contains("@agent-tool:") {
                                    if let Some(tool_meta) = self.parse_agent_tool_marker(line) {
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

    fn parse_agent_tool_marker(&self, line: &str) -> Option<serde_json::Value> {
        // Expected format: @agent-tool: name="name" desc="description"
        let re = regex::Regex::new(r#"@agent-tool:\s+name="(?P<name>[^"]+)"\s+desc="(?P<desc>[^"]+)""#).ok()?;
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

    fn discover_plates_heuristically(&self) -> (Vec<PlateSummary>, Vec<RouteSummary>) {
        let mut plates = Vec::new();
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
                                println!("Agent: Scanning for plates in {:?}", src);
                                scan_dirs.push(src);
                            }
                            let tests = entry.path().join("tests");
                            if tests.exists() {
                                println!("Agent: Scanning for plates in {:?}", tests);
                                scan_dirs.push(tests);
                            }
                        }
                    }
                }
            }
        }

        let plate_re = regex::Regex::new(r"impl\s+Plate(?:<[^>]+>)?\s+for\s+(\w+)").unwrap();
        let route_re = regex::Regex::new(r"impl\s+Route(?:<[^>]+>)?\s+for\s+(\w+)").unwrap();

        for src_dir in scan_dirs {
            if !src_dir.exists() { continue; }
            let walker = walkdir::WalkDir::new(&src_dir).into_iter();
            for entry in walker.filter_map(|e| e.ok()) {
                if entry.path().is_file() && entry.path().extension().and_then(|s| s.to_str()) == Some("rs") {
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        println!("Agent: Checking file {:?}", entry.path());
                        // Discover Plates
                        for caps in plate_re.captures_iter(&content) {
                            let name = caps[1].to_string();
                            println!("Agent: Found plate implementation: {}", name);
                            if !plates.iter().any(|m: &PlateSummary| m.name == name) {
                                plates.push(PlateSummary {
                                    name,
                                    description: self.get_file_description(entry.path()).unwrap_or_else(|| "Discovered plate".to_string()),
                                    dependencies: Vec::new(),
                                    metadata: HashMap::new(),
                                });
                            }
                        }

                        // Discover Routes
                        for caps in route_re.captures_iter(&content) {
                            let name = caps[1].to_string();
                            println!("Agent: Found route implementation: {}", name);
                            routes.push(RouteSummary {
                                path: format!("(impl) {}", name),
                                kind: "Route".to_string(),
                                description: format!("Heuristically discovered Route: {}", name),
                                input_schema: None,
                                output_schema: None,
                                params_schema: None,
                                loader_output_schema: None,
                                action_input_schema: None,
                                action_output_schema: None,
                                metadata: HashMap::new(),
                            });
                        }
                    }
                }
            }
        }

        (plates, routes)
    }

    pub fn write_tools_spec(&self) -> Result<()> {
        let tools = self.generate_tools_spec()?;
        let content = serde_json::to_string_pretty(&tools)?;
        let path = self.agent_dir().join("tools.json");
        fs::write(path, content)?;
        Ok(())
    }

    /// Generates a comprehensive snapshot of the codebase.
    pub fn generate_snapshot(&self, project_name: &str) -> Result<AgentSnapshot> {
        self.generate_snapshot_with_spec(project_name, None)
    }

    pub fn generate_snapshot_with_spec(&self, project_name: &str, spec: Option<montrs_core::AppSpecExport>) -> Result<AgentSnapshot> {
        let mut structure = Vec::new();
        let walker = ignore::WalkBuilder::new(&self.root_path)
            .hidden(false)
            .git_ignore(true)
            .filter_entry(|entry| {
                let name = entry.file_name().to_string_lossy();
                name != ".git" && name != "target" && name != ".agent"
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

        let (plates, routes) = if let Some(s) = spec {
            let mut plates = Vec::new();
            let mut routes = Vec::new();

            for plate_spec in s.plates {
                plates.push(PlateSummary {
                    name: plate_spec.name,
                    description: plate_spec.description,
                    dependencies: plate_spec.dependencies,
                    metadata: plate_spec.metadata,
                });
            }
            
            for (path, meta) in s.router.routes {
                routes.push(RouteSummary {
                    path: path.clone(),
                    kind: "Route".to_string(),
                    description: meta.loader_description.clone(),
                    input_schema: None,
                    output_schema: None,
                    params_schema: None,
                    loader_output_schema: None,
                    action_input_schema: None,
                    action_output_schema: None,
                    metadata: HashMap::new(),
                });
            }
            (plates, routes)
        } else {
            self.discover_plates_heuristically()
        };

        Ok(AgentSnapshot {
            project_name: project_name.to_string(),
            timestamp: Utc::now(),
            framework_version: "0.1.0".to_string(),
            structure,
            plates,
            routes,
            documentation_snippets,
        })
    }

    pub fn check_invariants(&self, snapshot: &AgentSnapshot) -> Result<Vec<String>> {
        let mut violations = Vec::new();

        // 1. Check Plate Dependencies
        // For each plate, all its dependencies must exist in the snapshot
        for plate in &snapshot.plates {
            for dep in &plate.dependencies {
                if !snapshot.plates.iter().any(|p| &p.name == dep) {
                    violations.push(format!(
                        "Plate '{}' depends on missing plate '{}'.",
                        plate.name, dep
                    ));
                }
            }
        }

        // 2. Check for circular dependencies (simple depth-limited check)
        for plate in &snapshot.plates {
            let mut visited = std::collections::HashSet::new();
            let mut stack = vec![(&plate.name, 0)];
            
            while let Some((current_name, depth)) = stack.pop() {
                if depth > 10 { // Limit depth to avoid infinite loops in complex cycles
                    violations.push(format!("Potential deep dependency cycle involving plate '{}'.", plate.name));
                    break;
                }
                
                if visited.contains(current_name) {
                    violations.push(format!("Circular dependency detected involving plate '{}'.", current_name));
                    break;
                }
                visited.insert(current_name);
                
                if let Some(p) = snapshot.plates.iter().find(|p| &p.name == current_name) {
                    for dep in &p.dependencies {
                        stack.push((dep, depth + 1));
                    }
                }
            }
        }

        Ok(violations)
    }
}
