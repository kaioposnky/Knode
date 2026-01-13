use chrono::Utc;
use sysinfo::System;
use crate::domain::models::{CpuStats, MachineReport, MemoryStats, Metadata, NetworkStats, PhysicalSensors, ProcessStats, SecurityStats, StorageStats, SystemHealth};

mod cpu;
mod memory;
mod network;
mod processes;
mod storage;
mod security;
mod metadata;

pub struct TelemetryCollector {
    sys: System,
    // static_metadata: Metadata
}

impl TelemetryCollector {
    pub fn new() -> Self {
        let mut sys = System::new_all();

        sys.refresh_all();

        TelemetryCollector {
            sys: sys,
            // static_metadata:
        }
    }

    pub fn collect_telemetry(self) -> MachineReport {

        let metadata: Metadata;
        let cpu: CpuStats;
        let memory: MemoryStats;
        let processes: ProcessStats;
        let network: NetworkStats;
        let storage: StorageStats;
        let sensors: PhysicalSensors;
        let security: SecurityStats;
        let health: SystemHealth;
        let timestamp = Utc::now().timestamp();

        MachineReport {
             metadata: metadata,
             cpu: cpu,
             memory: memory,
             processes: processes,
             network: network,
             storage: storage,
             sensors: sensors,
             security: security,
             health: health,
             timestamp: timestamp
        }
    }
}