use montrs_llm::{LlmManager, LlmSnapshot, ErrorFile};
use tempfile::tempdir;
use std::fs;

#[tokio::test]
async fn test_llm_generation() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    
    // Create a dummy file
    fs::write(root.join("test.rs"), "fn main() {}").unwrap();
    
    let manager = LlmManager::new(root);
    let snapshot = manager.generate_snapshot("test-project".to_string()).unwrap();
    
    assert_eq!(snapshot.project_name, "test-project");
    assert!(snapshot.structure.iter().any(|f| f.path == "test.rs"));
    assert!(snapshot.documentation_snippets.contains_key("architecture"));
    
    manager.write_snapshot(&snapshot, "json").unwrap();
    assert!(root.join(".llm/llm.json").exists());
}

#[tokio::test]
async fn test_error_reporting() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    
    let manager = LlmManager::new(root);
    manager.report_error("Something went wrong".to_string()).unwrap();
    
    assert!(root.join(".llm/errorfile.txt").exists());
    let content = fs::read_to_string(root.join(".llm/errorfile.txt")).unwrap();
    assert!(content.contains("Something went wrong"));
}
