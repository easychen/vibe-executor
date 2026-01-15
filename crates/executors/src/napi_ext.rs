use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use futures::StreamExt;
use napi::bindgen_prelude::*;
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode};
use napi_derive::napi;
use tokio::sync::Mutex;

use executors_core::env::ExecutionEnv;
use executors_core::executors::{CodingAgent, SpawnedChild, StandardCodingAgentExecutor};

#[napi]
pub struct JsExecutor {
  inner: CodingAgent,
}

#[napi]
impl JsExecutor {
  #[napi(factory)]
  pub fn from_config(config_json: String) -> Result<JsExecutor> {
    let agent: CodingAgent = serde_json::from_str(&config_json)
      .map_err(|e| Error::new(Status::InvalidArg, format!("Failed to parse config: {}", e)))?;
    Ok(JsExecutor { inner: agent })
  }

  #[napi]
  pub async fn spawn(
    &self,
    cwd: String,
    prompt: String,
    env: Option<HashMap<String, String>>,
  ) -> Result<JsChild> {
    let mut execution_env = ExecutionEnv::new();
    if let Some(env_map) = env {
      execution_env = execution_env.with_overrides(&env_map);
    }

    let child = self
      .inner
      .spawn(&PathBuf::from(cwd), &prompt, &execution_env)
      .await
      .map_err(|e| {
        Error::new(
          Status::GenericFailure,
          format!("Failed to spawn executor: {}", e),
        )
      })?;

    Ok(JsChild {
      inner: Arc::new(Mutex::new(Some(child))),
    })
  }
}

#[napi]
pub struct JsChild {
  inner: Arc<Mutex<Option<SpawnedChild>>>,
}

#[napi]
impl JsChild {
  #[napi]
  pub fn stream_output(
    &self,
    callback: ThreadsafeFunction<String, ErrorStrategy::Fatal>,
  ) -> Result<()> {
    let inner = self.inner.clone();
    
    // Spawn a task to setup the stream so we don't block the JS thread
    tokio::spawn(async move {
        let mut guard = inner.lock().await;
        if let Some(child) = guard.as_mut() {
            match executors_core::stdout_dup::duplicate_stdout(&mut child.child) {
                Ok(mut stream) => {
                    // Release the lock so other operations (like wait/kill) can proceed
                    drop(guard);
                    
                    while let Some(result) = stream.next().await {
                        if let Ok(line) = result {
                            callback.call(line, ThreadsafeFunctionCallMode::NonBlocking);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error duplicating stdout: {}", e);
                }
            }
        }
    });

    Ok(())
  }

  #[napi]
  pub async fn wait(&self) -> Result<i32> {
    let inner = self.inner.clone();
    let mut guard = inner.lock().await;
    
    if let Some(child) = guard.as_mut() {
       let status = child.child.wait().await.map_err(|e| {
           Error::new(Status::GenericFailure, format!("Failed to wait: {}", e))
       })?;
       
       Ok(status.code().unwrap_or(-1))
    } else {
       Err(Error::new(Status::GenericFailure, "Child process not available".to_string()))
    }
  }

  #[napi]
  pub async fn kill(&self) -> Result<()> {
    let inner = self.inner.clone();
    let mut guard = inner.lock().await;
    
    if let Some(child) = guard.as_mut() {
       child.child.start_kill().map_err(|e| {
           Error::new(Status::GenericFailure, format!("Failed to kill: {}", e))
       })?;
       Ok(())
    } else {
       Err(Error::new(Status::GenericFailure, "Child process not available".to_string()))
    }
  }
}
