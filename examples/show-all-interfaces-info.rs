//! List information of all interfaces
use ethernet_info::get_ethernet_info;

fn main() {
    let ethernet_info = get_ethernet_info(None);
    for nic in ethernet_info {
        println!(
            "Interface: {}, Port: {}, Supported: {}",
            nic.name(),
            nic.port(),
            nic.supported().join(", ")
        );
    }
}
