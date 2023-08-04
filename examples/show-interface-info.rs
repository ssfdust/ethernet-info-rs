//! Get interface information by given interface name
use ethernet_info::get_ethernet_info;

fn main() {
    let interface = std::env::args().nth(1);
    let ethernet_info = get_ethernet_info(interface.as_deref());
    for nic in ethernet_info {
        println!(
            "Interface: {}, Port: {}, Supported: {}",
            nic.name(),
            nic.port(),
            nic.supported().join(", ")
        );
    }
}
