<!-- TODO Reviewed ??-??-2026 -->
## Reference Definitions

`mdbook-utils refdefs` offers the following subcommands:

```txt
Manage reference definitions

Usage: mdbook-utils refdefs [OPTIONS] <COMMAND>

Commands:
  write     Write existing reference definitions to a file
  generate  Generate reference definitions (e.g. badges for Github links or from the dependencies of the code examples)
  help      Print this message or the help of the given subcommand(s)

Options:
  -y, --yes        Automatically answer `yes` to any user confirmation request
      --log <LOG>  Set the logging level (error, warn, info, debug, trace) [env: RUST_LOG=]
  -h, --help       Print help
```

Generate GitHub badges with the default flat Shields.io style:

```sh
mdbook-utils refdefs generate --badges --markdown-dir ./src --output ./badge_refs.md
```

Choose another supported style with `--badge-style`. The style option also
enables badge generation, so `--badges` may be omitted:

```sh
mdbook-utils refdefs generate --badge-style flat-square --markdown-dir ./src --output ./badge_refs.md
```

Supported values are `flat`, `flat-square`, `plastic`, `for-the-badge`, and
`social`. Omitting the option keeps the default flat output and leaves the
`style` query parameter out of generated URLs.

{{#include ../refs.md}}
