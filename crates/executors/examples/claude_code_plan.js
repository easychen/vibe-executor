const { JsExecutor } = require("../index.js");

async function main() {
  console.log("Creating Claude Code executor with plan mode...");

  const config = JSON.stringify({
    CLAUDE_CODE: {
      plan: true,
      approvals: false,
      dangerously_skip_permissions: true,
      model: "claude-sonnet-4-5-20250929"
    }
  });

  const executor = JsExecutor.fromConfig(config);
  console.log("Executor created.");

  const prompt =
    "Plan a sequence of steps to list files in the current directory.";

  const child = await executor.spawn(process.cwd(), prompt, {});

  console.log("Child process spawned. Streaming output...");

  child.streamOutput((line) => {
    process.stdout.write(`[Out]: ${line}`);
  });

  const exitCode = await child.wait();
  console.log(`\nExecutor finished with code: ${exitCode}`);
}

if (require.main === module) {
  main().catch((err) => {
    console.error("Error encountered:", err);
    process.exitCode = 1;
  });
}

