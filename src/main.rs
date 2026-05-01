//Maintained and Developed by Ali Zain <alizain.x404@gmail.com>

use clap::Parser;
use trace::*;
use colored::Colorize;
use log::info;

fn main() {
    let cli = Cli::parse();

    // Initialize logging
    utils::init_logger(cli.verbose);

    // Validate CLI arguments
    if let Err(e) = cli.validate() {
        eprintln!("{} {}", "✗".red(), e);
        std::process::exit(1);
    }

    // Verify permissions
    if let Err(e) = sandbox::verify_permissions() {
        eprintln!("{} {}", "✗".red(), e);
        std::process::exit(1);
    }

    // Handle subcommands
    if let Some(cmd) = &cli.command {
        match cmd {
            cli::Commands::Info => {
                if let Err(e) = handle_info() {
                    eprintln!("{} {}", "✗".red(), e);
                    std::process::exit(1);
                }
                return;
            }
            cli::Commands::Processes => {
                if let Err(e) = handle_processes() {
                    eprintln!("{} {}", "✗".red(), e);
                    std::process::exit(1);
                }
                return;
            }
        }
    }

    // Handle main tracing functionality
    let target_pid = match (&cli.process, &cli.pid) {
        (Some(name), _) => {
            // Find process by name
            match tracer::ProcessInfo::from_name(name) {
                Ok(proc) => proc.pid,
                Err(_) => {
                    eprintln!("\n{} Process '{}' not found or not running.\n", "✗".red(), name.cyan());
                    eprintln!("   Quick fixes:");
                    eprintln!("   • Check process name spelling");
                    eprintln!("   • Use PID instead: {} --pid 1234", "trace".bold());
                    eprintln!("   • List running processes: {} processes\n", "trace".bold());
                    std::process::exit(1);
                }
            }
        }
        (_, Some(pid)) => {
            // Verify PID exists
            if !tracer::ProcessInfo::exists(*pid) {
                eprintln!("\n{} Process with PID {} not found.\n", "✗".red(), pid.to_string().cyan());
                eprintln!("   Quick fixes:");
                eprintln!("   • Verify the PID is correct");
                eprintln!("   • List running processes: {} processes", "trace".bold());
                eprintln!("   • Check your permissions (may need root)\n");
                std::process::exit(1);
            }
            *pid
        }
        _ => {
            eprintln!("\n{} No process specified.\n", "✗".red());
            eprintln!("   Usage:");
            eprintln!("   • {} --process <name>  (trace by process name)", "trace".bold());
            eprintln!("   • {} --pid <PID>       (trace by process ID)", "trace".bold());
            eprintln!("   • {} info              (show system info)", "trace".bold());
            eprintln!("   • {} processes         (list running processes)\n", "trace".bold());
            std::process::exit(1);
        }
    };

    // Collect trace data
    match TraceData::collect(target_pid) {
        Ok(data) => {
            // Format output
            let output_format = cli.get_output_format();
            match output_format.format(&data) {
                Ok(formatted) => {
                    println!("{}", formatted);

                    // Save to file if requested
                    if let Some(output_dir) = &cli.output {
                        let filename = utils::generate_filename(&data.process.name, "json");
                        match utils::write_output_file(output_dir, &filename, &formatted) {
                            Ok(path) => {
                                println!("Output saved: {}", path.green());
                            }
                            Err(e) => {
                                eprintln!("\n{} Failed to save output: {}", "⚠".yellow(), e);
                            }
                        }
                    }
                    info!("Trace completed successfully for PID {}", target_pid);
                }
                Err(e) => {
                    eprintln!("\n{} Failed to format output: {}\n", "✗".red(), e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("\n{} Failed to collect trace data: {}\n", "✗".red(), e);
            std::process::exit(1);
        }
    }
}

fn handle_info() -> TraceResult<()> {
    println!("\n{}\n", "System Information".green().bold().underline());

    match OsInfo::detect() {
        Ok(os) => {
            println!("OS Name: {}", os.name);
            println!("OS Version: {}", os.version);
            println!("Distro: {}", os.distro);
        }
        Err(e) => {
            eprintln!("Failed to detect OS: {}", e);
        }
    }

    match KernelInfo::detect() {
        Ok(kernel) => {
            println!("Kernel Version: {}", kernel.version);
            println!("Architecture: {}", kernel.arch);
        }
        Err(e) => {
            eprintln!("Failed to detect kernel: {}", e);
        }
    }

    println!();
    Ok(())
}

fn handle_processes() -> TraceResult<()> {
    println!("\n{}\n", "Running Processes".green().bold().underline());

    if let Ok(procs) = procfs::process::all_processes() {
        for proc in procs.take(20) {
            if let Ok(proc) = proc {
                if let Ok(stat) = proc.stat() {
                    println!("  {} - {} (UID: {})", proc.pid(), stat.comm, proc.uid()?);
                }
            }
        }
        println!("\n(Showing first 20 processes)\n");
        Ok(())
    } else {
        Err(TraceError::IoError("Failed to read processes".to_string()))
    }
}
