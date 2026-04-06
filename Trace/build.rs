fn main() {
    // You can add build-time checks and setup here
    
    // Example: Check that we're on a supported platform
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS not set");
    
    if target_os != "linux" {
        panic!("trace only supports Linux. Target OS: {}", target_os);
    }

    // Verify we're on a supported architecture
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH not set");
    
    match target_arch.as_str() {
        "x86_64" | "aarch64" | "arm" => {
            println!("cargo:rustc-env=TRACE_ARCH={}", target_arch);
        }
        _ => {
            eprintln!("Warning: trace is primarily tested on x86_64, aarch64, and arm");
            eprintln!("Current target: {}", target_arch);
        }
    }

    // Print build information
    println!("cargo:rustc-env=TRACE_BUILD_TIME={}", chrono_build_time());
}

fn chrono_build_time() -> String {
    #[cfg(feature = "build-time")]
    {
        use std::time::SystemTime;
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => duration.as_secs().to_string(),
            Err(_) => "unknown".to_string(),
        }
    }
    #[cfg(not(feature = "build-time"))]
    {
        "unknown".to_string()
    }
}
