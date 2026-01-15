# Vibe Executors Standalone Workspace

This workspace extracts the Vibe Kanban executors into a standalone project that can be used as a Node.js native extension.

The published npm package name is `vibe-executor`, which exposes the `JsExecutor` API for running coding agents from Node.js (ESM or CommonJS).

## Layout

- `crates/executors` – N-API wrapper crate that builds the `.node` binary and Node.js entry points.
- `crates/executors-core` – Shared Rust executors implementation reused from the original Vibe Kanban project.
- `crates/utils` – Common Rust utilities used by the executors.

For details on how to install, build, and use the Node package, see the README inside `crates/executors`:

- [executors README](crates/executors/README.md)

## License

The Node.js package and glue code in this workspace are provided under the Apache License, Version 2.0. See the LICENSE file for the full text.

The core executors and utilities are reused from the Vibe Kanban project:

- Vibe Kanban – https://github.com/BloopAI/vibe-kanban

Those components are also licensed under the Apache License, Version 2.0, and additional attribution details are included in the NOTICE file.
