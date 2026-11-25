# GitHub Actions workflows

<!-- workflow-operations-reference:start -->

<!-- TODO Reviewed ??-??-2026. -->

## Workflow operations reference

The workflow YAML files own execution behavior. Their inline comments explain the purpose and
the boundaries that must survive future refactoring. This reference records the current job map;
repository-level Actions enable/disable settings remain independent of these tracked definitions.

### Changes in this review

- pages.yml: Bound build to 120 minutes.
- pages.yml: Bound deploy to 30 minutes.
- publish.yml: Bound build to 60 minutes.
- publish.yml: Bound pages-deploy to 30 minutes.
- release.yml: Added branch-scoped concurrency without cancelling publication.
- release.yml: Bound build to 120 minutes.
- release.yml: Bound release to 30 minutes.

### [Rust CI](ci.yml)

Events: `pull_request, push, workflow_dispatch`. Branch/path filters and manual defaults are defined in the linked file.

| Job | Runs on / reuse | Dependencies | Guard | Main checks or side effects |
| --- | --- | --- | --- | --- |
| changes | "ubuntu-latest" | [] | Successful dependencies (default) | actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1; dorny/paths-filter@0e4a8c6effa4802afeda77dc8d303f8176d7dfad |
| validate | "${{ matrix.os }}" | "changes" | ${{ github.event_name == 'workflow_dispatch' \|\| needs.changes.outputs.code == 'true' }} | actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1; Load pinned stable Rust version; dtolnay/rust-toolchain@683b588c487a69d01f6c6ddf40f036ff682cc2c8; Install pinned nightly rustfmt; Check formatting; Install compiler cache; Swatinem/rust-cache@d1527c8f67ef9af357290475d4af4b0fab8f12e0; taiki-e/install-action@5488ac5a951645ce95062214902ac4b6bc1a1037; Run Clippy; Run tests; Run doctests; Build API documentation; Build and test mdBooks |
| documentation | "ubuntu-latest" | "changes" | needs.changes.outputs.code != 'true' | actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1; dprint/check@371a959e3c77f34a76a03d2c50df2e3f2302d016; taiki-e/install-action@5488ac5a951645ce95062214902ac4b6bc1a1037; Build and test mdBooks |
| coverage | "ubuntu-latest" | "changes" | ${{ github.event_name == 'workflow_dispatch' \|\| needs.changes.outputs.code == 'true' }} | actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1; Load pinned stable Rust version; dtolnay/rust-toolchain@683b588c487a69d01f6c6ddf40f036ff682cc2c8; Install compiler cache; Swatinem/rust-cache@d1527c8f67ef9af357290475d4af4b0fab8f12e0; taiki-e/install-action@5488ac5a951645ce95062214902ac4b6bc1a1037; Report line coverage |
| msrv | "ubuntu-latest" | "changes" | ${{ github.event_name == 'workflow_dispatch' \|\| needs.changes.outputs.code == 'true' }} | actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1; dtolnay/rust-toolchain@683b588c487a69d01f6c6ddf40f036ff682cc2c8; Check declared minimum version |
| container-build | "ubuntu-latest" | "changes" | ${{ github.event_name == 'workflow_dispatch' \|\| needs.changes.outputs.container == 'true' }} | actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1; Load pinned container build inputs; Build the CI image |

### [Deploy the user guide to Pages](pages.yml)

Events: `push, workflow_dispatch`. Branch/path filters and manual defaults are defined in the linked file.

| Job | Runs on / reuse | Dependencies | Guard | Main checks or side effects |
| --- | --- | --- | --- | --- |
| build | "ubuntu-latest" | [] | Successful dependencies (default) | actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1; Load pinned stable Rust version; dtolnay/rust-toolchain@683b588c487a69d01f6c6ddf40f036ff682cc2c8; Install compiler cache; Swatinem/rust-cache@d1527c8f67ef9af357290475d4af4b0fab8f12e0; taiki-e/install-action@5488ac5a951645ce95062214902ac4b6bc1a1037; Build and test the user guide; Write sitemap.xml into the rendered guide; actions/configure-pages@45bfe0192ca1faeb007ade9deae92b16b8254a0d; actions/upload-pages-artifact@fc324d3547104276b827a68afc52ff2a11cc49c9 |
| deploy | "ubuntu-latest" | "build" | Successful dependencies (default) | actions/deploy-pages@368f82528645a54fb793d4d04e342629a3f51346 |

### [Build, test, optionally deploy to Pages & publish](publish.yml)

Events: `workflow_dispatch`. Branch/path filters and manual defaults are defined in the linked file.

| Job | Runs on / reuse | Dependencies | Guard | Main checks or side effects |
| --- | --- | --- | --- | --- |
| build | "ubuntu-latest" | [] | Successful dependencies (default) | Basic information; Code checkout; Set up Docker Buildx; Git status and tags; Create output folder; Build Docker image; Build / test the code and the user guide; Build / test the code and the user guide AND PUBLISH to crates.io; Setup Github Pages; Upload artifact |
| pages-deploy | "ubuntu-latest" | "build" | ${{ inputs.deploy_to_gh_pages }} | Deploy to GitHub Pages |

### [Release binaries](release.yml)

Events: `push, workflow_dispatch`. Branch/path filters and manual defaults are defined in the linked file.

| Job | Runs on / reuse | Dependencies | Guard | Main checks or side effects |
| --- | --- | --- | --- | --- |
| build | "${{ matrix.os }}" | [] | Successful dependencies (default) | actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1; Load pinned stable Rust version; dtolnay/rust-toolchain@683b588c487a69d01f6c6ddf40f036ff682cc2c8; Install the configured compiler wrapper; Build; Package; Smoke test the packaged binary; actions/upload-artifact@cf430e030ddbb5b0abf93d22962f4752f3646cd9 |
| release | "ubuntu-latest" | "build" | github.event_name == 'push' && startsWith(github.ref, 'refs/tags/v') | actions/download-artifact@9000827ccba6bdab643e8b6fd33ac0654aef8333; softprops/action-gh-release@efb35369e0ad2afab669f228072c1b0d510eae64 |

### Caches, permissions and failures

- Keep package downloads and compiler outputs separate. Match cache keys to compiler, OS, dependency locks, build profiles and features; a cache is an optimization, not a prerequisite for correctness.
- Matrix jobs start on separate machines. Build/test commands in one job reuse local outputs; moving them to separate jobs requires an explicit compatible artifact handoff.
- Read-only validation and live publication have different lifecycles. Preserve existing credentials, event/ref conditions, environments and non-cancelling deployment groups.
- Timeouts bound hung or unexpectedly expensive runs. Diagnose a timeout before increasing it; never convert a failed check into an unconditional success.
- A queued/disabled/billing-blocked run is not a passed check. Record hosted results separately from local workflow parsing/format validation.

### Local validation

```sh
actionlint
git diff --check
```

Use the repository formatter for edited YAML/Markdown. Run the commands named by the affected
jobs when their executable behavior changes; platform/container/credential-dependent checks
must be reported at the scope actually executed. Do not run publishing commands merely to test
a comment or validation-only change.

<!-- workflow-operations-reference:end -->
