use serde::Serialize;
use serde_json;
use sysinfo::{Components, Disks, Networks, System};

#[derive(Debug, Serialize)]
struct MachineInfo {
    sys: System,
    disks: Disks,
    networks: Networks,
    components: Components,
}

fn main() {
    println!("Git Hash: {}", env!("GIT_HASH"));
    println!("Git Branch: {}", env!("GIT_BRANCH"));
    println!("Commit Date: {}", env!("GIT_COMMIT_DATE"));

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
