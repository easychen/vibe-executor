const { JsExecutor } = require('../index.js');
const path = require('path');

async function main() {
    console.log("Creating executor...");
    
    // Example config for ClaudeCode
    // Using a configuration that tries to run 'ls' via bash tool if possible, 
    // or just checking if the agent initializes.
    // Note: This requires the underlying agent (e.g. claude-code) to be installed in the environment 
    // if the executor tries to run it.
    // For 'CLAUDE_CODE', it tries to run 'npx ...'. 
    
    const config = JSON.stringify({
        "CLAUDE_CODE": {
            "dangerously_skip_permissions": true
        }
    });

    try {
        const executor = JsExecutor.fromConfig(config);
        console.log("Executor created.");

        const cwd = process.cwd();
        // A simple prompt. The actual behavior depends on the agent's capability.
        const prompt = "Please say hello";
        
        console.log(`Spawning executor in ${cwd} with prompt: "${prompt}"`);
        
        const child = await executor.spawn(cwd, prompt, {
            "MY_ENV_VAR": "test_value"
        });
        
        console.log("Child process spawned. Streaming output...");

        child.streamOutput((line) => {
            // Raw output might need handling if it contains newlines or specific encoding
            process.stdout.write(`[Out]: ${line}`);
        });

        // In a real scenario, you might want to handle interrupts or timeouts.
        // For this example, we just wait.
        const exitCode = await child.wait();
        console.log(`\nExecutor finished with exit code: ${exitCode}`);

    } catch (e) {
        console.error("Error encountered:", e);
    }
}

if (require.main === module) {
    main();
}
