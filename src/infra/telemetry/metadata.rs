use sysinfo::System;
use crate::domain::models::Metadata;

fn get_machine_metadata(sys: System) -> Metadata {

    // pub machine_id: String,
    let machine_id = get_machine_id();

    // pub hostname: String,

    // pub os_distro: String,
    // pub kernel_version: String,
    // pub uptime: u64,
    // pub virtualization: String,
    // /// Número de série da placa-mãe. Exemplo: "XYZ-12345"
    // pub bios_serial: String,
    // /// Versão do Firmware. Exemplo: "UEFI 2.7.1"
    // pub bios_version: String,
    // /// Timestamp do momento em que o sistema ligou. Exemplo: 1704913600
    // pub boot_time: i64,
    // /// Fuso horário configurado. Exemplo: "America/Sao_Paulo"
    // pub timezone: String,
    // /// Desvio do relógio local vs servidor NTP em ms. Exemplo: 0.005
    // pub ntp_offset_ms: f32,

    Metadata {

    }
}

pub fn get_machine_id() -> String {

    let machine_uuid_paths = ["/var/lib/dbus/machine-id", "/etc/machine-id"];

    for path in machine_uuid_paths{
        match std::fs::read_to_string(path) {
            Ok(id) => {
                return id;
            }

            Err(_) => {
                continue;
            }
        }
    }
    "".to_string()
}