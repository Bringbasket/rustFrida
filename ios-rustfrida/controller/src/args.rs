use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about = "iOS jailbreak edition of rustFrida controller")]
pub struct Args {
    #[arg(long, value_name = "PID")]
    pub pid: Option<i32>,

    #[arg(long, value_name = "BUNDLE_ID")]
    pub bundle_id: Option<String>,

    #[arg(long, requires = "bundle_id")]
    pub spawn: bool,

    #[arg(long, value_name = "FILE")]
    pub script: Option<String>,

    #[arg(long)]
    pub list_images: bool,

    #[arg(long, default_value = "/usr/lib/agent.dylib")]
    pub agent_path: String,

    #[arg(long, default_value = "ios_agent_entry")]
    pub entry_symbol: String,

    #[arg(long, default_value_t = 15)]
    pub connect_timeout: u64,
}
