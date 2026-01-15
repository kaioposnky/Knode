use std::fs::{read_link, read_to_string};
use std::sync::OnceLock;
use sysinfo::{System};
use crate::domain::models::Metadata;

struct CachedMetadata{
    pub machine_id: String,
    pub hostname: String,
    pub os_distro: String,
    pub virtualization: String,
    pub timezone: String,
}

static METADATA_CACHE: OnceLock<CachedMetadata> = OnceLock::new();

pub fn get_machine_metadata() -> Metadata {

    let cache = get_static_cache();

    let uptime = System::uptime(); // Uptime in seconds
    let boot_time = System::boot_time();

    Metadata {
        machine_id: cache.machine_id.trim().to_string(),
        hostname: cache.hostname.trim().to_string(),
        os_distro: cache.os_distro.trim().to_string(),
        virtualization: cache.virtualization.trim().to_string(),
        timezone: cache.timezone.trim().to_string(),
        boot_time: boot_time,
        uptime,
    }
}

fn get_static_cache() -> &'static CachedMetadata {
    METADATA_CACHE.get_or_init(|| {
        CachedMetadata {
            machine_id: get_machine_id(),
            hostname: System::host_name().expect("COULD NOT OBTAIN HOSTNAME!"),
            os_distro: os_info::get().to_string(),
            virtualization: detect_virtualization(&read_dmi("sys_vendor")),
            timezone: get_timezone(),
        }
    })
}

fn get_machine_id() -> String {

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
    String::new()
}

fn read_dmi(file: &str) -> String {
    let path = format!("/sys/class/dmi/{}", file);
    read_to_string(path)
        .unwrap_or_else(|_| "Permisison Denied/Unknown".to_string())
        .trim()
        .to_string()
}

fn detect_virtualization(vendor: &str) -> String {
    if vendor.contains("kvm") || vendor.contains("qemu") || vendor.contains("vmware")
        || vendor.contains("virtualbox") || vendor.contains("xen"){
        return format!("Virtual Machine from {}", vendor);
    }
    "Physical".to_string()
}

fn get_timezone() -> String {

    if let Ok(timezone) = read_to_string("/etc/timezone"){
        return timezone.trim().to_string();
    }

    if let Ok(timezone_path) = read_link("/etc/localtime"){

        let path = timezone_path.to_string_lossy();

        if let Some(pos) = path.find("zoneinfo/"){
            return path[pos + 9..].to_string();
        }
    }

    "Unkown".to_string()
}
