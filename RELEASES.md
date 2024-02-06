# RELEASES

## v0.1.1

- Enable markdown > remove-includes subcommand.
- Add `-y` global option to skip confirmation requests.
- Tool install instructions.
- Add BOOK_MARKDOWN_BUILD_DIR_PATH environment variable and provide a default value when `[output.markdown]` is added to `book.toml`.
- Sitemap generation: fix the location of `sitemap.xml` when `book.toml` includes more than one [output.*] table.
- Use `mdbook`'s default values, if `book.toml` is found, but `book.src` or `build.build-dir` are not present.
- Documentation.

## v0.1.0

- Initial release
