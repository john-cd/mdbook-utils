# RELEASES

## vNext (0.1.3)

- Updated main README
- Removed container after run in CI workflow
- Upgraded to Rust 1.76
- Code documentation improvements
- Added test stubs
- Added cargo bump
- Excluded unecessary files from package

## v0.1.2

- markdown > remove-includes:
  - added hard-coded string to replace the removed includes.
  - List modified files when calling the markdown > remove-includes subcommand.
  - Return a list of modified files from the markdown::remove_includes_in_all_markdown_files_in function
- Add RELEASES.md file.
- Exclude unnecessary files from package.

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
