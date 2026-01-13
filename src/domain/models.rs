use serde::{Deserialize, Serialize};

/// Estrutura completa de telemetria para sistemas Linux.
#[derive(Debug, Serialize, Deserialize)]
pub struct MachineReport {
    pub metadata: Metadata,
    pub cpu: CpuStats,
    pub memory: MemoryStats,
    pub processes: ProcessStats,
    pub network: NetworkStats,
    pub storage: StorageStats,
    pub sensors: PhysicalSensors,
    pub security: SecurityStats,
    pub health: SystemHealth,
    /// Unix Epoch em milissegundos. Exemplo: 1705000000000
    pub timestamp: i64,
}

/// Censo numérico e detalhamento dos principais processos do sistema.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessStats {
    /// Quantidade total de processos na tabela do Kernel. Exemplo: 285
    pub total_count: u32,
    /// Processos em execução ativa. Exemplo: 2
    pub running_count: u32,
    /// Processos em espera (I/O ou Sleep). Exemplo: 282
    pub sleeping_count: u32,
    /// Processos mortos que ainda ocupam a tabela. Exemplo: 1
    pub zombie_count: u32,
    /// Lista dos 5 processos que mais consomem CPU no momento.
    pub top_cpu: Vec<ProcessInfo>,
    /// Lista dos 5 processos que mais consomem memória no momento.
    pub top_memory: Vec<ProcessInfo>,
}

/// Informações resumidas de um processo específico para diagnóstico.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessInfo {
    /// Identificador do processo. Exemplo: 1234
    pub pid: u32,
    /// Nome do executável. Exemplo: "rust_service"
    pub name: String,
    /// Usuário que iniciou o processo. Exemplo: "root" ou "kaio"
    pub user: String,
    /// Porcentagem de uso de CPU. Exemplo: 15.5
    pub cpu_usage: f32,
    /// Uso de memória RAM em Megabytes. Exemplo: 128.5
    pub mem_usage_mb: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub machine_id: String,
    pub hostname: String,
    pub os_distro: String,
    pub kernel_version: String,
    pub uptime: u64,
    pub virtualization: String,
    /// Número de série da placa-mãe. Exemplo: "XYZ-12345"
    pub bios_serial: String,
    /// Versão do Firmware. Exemplo: "UEFI 2.7.1"
    pub bios_version: String,
    /// Timestamp do momento em que o sistema ligou. Exemplo: 1704913600
    pub boot_time: i64,
    /// Fuso horário configurado. Exemplo: "America/Sao_Paulo"
    pub timezone: String,
    /// Desvio do relógio local vs servidor NTP em ms. Exemplo: 0.005
    pub ntp_offset_ms: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CpuStats {
    pub usage_total_pct: f32,
    pub load_avg: [f64; 3],
    pub threads_usage: Vec<f32>,
    pub interrupts_sec: u64,
    pub user_time: u64,
    pub system_time: u64,
    /// Tempo de espera por I/O de disco. Exemplo: 2.5
    pub io_wait_time: f32,
    pub idle_time: u64,
    /// Trocas de contexto por segundo. Exemplo: 5000
    pub context_switches_sec: u64,
    /// Frequência atual de cada núcleo em MHz. Exemplo: [3200, 2400, 3200]
    pub threads_freq_mhz: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryStats {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub buffers_cache_bytes: u64,
    pub swap_total_bytes: u64,
    pub swap_used_bytes: u64,
    /// Falhas graves (requerem leitura de disco). Exemplo: 10
    pub page_faults_major: u64,
    /// Falhas leves (resolvidas na RAM). Exemplo: 500
    pub page_faults_minor: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkStats {
    pub aggregate_rx_bytes_sec: u64,
    pub aggregate_tx_bytes_sec: u64,
    /// Quantidade total de pacotes recebidos. Exemplo: 15000
    pub aggregate_rx_packets: u64,
    /// Quantidade total de pacotes enviados. Exemplo: 12000
    pub aggregate_tx_packets: u64,
    pub total_errors: u64,
    pub total_drops: u64,
    /// Lista de interfaces físicas e seus IPs. Exemplo: {"eth0": ["192.168.1.10"]}
    pub interface_ips: std::collections::HashMap<String, Vec<String>>,
    pub tcp_active_connections: u32,
    pub tcp_time_wait_connections: u32,
    /// Portas TCP atualmente em escuta. Exemplo: [22, 80, 443]
    pub listening_ports: Vec<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageStats {
    pub partitions: Vec<PartitionInfo>,
    pub total_read_bytes_sec: u64,
    pub total_write_bytes_sec: u64,
    pub total_read_iops: u64,
    pub total_write_iops: u64,
    /// Latência média de I/O em ms (Queue Depth). Exemplo: 0.8
    pub io_latency_ms: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartitionInfo {
    pub mount_point: String,
    pub usage_pct: f32,
    pub free_bytes: u64,
    pub total_bytes: u64,
}

/// Sensores físicos (disponível em Bare Metal/Dedicado).
#[derive(Debug, Serialize, Deserialize)]
pub struct PhysicalSensors {
    /// Temperatura global da CPU em °C. Exemplo: 55.0
    pub cpu_temp: f32,
    /// Temperatura de cada núcleo individual. Exemplo: [52.0, 54.0, 55.0]
    pub core_temps: Vec<f32>,
    /// Temperatura de discos SSD/NVMe. Exemplo: [38.0]
    pub storage_temps: Vec<f32>,
    /// Rotação das ventoinhas em RPM. Exemplo: [2200, 2400]
    pub fan_speeds: Vec<u32>,
    /// Voltagem atual do núcleo da CPU. Exemplo: 1.15
    pub voltage_vcore: f32,
    /// Status de carga e saúde da bateria. Exemplo: "Charging (85%)"
    pub battery_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityStats {
    /// Data/Hora do último login via SSH ou local. Exemplo: "2024-05-20 14:00"
    pub last_login: String,
    /// Contador de falhas de autenticação de administrador. Exemplo: 0
    pub sudo_failures: u32,
    /// Indica se o Firewall (UFW/IPTables) está ativo. Exemplo: true
    pub firewall_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemHealth {
    pub entropy_avail: u32,
    pub active_users: u32,
}