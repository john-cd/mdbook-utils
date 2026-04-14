import re

with open("src/link/link_and_linkbuilder/link.rs", "r") as f:
    content = f.read()

content = content.replace("    use super::*;", "    use super::*;\n    use crate::link::LinkBuilder;")

with open("src/link/link_and_linkbuilder/link.rs", "w") as f:
    f.write(content)
