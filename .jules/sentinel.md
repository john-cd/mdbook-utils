## YYYY-MM-DD - Arbitrary File Overwrite via Unvalidated Paths
**Vulnerability:** In `src/dependencies/get_dependencies.rs`, the `write_log` function accepted an optional `log_file_path` but passed it directly to `File::create(path)` without performing any boundary checks. This allowed path traversal to overwrite arbitrary files on the system if `log_file_path` was influenced by external input.
**Learning:** Blindly trusting explicitly provided output file paths for logs or build artifacts without validating them against an expected base directory (such as the current working directory) creates a critical risk of arbitrary file overwrite.
**Prevention:** Always resolve the intended base directory (e.g., `std::env::current_dir()`) and validate the user-provided path against it using `crate::fs::is_path_within` before invoking `File::create`.
## YYYY-MM-DD - Arbitrary File Overwrite via Unvalidated Paths
**Vulnerability:** Hardcoded paths like `./book/temp/test.log` when creating temporary files allow potential arbitrary file overwrites.
**Learning:** Blindly writing to fixed relative paths without creating explicit secure temporary directories makes the application vulnerable.
**Prevention:** Use the `tempfile` crate to securely generate temporary files in the system's designated temp directory.
## YYYY-MM-DD - Hardcoded Test Log File Overwrite
**Vulnerability:** In `src/api/debug.rs` and `src/lib.rs`, the `test()` function created a log file using a hardcoded, predictable path (`./book/temp/test.log`) via `File::create(dest_file_path)` without ensuring exclusive creation or using the system temporary directory. This can lead to arbitrary file overwrite or symlink attacks if the working directory is shared or if the directory name matches a critical system path.
**Learning:** Even in test or debug functions, hardcoding file paths in standard directories like `./book/temp/` using `File::create` exposes the application to path manipulation and predictable file overwrites.
**Prevention:** Always use the `tempfile` crate (e.g., `tempfile::Builder::new().tempfile()`) to securely generate temporary files in the system designated temp folder. This guarantees uniqueness, proper permissions, and automatic cleanup.
