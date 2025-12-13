use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use time::format_description;
use time::OffsetDateTime;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const PROGRAM_NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if help option is provided
    if args.len() > 1 && (args[1] == "-h" || args[1] == "--help") {
        print_help();
        process::exit(1);
    }

    // Check if argument is provided
    if args.len() < 2 {
        eprintln!("Error: No file path provided.");
        eprintln!("Use -h or --help for usage information.");
        process::exit(1);
    }

    let input_path_str = &args[1];

    // Resolve the full absolute path
    let input_path = match resolve_path(input_path_str) {
        Ok(path) => path,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    // Check if path exists
    if !input_path.exists() {
        eprintln!("Error: The specified path does not exist: {}", input_path.display());
        process::exit(1);
    }

    // Check if path is a file (not a directory or link)
    if !input_path.is_file() {
        eprintln!("Error: The specified path is not a file: {}", input_path.display());
        process::exit(1);
    }

    // Get current local time and format it
    let datetime_string = match get_datetime_string() {
        Ok(s) => s,
        Err(err) => {
            eprintln!("Error: Failed to get current time: {}", err);
            process::exit(1);
        }
    };

    // Calculate the backup file path
    let backup_path = create_backup_path(&input_path, &datetime_string);

    // Check if backup file already exists
    if backup_path.exists() {
        eprintln!(
            "Error: Backup file already exists: {}",
            backup_path.display()
        );
        process::exit(1);
    }

    // Copy the file to the backup path
    if let Err(err) = fs::copy(&input_path, &backup_path) {
        eprintln!(
            "Error: Failed to copy file: {}",
            err
        );
        process::exit(1);
    }

    println!("Successfully created backup: {}", backup_path.display());
    process::exit(0);
}

fn print_help() {
    println!("{} v{}", PROGRAM_NAME, VERSION);
    println!();
    println!("USAGE:");
    println!("    {} <FILE>", PROGRAM_NAME);
    println!("    {} [OPTIONS]", PROGRAM_NAME);
    println!();
    println!("ARGUMENTS:");
    println!("    <FILE>    Path to the file to backup");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help    Show this help message");
    println!();
    println!("DESCRIPTION:");
    println!("    Creates a timestamped backup copy of the specified file.");
    println!("    The backup file will have the format: <filename>.YYYYMMDD_HHMMSS.bak");
    println!();
    println!("EXAMPLES:");
    println!("    {} test.txt", PROGRAM_NAME);
    println!("    {} ~/documents/important.doc", PROGRAM_NAME);
    #[cfg(windows)]
    println!("    {} C:\\Temp\\test.txt", PROGRAM_NAME);
}

fn resolve_path(path_str: &str) -> Result<PathBuf, String> {
    let path = if path_str.starts_with("~/") {
        // Expand ~ to home directory
        let home_dir = dirs::home_dir()
            .ok_or_else(|| "Failed to determine home directory".to_string())?;
        let relative = &path_str[2..];
        home_dir.join(relative)
    } else {
        PathBuf::from(path_str)
    };

    // Canonicalize to get absolute path
    // On Windows, this will handle both forward and backward slashes
    path.canonicalize()
        .map_err(|e| format!("Failed to resolve path '{}': {}", path_str, e))
}

fn get_datetime_string() -> Result<String, String> {
    let now = OffsetDateTime::now_local()
        .map_err(|e| format!("Failed to get local time: {}", e))?;
    
    let format = format_description::parse("[year][month][day]_[hour][minute][second]")
        .map_err(|e| format!("Failed to parse format: {}", e))?;
    
    now.format(&format)
        .map_err(|e| format!("Failed to format time: {}", e))
}

fn create_backup_path(original_path: &Path, datetime_string: &str) -> PathBuf {
    let file_name = original_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");
    
    let backup_name = format!("{}.{}.bak", file_name, datetime_string);
    
    if let Some(parent) = original_path.parent() {
        parent.join(backup_name)
    } else {
        PathBuf::from(backup_name)
    }
}
