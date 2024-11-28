use clap::Parser;
use serde::Serialize;
use serde_json;
use sysinfo::{Components, Disks, Networks, System};

const fn version_string() -> &'static str {
    concat!(
        env!("GIT_HASH"),
        "(mini hash), ",
        env!("GIT_BRANCH"),
        "(branch)\nCommit at ",
        env!("GIT_COMMIT_DATE"),
        "\nBuild at ",
        env!("BUILD_TIMESTAMP")
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
    let _ = Cli::parse();

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
