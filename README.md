# dtbak - Timestamped File Backup Utility

A simple command-line utility for creating timestamped backup copies of files on Linux and Windows.

## Features

- Creates timestamped backup copies of files
- Supports both Windows and POSIX (Linux/macOS) paths
- Handles relative paths and `~/` home directory expansion
- Prevents accidental overwrites (fails if backup already exists)
- Cross-platform support (Linux and Windows)

## Usage

```bash
dtbak <FILE>
dtbak [OPTIONS]
```

### Arguments

- `<FILE>` - Path to the file to backup

### Options

- `-h, --help` - Show help message and exit

## Examples

```bash
# Backup a file in the current directory
dtbak test.txt

# Backup a file using absolute path
dtbak /path/to/important.doc

# Backup a file in home directory
dtbak ~/documents/important.doc

# Windows example
dtbak C:\Temp\test.txt
```

## Backup File Format

The backup file name is created by appending a timestamp and `.bak` extension to the original filename:

```
<original-filename>.YYYYMMDD_HHMMSS.bak
```

### Examples:

- `test.txt` → `test.txt.20251213_090807.bak`
- `document.pdf` → `document.pdf.20251213_143022.bak`

## Building

### Build for your current platform:

```bash
cargo build --release
```

The executable will be located at `target/release/dtbak` (or `dtbak.exe` on Windows).

### Cross-compilation for Windows (from Linux):

```bash
# Install cross-compilation target
rustup target add x86_64-pc-windows-gnu

# Build for Windows
cargo build --release --target x86_64-pc-windows-gnu
```

## Exit Codes

- `0` - Success
- `1` - Error (see error message for details)

## Error Conditions

The program will exit with an error in the following cases:

- No file path is provided
- The specified path does not exist
- The specified path is not a file (directories and symlinks are not supported)
- The backup file already exists
- The file copy operation fails

## Dependencies

- `time` - For timestamp formatting
- `dirs` - For home directory resolution

## CI/CD

GitHub Actions workflows are configured to automatically build executables for both Linux and Windows on every push to the main branch. The built executables are available as artifacts in the Actions tab.

## License

This project is provided as-is for educational and personal use.
