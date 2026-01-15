import vibeExecutor from "../index.js";

const { JsExecutor } = vibeExecutor;

async function main() {
  console.log("Creating Amp executor...");

  const config = JSON.stringify({
    AMP: {
      dangerously_allow_all: true
    }
  });

  const executor = JsExecutor.fromConfig(config);
  console.log("Executor created.");

  const prompt = "Please say hello from Amp.";

  const child = await executor.spawn(process.cwd(), prompt, {});

  console.log("Child process spawned. Streaming output...");

  child.streamOutput((line) => {
    process.stdout.write(`[Out]: ${line}`);
  });

  const exitCode = await child.wait();
  console.log(`\nExecutor finished with code: ${exitCode}`);
}

main().catch((err) => {
  console.error("Error encountered:", err);
  process.exitCode = 1;
});
