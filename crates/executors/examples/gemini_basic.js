import vibeExecutor from "../index.js";

const { JsExecutor } = vibeExecutor;

async function main() {
  console.log("Creating Gemini executor...");

  const config = JSON.stringify({
    GEMINI: {
      model: "gemini-2.0-flash",
      yolo: true
    }
  });

  const executor = JsExecutor.fromConfig(config);
  console.log("Executor created.");

  const prompt = "Please say hello from Gemini.";

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
