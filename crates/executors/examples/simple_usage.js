import vibeExecutor from "../index.js";

const { JsExecutor } = vibeExecutor;

async function main() {
  console.log("Creating executor...");

  const config = JSON.stringify({
    CLAUDE_CODE: {
      dangerously_skip_permissions: true
    }
  });

  try {
    const executor = JsExecutor.fromConfig(config);
    console.log("Executor created.");

    const cwd = process.cwd();
    const prompt = "Please say hello";

    console.log(`Spawning executor in ${cwd} with prompt: "${prompt}"`);

    const child = await executor.spawn(cwd, prompt, {
      MY_ENV_VAR: "test_value"
    });

    console.log("Child process spawned. Streaming output...");

    child.streamOutput((line) => {
      process.stdout.write(`[Out]: ${line}`);
    });

    const exitCode = await child.wait();
    console.log(`\nExecutor finished with exit code: ${exitCode}`);
  } catch (e) {
    console.error("Error encountered:", e);
    process.exitCode = 1;
  }
}

main().catch((err) => {
  console.error("Unhandled error:", err);
  process.exitCode = 1;
});
