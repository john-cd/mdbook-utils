import re

with open("src/dependencies/get_dependencies.rs", "r") as f:
    content = f.read()

# Completely rewrite write_log
replacement = """/// Write e.g. stdout / stderr to a file.
fn write_log(out: &[u8], err: &[u8], log_file_path: Option<&Path>) -> Result<()> {
    let (file, actual_path) = match log_file_path {
        Some(path) => match File::create(path) {
            Ok(f) => (f, path.to_path_buf()),
            Err(e) => {
                warn!("Failed to create log file {}: {}", path.display(), e);
                return Ok(());
            }
        },
        None => match Builder::new()
            .prefix("mdbook-utils-dependencies-")
            .suffix(".log")
            .tempfile()
        {
            Ok(tf) => {
                let actual_path = tf.path().to_path_buf();
                match tf.keep() {
                    Ok((f, _p)) => (f, actual_path),
                    Err(e) => {
                        warn!("Failed to keep temporary log file: {}", e);
                        return Ok(());
                    }
                }
            },
            Err(e) => {
                warn!("Failed to create temporary log file: {}", e);
                return Ok(());
            }
        },
    };

    let mut buffer = BufWriter::new(file);
    if let Err(e) = buffer
        .write_all(out)
        .and_then(|_| buffer.write_all(err))
        .and_then(|_| buffer.flush())
    {
        warn!(
            "Failed to write to log file {}: {}",
            actual_path.display(),
            e
        );
    } else {
        debug!("Dependencies log written to {}", actual_path.display());
    }
    Ok(())
}"""

content = re.sub(r'/// Write e\.g\. stdout / stderr to a file\..*?Ok\(\(\)\)\n\}', replacement, content, flags=re.DOTALL)

with open("src/dependencies/get_dependencies.rs", "w") as f:
    f.write(content)
