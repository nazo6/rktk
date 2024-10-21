# ra-check

In default config, rust-analyzer executes `cargo check --workspace` but it
generate poor error because there are multiple mcu projects in this workspace,
and it enables exclusive features simultaneously (if you try run
`cargo check --workspace` in this workspace, you will see a lot of errors).

So, this crate wraps `cargo clippy` and by executing it in crate directory (not
workspace directory), provides proper error messages. This can be achieved using
`rust-analyzer.check.overrideCommand` config. See `.vscode/settings.json` for
detail.

This crate is separated from `rktk-cli` crate because if this feature is in
`rktk-cli` crate, diagnostics does not work for `rktk-cli` crate itself!
