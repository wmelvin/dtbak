use std::process::Command;

#[test]
fn test_help_short_flag() {
    let output = Command::new(env!("CARGO_BIN_EXE_dtbak"))
        .arg("-h")
        .output()
        .expect("Failed to execute dtbak");

    // Convert output to string
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Verify help message contains expected content
    assert!(stdout.contains("USAGE:"), "Help output should contain USAGE section");
    assert!(stdout.contains("ARGUMENTS:"), "Help output should contain ARGUMENTS section");
    assert!(stdout.contains("OPTIONS:"), "Help output should contain OPTIONS section");
    assert!(stdout.contains("-h, --help"), "Help output should mention -h and --help flags");
    assert!(stdout.contains("dtbak"), "Help output should contain program name");
}

#[test]
fn test_help_long_flag() {
    let output = Command::new(env!("CARGO_BIN_EXE_dtbak"))
        .arg("--help")
        .output()
        .expect("Failed to execute dtbak");

    // Convert output to string
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Verify help message contains expected content
    assert!(stdout.contains("USAGE:"), "Help output should contain USAGE section");
    assert!(stdout.contains("ARGUMENTS:"), "Help output should contain ARGUMENTS section");
    assert!(stdout.contains("OPTIONS:"), "Help output should contain OPTIONS section");
    assert!(stdout.contains("-h, --help"), "Help output should mention -h and --help flags");
    assert!(stdout.contains("dtbak"), "Help output should contain program name");
}

#[test]
fn test_help_outputs_are_identical() {
    let output_short = Command::new(env!("CARGO_BIN_EXE_dtbak"))
        .arg("-h")
        .output()
        .expect("Failed to execute dtbak with -h");

    let output_long = Command::new(env!("CARGO_BIN_EXE_dtbak"))
        .arg("--help")
        .output()
        .expect("Failed to execute dtbak with --help");

    // Both flags should produce identical output
    assert_eq!(
        output_short.stdout,
        output_long.stdout,
        "Both -h and --help should produce identical output"
    );
}
