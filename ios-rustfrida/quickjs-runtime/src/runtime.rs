use std::collections::BTreeSet;

use common::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeStatus {
    Cold,
    Ready,
}

#[derive(Debug, Clone)]
pub struct QuickJsRuntime {
    status: RuntimeStatus,
    builtins: BTreeSet<&'static str>,
    last_script: Option<String>,
}

impl Default for QuickJsRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl QuickJsRuntime {
    pub fn new() -> Self {
        Self {
            status: RuntimeStatus::Cold,
            builtins: BTreeSet::from([
                "console",
                "Module",
                "Memory",
                "Native",
                "ObjC",
                "Interceptor",
                "ptr",
            ]),
            last_script: None,
        }
    }

    pub fn status(&self) -> &RuntimeStatus {
        &self.status
    }

    pub fn initialize(&mut self) -> Result<String> {
        self.status = RuntimeStatus::Ready;
        Ok(self.bootstrap_script())
    }

    pub fn eval(&mut self, script: &str) -> Result<String> {
        if self.status != RuntimeStatus::Ready {
            return Err(Error::State("quickjs runtime is not initialized".into()));
        }
        let trimmed = script.trim();
        if trimmed.is_empty() {
            return Err(Error::InvalidArgument("script is empty".into()));
        }

        self.last_script = Some(trimmed.to_string());

        if trimmed == "ObjC.available" {
            return Ok("true".into());
        }
        if trimmed == "Native.platform" {
            return Ok("ios".into());
        }
        Ok(format!("[quickjs-runtime] accepted {} bytes", trimmed.len()))
    }

    pub fn complete(&self, prefix: &str) -> Vec<String> {
        self.builtins
            .iter()
            .filter(|item| item.starts_with(prefix))
            .map(|item| item.to_string())
            .collect()
    }

    pub fn bootstrap_script(&self) -> String {
        let builtins = self
            .builtins
            .iter()
            .map(|name| format!("\"{name}\""))
            .collect::<Vec<_>>()
            .join(", ");

        format!(
            "globalThis.__iosRustFrida = {{ builtins: [{builtins}] }};\n\
             globalThis.Native = globalThis.Native || {{ platform: 'ios', backend: 'mach' }};\n\
             globalThis.ObjC = globalThis.ObjC || {{ available: true }};\n"
        )
    }
}

