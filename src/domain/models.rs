use serde::{Deserialize, Serialize};

/// Complete telemetry structure for Linux systems.
#[derive(Debug, Serialize, Deserialize)]
pub struct MachineReport {
    /// System metadata information.
    pub metadata: Metadata,
    /// CPU statistics.
    pub cpu: CpuStats,
    /// Memory statistics.
    pub memory: MemoryStats,
    /// Process statistics.
    pub processes: ProcessStats,
    /// Network statistics.
    pub network: NetworkStats,
    /// Storage statistics.
    pub storage: StorageStats,
    /// Physical sensors data.
    pub sensors: PhysicalSensors,
    /// Security statistics.
    pub security: SecurityStats,
    /// System health information.
    pub health: SystemHealth,
    /// Unix Epoch in milliseconds. Example: 1705000000000
    pub timestamp: i64,
}

/// Numerical census and details of main system processes.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessStats {
    /// Total number of processes in the Kernel table. Example: 285
    pub total_count: u32,
    /// Processes currently running. Example: 2
    pub running_count: u32,
    /// Processes waiting (I/O or Sleep). Example: 282
    pub sleeping_count: u32,
    /// Dead processes still occupying the table. Example: 1
    pub zombie_count: u32,
    /// List of top 5 processes consuming the most CPU currently.
    pub top_cpu: Vec<ProcessInfo>,
    /// List of top 5 processes consuming the most memory currently.
    pub top_memory: Vec<ProcessInfo>,
}

/// Summary information of a specific process for diagnosis.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessInfo {
    /// Process identifier (PID). Example: 1234
    pub pid: u32,
    /// Executable name. Example: "rust_service"
    pub name: String,
    /// User who started the process. Example: "root" or "user"
    pub user: String,
    /// CPU usage percentage. Example: 15.5
    pub cpu_usage: f32,
    /// RAM memory usage in Megabytes. Example: 128.5
    pub mem_usage_mb: f32,
}

/// System metadata.
#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    /// Unique machine identifier.
    pub machine_id: String,
    /// Hostname of the system.
    pub hostname: String,
    /// Operating system distribution.
    pub os_distro: String,
    /// System uptime in seconds.
    pub uptime: u64,
    /// Virtualization type (if any).
    pub virtualization: String,
    /// Timestamp of when the system booted. Example: 1704913600
    pub boot_time: u64,
    /// Configured time zone. Example: "America/Sao_Paulo"
    pub timezone: String,
}

/// CPU statistics.
#[derive(Debug, Serialize, Deserialize)]
pub struct CpuStats {
    /// Total CPU usage percentage.
    pub usage_total_pct: f32,
    /// Load average over 1, 5, and 15 minutes.
    pub load_avg: [f64; 3],
    /// Usage percentage per thread/core.
    pub threads_usage: Vec<f32>,
    /// Interrupts per second.
    pub interrupts_sec: u64,
    /// Disk I/O wait time percentage. Example: 2.5
    pub io_wait_time: f32,
    /// CPU idle time.
    pub idle_time: u64,
    /// Current frequency of each core in MHz. Example: [3200, 2400, 3200]
    pub threads_freq_mhz: Vec<u32>,
    /// Current CPU core voltage. Example: 1.15
    pub voltage_vcore: f32,
}

/// Memory statistics.
#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryStats {
    /// Total memory in bytes.
    pub total_bytes: u64,
    /// Used memory in bytes.
    pub used_bytes: u64,
    /// Available memory in bytes.
    pub available_bytes: u64,
    /// Memory used by buffers and cache in bytes.
    pub buffers_cache_bytes: u64,
    /// Total swap memory in bytes.
    pub swap_total_bytes: u64,
    /// Used swap memory in bytes.
    pub swap_used_bytes: u64,
}

/// Network statistics.
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkStats {
    /// Aggregate received bytes per second.
    pub aggregate_rx_bytes_sec: u64,
    /// Aggregate transmitted bytes per second.
    pub aggregate_tx_bytes_sec: u64,
    /// Total received packets. Example: 15000
    pub aggregate_rx_packets: u64,
    /// Total transmitted packets. Example: 12000
    pub aggregate_tx_packets: u64,
    /// Total network errors.
    pub total_errors: u64,
    /// Total dropped packets.
    pub total_drops: u64,
    /// List of physical interfaces and their IPs. Example: {"eth0": ["192.168.1.10"]}
    pub interface_ips: std::collections::HashMap<String, Vec<String>>,
    /// Active TCP connections count.
    pub tcp_active_connections: u32,
    /// TCP connections in TIME_WAIT state.
    pub tcp_time_wait_connections: u32,
    /// TCP ports currently listening. Example: [22, 80, 443]
    pub listening_ports: Vec<u16>,
}

/// Storage statistics.
#[derive(Debug, Serialize, Deserialize)]
pub struct StorageStats {
    /// List of partition information.
    pub partitions: Vec<PartitionInfo>,
    /// Total bytes read per second.
    pub total_read_bytes_sec: u64,
    /// Total bytes written per second.
    pub total_write_bytes_sec: u64,
    /// Total read I/O operations per second.
    pub total_read_iops: u64,
    /// Total write I/O operations per second.
    pub total_write_iops: u64,
    /// Average I/O latency in ms (Queue Depth). Example: 0.8
    pub io_latency_ms: f32,
}

/// Information about a disk partition.
#[derive(Debug, Serialize, Deserialize)]
pub struct PartitionInfo {
    /// Mount point path.
    pub mount_point: String,
    /// Usage percentage.
    pub usage_pct: f32,
    /// Free space in bytes.
    pub free_bytes: u64,
    /// Total space in bytes.
    pub total_bytes: u64,
}

/// Physical sensors (available on Bare Metal/Dedicated).
#[derive(Debug, Serialize, Deserialize)]
pub struct PhysicalSensors {
    /// Global CPU temperature in °C. Example: 55.0
    pub cpu_temp: f32,
    /// Individual core temperatures. Example: [52.0, 54.0, 55.0]
    pub core_temps: Vec<f32>,
    /// SSD/NVMe storage temperatures. Example: [38.0]
    pub storage_temps: Vec<f32>,
    /// Fan speeds in RPM. Example: [2200, 2400]
    pub fan_speeds: Vec<u32>,
}

/// Security statistics.
#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityStats {
    /// Date/Time of the last SSH or local login. Example: "2024-05-20 14:00"
    pub last_login: String,
    /// Indicates if the Firewall (UFW/IPTables) is active. Example: true
    pub firewall_active: bool,
}

/// System health metrics.
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemHealth {
    /// Available system entropy.
    pub entropy_avail: u32,
}
