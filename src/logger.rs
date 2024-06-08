use flexi_logger::{Logger, FileSpec, Criterion, Naming, Cleanup};

pub fn initialize_logger() {
    let temp_dir = std::env::temp_dir();
    let log_file_name = "ruskgpt";
    
    Logger::try_with_str("info")
        .unwrap()
        .log_to_file(FileSpec::default().directory(temp_dir).basename(log_file_name))
        .rotate(
            Criterion::Size(10_000_000), // Rotate log file after it reaches 10 MB
            Naming::Timestamps,          // Use timestamps for rotated file names
            Cleanup::KeepLogFiles(7),    // Keep a maximum of 7 log files
        )
        .start()
        .unwrap();
}
