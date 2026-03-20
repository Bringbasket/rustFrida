use std::fs;

use common::{ControllerConfig, InjectionMode, Result};
use native_api::{enumerate_images, InjectionTarget, MachInjector};

use crate::launch::spawn_target;

pub fn run_controller(config: &ControllerConfig, list_images: bool) -> Result<()> {
    if list_images {
        let images = enumerate_images()?;
        for image in images {
            println!("0x{:x} {}", image.slide, image.name);
        }
        return Ok(());
    }

    let pid = match config.mode {
        InjectionMode::Attach => config.pid.expect("validated pid"),
        InjectionMode::Spawn => spawn_target(config.bundle_id.as_deref().expect("validated bundle id"))?,
    };

    let injector = MachInjector;
    injector.inject(&InjectionTarget {
        pid,
        dylib_path: config.agent_path.clone(),
        entry_symbol: config.entry_symbol.clone(),
    })?;

    if let Some(script_path) = &config.script_path {
        let script = fs::read_to_string(script_path)?;
        println!("loaded bootstrap script from {} ({} bytes)", script_path, script.len());
    }

    println!(
        "controller prepared injection plan for pid {} using {}::{}",
        pid, config.agent_path, config.entry_symbol
    );
    Ok(())
}

