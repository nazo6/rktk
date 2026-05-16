# Project Overview: rktk (Rust Keyboard Toolkit)

## Purpose
`rktk` is a modern keyboard firmware framework built with Rust. It aims to provide a flexible, modular, and high-performance foundation for creating custom keyboard firmware, leveraging the safety and concurrency features of Rust (specifically `embassy` for async embedded development).

## Communication Guidelines
- **Project Language**: All code, documentation, and commit messages must be in **English**.
- **Interaction**: AI agents and contributors should respond in the language used by the user when communicating in chat or PR comments.

## Architecture

The project follows a modular architecture to separate core logic from hardware-specific implementations.

### 1. Core Framework (`rktk`)
The central engine that manages keyboard states, event routing, and coordination between drivers and the key mapping system.

### 2. Key Mapping State Machine (`kmsm`)
An independent crate responsible for handling complex key behaviors like Tap-Hold, Tap Dance, and Layers. It is designed to be platform-agnostic and does not depend on the main `rktk` crate or `embassy`.

### 3. Drivers
Located in `crates/rktk-drivers-*`, these provide the interface between the core framework and the underlying hardware (e.g., USB, Bluetooth, Keyscan, Display).
- `rktk-drivers-common`: Shared driver logic for all platforms.
- `rktk-drivers-nrf`: Hardware-specific drivers for Nordic nRF52 series.
- `rktk-drivers-rp`: Hardware-specific drivers for Raspberry Pi RP2040.

### 4. Remote Protocol (`rktk-rrp` & `rktk-client`)
- `rktk-rrp`: Defines a communication protocol for remote configuration.
- `rktk-client`: A Dioxus-based GUI (remapper) for real-time keyboard configuration.

### 5. Xtask (`xtask`)
A custom automation tool (located in `crates/xtask`) used for project management tasks. It abstracts complex cargo commands and provides a unified interface for:
- `cargo xtask check all`: Validates the entire workspace including different feature combinations.
- `cargo xtask test all`: Runs all tests.
- `cargo xtask doc`: Generates API documentation.

## Directory Structure

```text
.
├── crates/             # Core library crates
│   ├── rktk/           # Main framework core
│   ├── kmsm/           # Key mapping logic (platform-agnostic)
│   ├── rktk-rrp/       # Remote protocol definition
│   ├── rktk-drivers-*/ # Hardware-specific drivers
│   ├── rktk-client/    # Dioxus-based remapper client
│   └── xtask/          # Automation scripts and management tools
├── keyboards/          # Keyboard-specific implementations
│   └── <name>/         # Each keyboard directory (e.g., corne, keyball61)
│       └── crates/     # MCU-specific firmware crates (e.g., <kb>-nrf, <kb>-rp)
├── site/               # Documentation and marketing website (Next.js/Pnpm)
├── Cargo.toml          # Workspace configuration
└── mise.toml           # Development tool and task configuration
```

## Development Workflow

The project uses `mise` for environment and task management. The common workflow involves the following tasks defined in `mise.toml`:

### Common Tasks
- **Check All**: `mise run check-all` (runs `cargo xtask check all`)
  Verifies the entire workspace for compilation errors.
- **Test All**: `mise run test-all` (runs `cargo xtask test all`)
  Runs tests across all crates.
- **Remapper Development**: `mise run rktk-client-start`
  Starts the `rktk-client` development server (requires `pnpm`).
- **Site Development**: `mise run site-dev`
  Starts the Next.js development server for the documentation site (requires `pnpm`).
- **Documentation Generation**: `mise run doc-gen`
  Generates documentation using the `xtask` tool.

### Local Development Requirements
- **Rust Nightly**: Required for binary size optimization (crucial for embedded targets) and cargo unstable features used in the workspace.
- **Mise**: For task management and environment setup.
- **Pnpm**: For frontend development (`rktk-client`, `site`).
- **Cargo-hack**: Used by `xtask` for feature matrix checking.
