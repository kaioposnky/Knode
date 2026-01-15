use sysinfo::System;
use crate::domain::models::MemoryStats;
use std::fs;
use std::collections::HashMap;


pub fn get_system_memory_info(sys: &System) -> Result<MemoryStats, std::io::Error> {

    let total = sys.total_memory();
    let free = sys.free_memory();
    let used = sys.used_memory();
    let swap_total = sys.total_swap();
    let swap_free = sys.free_swap();



    Ok(MemoryStats {
        total_bytes: total,
        used_bytes: used,
        free_bytes: free,
        buffers_cache_bytes: get_buffers_and_cache_bytes(),
        swap_total_bytes: swap_total,
        swap_free_bytes: swap_free,
    })
}


fn get_buffers_and_cache_bytes() -> u64 {
    // File with memory info on Linux
    let content = fs::read_to_string("/proc/meminfo").unwrap_or_else(|e| {

        eprint!("Error reading /proc/meminfo for buffers and cached memory info! Are you sure you are running on Linux ? \nThe default value (0) is going to be used.\n");

        return String::new();
    });
    let mut values: HashMap<&str, u64> = HashMap::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let key = parts[0].trim_end_matches(':');
            // Values in /proc/meminfo are in kB
            let value: u64 = parts[1].parse().unwrap_or(0) * 1024; // Convert to bytes
            values.insert(key, value);
        }
    }

    let buffers = *values.get("Buffers").unwrap_or(&0);
    let cached = *values.get("Cached").unwrap_or(&0);

    return buffers.saturating_add(cached);
}
