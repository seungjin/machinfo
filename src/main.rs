use clap::Parser;
use serde::Serialize;
use serde_json;
use sysinfo::{Components, Disks, Networks, System};

// Version information
const GIT_HASH: &str = env!("GIT_HASH"); // From build.rs
const GIT_BRANCH: &'static str = env!("GIT_BRANCH");
const GIT_COMMIT_DATE: &'static str = env!("GIT_COMMIT_DATE");
const BUILD_TIME: &str = env!("BUILD_TIMESTAMP");

fn version_string() -> &'static str {
    Box::leak(
        format!(
            "{}(mini hash), {}(branch)\nCommit at {}\n Build at {}",
            GIT_HASH, GIT_BRANCH, GIT_COMMIT_DATE, BUILD_TIME
        )
        .into_boxed_str(),
    )
}

#[derive(Parser)]
#[command(version = version_string())]
struct Cli {}

#[derive(Debug, Serialize)]
struct MachineInfo {
    sys: System,
    disks: Disks,
    networks: Networks,
    components: Components,
}

fn main() {
    let cli = Cli::parse();

    let mut sys = System::new_all();
    sys.refresh_all();

    let disks = Disks::new_with_refreshed_list();
    let networks = Networks::new_with_refreshed_list();
    let components = Components::new_with_refreshed_list();

    let this_machine = MachineInfo {
        sys,
        disks,
        networks,
        components,
    };

    let a = serde_json::to_string(&this_machine);
    println!("{}", a.unwrap());
}
