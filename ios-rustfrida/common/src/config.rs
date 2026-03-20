#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InjectionMode {
    Attach,
    Spawn,
}

#[derive(Debug, Clone)]
pub struct ControllerConfig {
    pub mode: InjectionMode,
    pub pid: Option<i32>,
    pub bundle_id: Option<String>,
    pub agent_path: String,
    pub entry_symbol: String,
    pub script_path: Option<String>,
    pub connect_timeout_secs: u64,
}

impl ControllerConfig {
    pub fn validate(&self) -> crate::Result<()> {
        match self.mode {
            InjectionMode::Attach if self.pid.is_none() => {
                Err(crate::Error::InvalidArgument("attach mode requires --pid".into()))
            }
            InjectionMode::Spawn if self.bundle_id.is_none() => {
                Err(crate::Error::InvalidArgument("spawn mode requires --bundle-id".into()))
            }
            _ => Ok(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BootstrapConfig {
    pub entry_symbol: String,
    pub socket_name: String,
    pub dylib_path: String,
}

impl Default for BootstrapConfig {
    fn default() -> Self {
        Self {
            entry_symbol: "ios_agent_entry".into(),
            socket_name: "ios-rustfrida.sock".into(),
            dylib_path: "/usr/lib/agent.dylib".into(),
        }
    }
}

