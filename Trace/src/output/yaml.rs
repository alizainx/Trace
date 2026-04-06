use crate::tracer::TraceData;

pub fn format_yaml(data: &TraceData) -> crate::utils::TraceResult<String> {
    let stats = data.syscalls.get_stats();
    let top_syscalls = stats
        .iter()
        .take(3)
        .map(|s| s.name.clone())
        .collect::<Vec<_>>()
        .join(", ");

    let yaml_output = format!(
        r#"process:
  pid: {}
  name: {}
  status: {}
  memory_mb: {}
  cpu_percent: {:.1}

syscalls:
  total: {}
  unique: {}
  top: {}

network:
  active_connections: {}
  bytes_sent: {}
  bytes_received: {}

timestamp: {}
"#,
        data.process.pid,
        data.process.name,
        data.process.status,
        data.memory.rss_mb,
        data.process.cpu_percent,
        data.syscalls.total_syscalls(),
        data.syscalls.unique_syscalls(),
        top_syscalls,
        data.network.active_connections,
        data.network.bytes_sent,
        data.network.bytes_received,
        chrono::Local::now().to_string()
    );

    Ok(yaml_output)
}
