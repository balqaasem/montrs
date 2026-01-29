use montrs_agent::{AgentManager, AgentSnapshot};
use tempfile::tempdir;
use std::fs;

#[tokio::test]
async fn test_agent_generation() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    
    // Create a dummy file
    fs::write(root.join("test.rs"), "fn main() {}").unwrap();
    
    let manager = AgentManager::new(root);
    let snapshot = manager.generate_snapshot("test-project".to_string()).unwrap();
    
    assert_eq!(snapshot.project_name, "test-project");
    assert!(snapshot.structure.iter().any(|f| f.path == "test.rs"));
    assert!(snapshot.documentation_snippets.contains_key("architecture"));
    
    manager.write_snapshot(&snapshot, "json").unwrap();
    assert!(root.join(".agent/agent.json").exists());
}

#[tokio::test]
async fn test_error_reporting() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    
    let manager = AgentManager::new(root);
    manager.report_error("Something went wrong".to_string()).unwrap();
    
    assert!(root.join(".agent/errorfiles/v1").exists());
    let error_dir = root.join(".agent/errorfiles/v1");
    let entries = fs::read_dir(error_dir).unwrap();
    let mut found = false;
    for entry in entries {
        let path = entry.unwrap().path();
        let content = fs::read_to_string(path).unwrap();
        if content.contains("Something went wrong") {
            found = true;
            break;
        }
    }
    assert!(found);
}
