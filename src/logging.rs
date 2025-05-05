use chrono::Local;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use tracing::info;
use tracing_subscriber::filter::LevelFilter;

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, Layer};

pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    //Generate timestamped log filename
    let timestamp = Local::now().format("dalli-klick-%Y-%m-%dT%H-%M-%S.log").to_string();
    let log_path = PathBuf::from("logs").join(timestamp);
    
    //Create logs directory if missing
    std::fs::create_dir_all("logs")?;
    
    let _ = File::create(&log_path)?;
    
    //Create or update latest.log symlink or copy
    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        let _ = std::fs::remove_file("logs/latest.log");
        let _ = symlink(&log_path, "logs/latest.log");
    }
    
    #[cfg(windows)]
    {
        use std::os::windows::fs::symlink_file;
        if let Err(_) = symlink_file(&log_path, "logs/latest.log") {
            let _ = std::fs::copy(&log_path, "logs/latest.log");
            info!("Could not create symlink")
        }
    }
    
    let stdout_layer = fmt::layer()
        .with_writer(io::stdout)
        .with_target(true)
        .with_line_number(true)
        .with_file(true)
        .with_ansi(true)
        .with_filter(LevelFilter::DEBUG);
        
    
    let file_layer = fmt::layer()
        .with_writer(move || File::options().append(true).open(&log_path).unwrap())
        .with_target(true)
        .with_line_number(true)
        .with_file(true)
        .with_ansi(false)
        .with_filter(LevelFilter::DEBUG);
    
    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(file_layer)
        .init();
    
    info!("Initialised logger");
    info!("Saving current log into latest.log");
    Ok(())
}