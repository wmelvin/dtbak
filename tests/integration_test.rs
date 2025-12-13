use std::fs;
#[cfg(unix)]
use std::os::unix::fs as unix_fs;
#[cfg(windows)]
use std::os::windows::fs as windows_fs;
use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_no_arguments_shows_error() {
    let output = Command::new(env!("CARGO_BIN_EXE_dtbak"))
        .output()
        .expect("Failed to execute dtbak");

    // Should exit with error code
    assert!(!output.status.success(), "Should exit with error code when no arguments provided");

    // Convert stderr to string
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Verify error message
    assert!(stderr.contains("Error: No file path provided"), 
        "Should show error message about no file path provided");
    assert!(stderr.contains("Use -h or --help"), 
        "Should suggest using help for usage information");
}

#[test]
fn test_nonexistent_path_shows_error() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let nonexistent_path = temp_dir.path().join("this_file_does_not_exist.txt");

    let output = Command::new(env!("CARGO_BIN_EXE_dtbak"))
        .arg(&nonexistent_path)
        .output()
        .expect("Failed to execute dtbak");

    // Should exit with error code
    assert!(!output.status.success(), "Should exit with error code when path does not exist");

    // Convert stderr to string
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Verify error message - the path must be resolved before checking existence
    // so the error could be from resolve_path or the existence check
    assert!(
        stderr.contains("Error:") && 
        (stderr.contains("does not exist") || stderr.contains("Failed to resolve path")),
        "Should show error message about path not existing or failing to resolve. Got: {}", 
        stderr
    );
}

#[test]
fn test_directory_path_shows_error() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let output = Command::new(env!("CARGO_BIN_EXE_dtbak"))
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute dtbak");

    // Should exit with error code
    assert!(!output.status.success(), "Should exit with error code when path is a directory");

    // Convert stderr to string
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Verify error message
    assert!(stderr.contains("Error: The specified path is not a file"), 
        "Should show error message about path not being a file. Got: {}", stderr);
}

#[test]
fn test_symlink_to_directory_shows_error() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let dir_path = temp_dir.path().join("some_directory");
    let link_path = temp_dir.path().join("link_to_dir");

    // Create a directory
    fs::create_dir(&dir_path).expect("Failed to create directory");

    // Create a symlink to the directory
    #[cfg(unix)]
    unix_fs::symlink(&dir_path, &link_path).expect("Failed to create symlink");
    #[cfg(windows)]
    windows_fs::symlink_dir(&dir_path, &link_path).expect("Failed to create symlink");

    let output = Command::new(env!("CARGO_BIN_EXE_dtbak"))
        .arg(&link_path)
        .output()
        .expect("Failed to execute dtbak");

    // Should exit with error code
    assert!(!output.status.success(), "Should exit with error code when path is a symlink to a directory");

    // Convert stderr to string
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Verify error message
    assert!(stderr.contains("Error: The specified path is not a file"), 
        "Should show error message about path not being a file. Got: {}", stderr);
}

#[test]
fn test_valid_file_creates_backup() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test.txt");
    let file_content = "This is a test file content";

    // Create a test file
    fs::write(&file_path, file_content).expect("Failed to create test file");

    let output = Command::new(env!("CARGO_BIN_EXE_dtbak"))
        .arg(&file_path)
        .output()
        .expect("Failed to execute dtbak");

    // Should exit successfully
    assert!(output.status.success(), 
        "Should exit successfully when backing up a valid file. stderr: {}", 
        String::from_utf8_lossy(&output.stderr));

    // Convert stdout to string
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Verify success message
    assert!(stdout.contains("Successfully created backup"), 
        "Should show success message. Got: {}", stdout);

    // Verify the backup file exists and has correct content
    let backup_files: Vec<_> = fs::read_dir(temp_dir.path())
        .expect("Failed to read directory")
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            name_str.starts_with("test.txt.") && name_str.ends_with(".bak")
        })
        .collect();

    assert_eq!(backup_files.len(), 1, 
        "Should have created exactly one backup file");

    let backup_file = &backup_files[0];
    let backup_content = fs::read_to_string(backup_file.path())
        .expect("Failed to read backup file");

    assert_eq!(backup_content, file_content, 
        "Backup file should have the same content as the original");

    // Verify the backup filename format (should be test.txt.YYYYMMDD_HHMMSS.bak)
    let backup_name = backup_file.file_name();
    let backup_name_str = backup_name.to_string_lossy();
    assert!(backup_name_str.starts_with("test.txt."), 
        "Backup filename should start with original filename");
    assert!(backup_name_str.ends_with(".bak"), 
        "Backup filename should end with .bak");
    
    // Verify the timestamp format (8 digits for date, underscore, 6 digits for time)
    let timestamp_part = backup_name_str
        .trim_start_matches("test.txt.")
        .trim_end_matches(".bak");
    assert_eq!(timestamp_part.len(), 15, 
        "Timestamp should be 15 characters (YYYYMMDD_HHMMSS)");
    assert!(timestamp_part.chars().nth(8) == Some('_'), 
        "Character at position 8 should be an underscore");
}
