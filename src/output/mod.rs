pub mod json;
pub mod yaml;
pub mod table;

use crate::tracer::TraceData;
use crate::utils::TraceResult;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Table,
    Json,
    Yaml,
}

impl OutputFormat {
    pub fn format(&self, data: &TraceData) -> TraceResult<String> {
        match self {
            OutputFormat::Table => table::format_table(data),
            OutputFormat::Json => json::format_json(data),
            OutputFormat::Yaml => yaml::format_yaml(data),
        }
    }
}
