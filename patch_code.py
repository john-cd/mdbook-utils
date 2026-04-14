import re

def fix_file(file_path):
    with open(file_path, "r") as f:
        content = f.read()

    # Fix P2 trait bound
    content = content.replace("P2: AsRef<Path>,", "P2: AsRef<Path> + std::marker::Sync,")
    content = content.replace("P: AsRef<Path>,", "P: AsRef<Path> + std::marker::Sync,")

    with open(file_path, "w") as f:
        f.write(content)

fix_file("src/markdown/extract_code.rs")
fix_file("src/markdown/replace_include.rs")
