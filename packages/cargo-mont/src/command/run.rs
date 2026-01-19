use crate::config::{MontConfig, TaskConfig};
use crate::ext::exe_command;
use console::style;
use std::collections::{HashMap, HashSet};
use std::process::Command;

pub async fn run(task_name: String) -> anyhow::Result<()> {
    let config = MontConfig::load()?;

    // Resolve dependencies and run them in order
    let mut executed = HashSet::new();
    execute_task_recursive(&task_name, &config, &mut executed).await?;

    Ok(())
}

async fn execute_task_recursive(
    name: &str,
    config: &MontConfig,
    executed: &mut HashSet<String>,
) -> anyhow::Result<()> {
    if executed.contains(name) {
        return Ok(());
    }

    let task = config
        .tasks
        .get(name)
        .ok_or_else(|| anyhow::anyhow!("Task '{}' not found in mont.toml", name))?;

    // 1. Run dependencies first
    if let TaskConfig::Detailed { dependencies, .. } = task {
        for dep in dependencies {
            Box::pin(execute_task_recursive(dep, config, executed)).await?;
        }
    }

    // 2. Run the task itself
    println!(
        "{} Running task: {}",
        style("🛠").bold(),
        style(name).cyan().bold()
    );

    match task {
        TaskConfig::Simple(cmd_str) => {
            run_shell_cmd(cmd_str, &HashMap::new())?;
        }
        TaskConfig::Detailed {
            command,
            env,
            description,
            ..
        } => {
            if let Some(desc) = description {
                println!("   {}", style(desc).italic().dim());
            }
            run_shell_cmd(command, env)?;
        }
    }

    executed.insert(name.to_string());
    Ok(())
}

fn run_shell_cmd(cmd_str: &str, env_vars: &HashMap<String, String>) -> anyhow::Result<()> {
    #[cfg(windows)]
    let mut cmd = Command::new("powershell");
    #[cfg(windows)]
    cmd.arg("-Command");

    #[cfg(not(windows))]
    let mut cmd = Command::new("sh");
    #[cfg(not(windows))]
    cmd.arg("-c");

    cmd.arg(cmd_str);

    for (key, val) in env_vars {
        cmd.env(key, val);
    }

    exe_command(&mut cmd)?;
    Ok(())
}

pub async fn list() -> anyhow::Result<()> {
    let config = MontConfig::load()?;

    if config.tasks.is_empty() {
        println!("No tasks defined in mont.toml");
        return Ok(());
    }

    println!("{}", style("Available Tasks:").bold());

    // Group by category if possible
    let mut categories: HashMap<String, Vec<(&String, &TaskConfig)>> = HashMap::new();
    for (name, task) in &config.tasks {
        let cat = match task {
            TaskConfig::Detailed { category, .. } => category.as_deref().unwrap_or("General"),
            TaskConfig::Simple(_) => "General",
        };
        categories
            .entry(cat.to_string())
            .or_default()
            .push((name, task));
    }

    for (cat, tasks) in categories {
        println!("\n[{}]", style(cat).yellow());
        for (name, task) in tasks {
            match task {
                TaskConfig::Simple(_) => {
                    println!("  - {}", style(name).cyan());
                }
                TaskConfig::Detailed { description, .. } => {
                    let desc = description.as_deref().unwrap_or("No description");
                    println!("  - {}: {}", style(name).cyan(), desc);
                }
            }
        }
    }
    Ok(())
}
