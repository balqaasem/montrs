use montrs_bench::BenchConfig;
use std::collections::HashMap;
use std::time::Duration;

#[test]
fn test_config_priority_and_compat() {
    // 1. Default values
    let empty_env = HashMap::<String, String>::new();
    let config = BenchConfig::build_with_env(vec!["bench"], |key: &str| empty_env.get(key).cloned());
    assert_eq!(config.warmup_iterations, 10);
    assert_eq!(config.iterations, 100);
    assert_eq!(config.duration, Some(Duration::from_secs(5)));

    // 2. Old Env Vars (Compat)
    let mut old_env = HashMap::new();
    old_env.insert("MONT_BENCH_WARMUP".to_string(), "20".to_string());
    old_env.insert("MONT_BENCH_ITERATIONS".to_string(), "200".to_string());
    old_env.insert("MONT_BENCH_TIMEOUT".to_string(), "8".to_string());
    
    let config = BenchConfig::build_with_env(vec!["bench"], |key: &str| old_env.get(key).cloned());
    assert_eq!(config.warmup_iterations, 20);
    assert_eq!(config.iterations, 200);
    assert_eq!(config.duration, Some(Duration::from_secs(8)));

    // 3. New Env Vars (Priority over Old)
    let mut mixed_env = HashMap::new();
    mixed_env.insert("MONT_BENCH_WARMUP".to_string(), "20".to_string());
    mixed_env.insert("MONTRS_BENCH_WARMUP".to_string(), "30".to_string());
    
    let config = BenchConfig::build_with_env(vec!["bench"], |key: &str| mixed_env.get(key).cloned());
    assert_eq!(config.warmup_iterations, 30); // New wins

    // 4. CLI Args (Priority over Env)
    let mut env_with_val = HashMap::new();
    env_with_val.insert("MONTRS_BENCH_WARMUP".to_string(), "30".to_string());
    
    // We pass args explicitly
    let config = BenchConfig::build_with_env(vec!["bench", "--warmup", "40"], |key: &str| env_with_val.get(key).cloned());
    assert_eq!(config.warmup_iterations, 40); // Args win

    // 5. Duration parsing via CLI
    let config = BenchConfig::build_with_env(vec!["bench", "--timeout", "10"], |key: &str| empty_env.get(key).cloned());
    assert_eq!(config.duration, Some(Duration::from_secs(10)));
    
    // 6. Filter and JSON output
    let config = BenchConfig::build_with_env(
        vec!["bench", "--filter", "my_test", "--json-output", "report.json"],
        |key: &str| empty_env.get(key).cloned()
    );
    assert_eq!(config.filter, Some("my_test".to_string()));
    assert_eq!(config.json_output, Some("report.json".to_string()));
}
