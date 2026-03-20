mod args;
mod injection;
mod launch;

use args::Args;
use clap::Parser;
use common::{ControllerConfig, InjectionMode};
use injection::run_controller;

fn main() {
    let args = Args::parse();
    let mode = if args.spawn { InjectionMode::Spawn } else { InjectionMode::Attach };
    let list_images = args.list_images;
    let config = ControllerConfig {
        mode,
        pid: args.pid,
        bundle_id: args.bundle_id,
        agent_path: args.agent_path,
        entry_symbol: args.entry_symbol,
        script_path: args.script,
        connect_timeout_secs: args.connect_timeout,
    };

    let result = if list_images {
        run_controller(&config, true)
    } else {
        config.validate().and_then(|_| run_controller(&config, false))
    };

    if let Err(err) = result {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
