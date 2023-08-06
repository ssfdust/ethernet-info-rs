//! Get interface information by given interface name
use ethernet_info::get_ethernet_info;

fn main() {
    let interface = std::env::args().nth(1);
    let ethernet_info_list = get_ethernet_info(interface.as_deref());
    for ethernet_info in ethernet_info_list {
        println!(
            "Interface: {}, Port: {}, Supported: {:?}",
            ethernet_info.name(),
            ethernet_info.port(),
            ethernet_info.supported()
        );
    }
}
