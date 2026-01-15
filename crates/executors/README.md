# Vibe Kanban Executors - Node.js Native Extension

This package provides Node.js bindings for the Vibe Kanban executors, allowing you to run AI coding agents (Claude Code, Amp, Gemini, etc.) directly from Node.js applications.

The Rust core logic is reused from the main `vibe-kanban` repository (`crates/executors`), and this package only adds a thin N-API layer on top.

## Installation

Install from npm:

```bash
npm install vibe-executor
# or
pnpm add vibe-executor
```

For local development in this repo:

```bash
cd crates/executors
npm install
```

## Building

To build the native extension in this repo:

```bash
cd crates/executors
npm run build
```

or with pnpm:

```bash
cd crates/executors
pnpm install
pnpm run build
```

This will compile the Rust code (`vibe-executor-napi` crate), link it against the shared `executors` crate from `ref/vibe-kanban-main`, and generate:

- the platform-specific `.node` binary
- the `index.js` loader
- the TypeScript declarations `index.d.ts`

## Runtime API

The N-API surface is intentionally small:

- `JsExecutor.fromConfig(configJson: string): JsExecutor`
  - Creates a new executor instance from a JSON configuration string.
- `JsExecutor.spawn(cwd: string, prompt: string, env?: Record<string, string>): Promise<JsChild>`
  - Spawns the underlying coding agent process.
- `JsChild.streamOutput(callback: (data: string) => void): void`
  - Registers a callback to receive stdout data (newline-delimited JSON).
- `JsChild.wait(): Promise<number>`
  - Waits for the process to exit and returns the exit code.
- `JsChild.kill(): Promise<void>`
  - Sends a kill signal to the process.

The JSON lines you receive from `streamOutput` are the same structured events used inside Vibe Kanban (system events, stream events, tool calls, final result, etc.).

## Configuration format

`fromConfig` expects a JSON string describing which executor to use and how it should behave. The structure matches the Rust `CodingAgent` / `BaseCodingAgent` types from the main repo.

At the top level, the object is keyed by executor name:

- `CLAUDE_CODE`
- `AMP`
- `GEMINI`
- (and others available in the Rust `BaseCodingAgent` enum)

Each key maps to an object with executor-specific options.

### Claude Code example

```javascript
const config = JSON.stringify({
  CLAUDE_CODE: {
    dangerously_skip_permissions: true,
    plan: false,
    approvals: false,
    model: "claude-sonnet-4-5-20250929"
  }
});
```

### Amp example

```javascript
const config = JSON.stringify({
  AMP: {
    dangerously_allow_all: true
  }
});
```

### Gemini example

```javascript
const config = JSON.stringify({
  GEMINI: {
    model: "gemini-2.0-flash",
    yolo: true
  }
});
```

You can also override the base command, parameters, and environment via the shared `cmd` field (mapped to `CmdOverrides` in Rust):

```javascript
const config = JSON.stringify({
  CLAUDE_CODE: {
    dangerously_skip_permissions: true,
    cmd: {
      base_command_override: "npx -y @anthropic-ai/claude-code@latest",
      additional_params: ["--verbose"],
      env: {
        CLAUDE_API_KEY: process.env.CLAUDE_API_KEY || ""
      }
    }
  }
});
```

## Basic usage (ESM)

### Single executor (Claude Code)

```javascript
import vibeExecutor from "vibe-executor";

const { JsExecutor } = vibeExecutor;

async function main() {
  const config = JSON.stringify({
    CLAUDE_CODE: {
      dangerously_skip_permissions: true
    }
  });

  const executor = JsExecutor.fromConfig(config);

  const child = await executor.spawn(
    process.cwd(),
    "Please say hello",
    {}
  );

  child.streamOutput((line) => {
    process.stdout.write(`[Out]: ${line}`);
  });

  const exitCode = await child.wait();
  console.log(`\nExecutor finished with code: ${exitCode}`);
}

main().catch((err) => {
  console.error("Executor error:", err);
  process.exitCode = 1;
});
```

## Executor-specific examples

There are several ready-to-run examples under `examples/`:

- `examples/simple_usage.js` – minimal CLAUDE_CODE example.
- `examples/claude_code_plan.js` – Claude Code with planning mode.
- `examples/amp_basic.js` – Amp executor with streaming logs.
- `examples/gemini_basic.js` – Gemini executor using the ACP harness.

You can run them from this directory:

```bash
node examples/simple_usage.js
node examples/claude_code_plan.js
node examples/amp_basic.js
node examples/gemini_basic.js
```

Make sure the corresponding CLI tools and API keys (Claude, Amp, Gemini) are installed and configured in your environment, as required by each executor.

## Apple Silicon Mac (M1/M2/M3)

If you are developing or running this on an Apple Silicon Mac, you should ensure you are targeting `aarch64-apple-darwin`.

1. Install the Rust target:

   ```bash
   rustup target add aarch64-apple-darwin
   ```

2. Verify your Node.js architecture:

   ```bash
   node -p "process.arch"
   # Should output 'arm64'
   ```

   If it outputs `x64`, you are running under Rosetta. Prefer a native arm64 Node.js to match the Rust target.

3. Build for the explicit target if needed:

   ```bash
   npm run build -- --target aarch64-apple-darwin
   ```

## Development

- `npm run build:debug` – Build in debug mode (faster compilation).
- `npm test` – Run the simple example script (`examples/simple_usage.js`).

## License and attribution

This npm package (`vibe-executor`) and its Node.js bindings are provided under the Apache License, Version 2.0. See the LICENSE file at the repository root for the full text.

The underlying executors implementation is reused from the Vibe Kanban project:

- Vibe Kanban – https://github.com/BloopAI/vibe-kanban

Those components are licensed under the Apache License, Version 2.0. Additional attribution details are included in the NOTICE file at the repository root.
