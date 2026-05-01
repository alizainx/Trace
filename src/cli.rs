use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "trace")]
#[clap(about = "Universal, distro-agnostic system call tracing tool for Linux", version)]
#[clap(long_about = "A production-ready system call tracer for debugging and process analysis.")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,

    #[clap(short, long)]
    /// Trace a process by name
    pub process: Option<String>,

    #[clap(short, long)]
    /// Trace a process by PID
    pub pid: Option<u32>,

    #[clap(short, long)]
    /// Enable live tracing output
    pub live: bool,

    #[clap(short, long)]
    /// Output format (table, json, yaml)
    pub format: Option<String>,

    #[clap(short, long)]
    /// Save output to directory
    pub output: Option<String>,

    #[clap(short, long)]
    /// Verbose logging
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show system information
    Info,

    /// List running processes
    Processes,
}

impl Cli {
    pub fn validate(&self) -> crate::utils::TraceResult<()> {
        // Must specify either process name or PID
        if self.pid.is_none() && self.process.is_none() && self.command.is_none() {
            return Err(crate::utils::TraceError::ConfigError(
                "Must specify either --process <name> or --pid <pid>".to_string(),
            ));
        }

        // Validate output format if provided
        if let Some(format) = &self.format {
            match format.as_str() {
                "table" | "json" | "yaml" => {}
                _ => {
                    return Err(crate::utils::TraceError::ConfigError(format!(
                        "Invalid format: {}. Must be table, json, or yaml",
                        format
                    )))
                }
            }
        }

        Ok(())
    }

    pub fn get_output_format(&self) -> crate::output::OutputFormat {
        match self.format.as_deref() {
            Some("json") => crate::output::OutputFormat::Json,
            Some("yaml") => crate::output::OutputFormat::Yaml,
            _ => crate::output::OutputFormat::Table,
        }
    }
}
