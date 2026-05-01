use crate::tracer::TraceData;
use serde_json::json;

pub fn format_json(data: &TraceData) -> crate::utils::TraceResult<String> {
    let stats = data.syscalls.get_stats();
    let top_syscalls = stats.iter().take(3).map(|s| s.name.clone()).collect::<Vec<_>>();

    let json = json!({
        "process": {
            "pid": data.process.pid,
            "name": data.process.name,
            "status": data.process.status,
            "memory_mb": data.memory.rss_mb,
            "cpu_percent": data.process.cpu_percent
        },
        "syscalls": {
            "total": data.syscalls.total_syscalls(),
            "unique": data.syscalls.unique_syscalls(),
            "top": top_syscalls,
            "detailed": stats.iter().map(|s| {
                json!({
                    "name": s.name,
                    "count": s.count,
                    "bytes": s.bytes
                })
            }).collect::<Vec<_>>()
        },
        "network": {
            "active_connections": data.network.active_connections,
            "bytes_sent": data.network.bytes_sent,
            "bytes_received": data.network.bytes_received
        },
        "timestamp": chrono::Local::now().to_rfc3339()
    });

    Ok(serde_json::to_string_pretty(&json)?)
}
