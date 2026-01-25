use sysinfo::{System, CpuRefreshKind, MemoryRefreshKind, RefreshKind};
use serde::{Deserialize, Serialize};

/// Captures system information for the benchmark report.
///
/// This provides context for benchmark results, allowing comparison across different environments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    /// Operating System name (e.g., "Windows", "Linux").
    pub os_name: String,
    /// Operating System version.
    pub os_version: String,
    /// Kernel version.
    pub kernel_version: String,
    /// Hostname of the machine.
    pub host_name: String,
    /// CPU brand/model string.
    pub cpu_brand: String,
    /// Number of physical CPU cores.
    pub cpu_cores: usize,
    /// Total system memory in Gigabytes.
    pub total_memory_gb: f64,
    /// Version of the Rust compiler used to build the benchmark.
    pub rust_version: String,
}

impl SystemInfo {
    /// Collects system information from the current environment.
    pub fn collect() -> Self {
        let mut sys = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        );
        sys.refresh_all();

        Self {
            os_name: System::name().unwrap_or_else(|| "Unknown".to_string()),
            os_version: System::os_version().unwrap_or_else(|| "Unknown".to_string()),
            kernel_version: System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
            host_name: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
            cpu_brand: sys.cpus().first().map(|c| c.brand().to_string()).unwrap_or_default(),
            cpu_cores: sys.physical_core_count().unwrap_or(sys.cpus().len()),
            total_memory_gb: sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0,
            rust_version: rustc_version_runtime::version().to_string(),
        }
    }
}
