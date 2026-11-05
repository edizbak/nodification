# Project Guidelines

## Project and development environment

- This is a Rust 2021, Leptos 0.8 client-side application compiled to WebAssembly with Trunk.
- Use Podman, not Docker, for local container workflows.
- The Dev Container is the development environment. Its setup installs the `wasm32-unknown-unknown` target and Trunk; it forwards port `8080`.
- Run the site inside the Dev Container with `trunk serve --address 0.0.0.0`. This is the app server, not a second container. Reconnect to the existing Dev Container after a VS Code window reload; do not start another container unless there is a specific need.
- Build with `trunk build --release`.

## Source of truth

- The app entry point and example destinations are in `src/main.rs`; package and dependency configuration is in `Cargo.toml`.
- The README documents the Rust/Leptos development and build commands.

## Multi-step visual changes

- For visual redesigns, first propose a concise sequence of stages and ask the user to approve the next stage before starting it. Wait for explicit approval; do not make changes for a later stage in advance. After each stage, summarize the result and request approval before continuing.
