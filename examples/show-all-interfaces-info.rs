//! List information of all interfaces
use ethernet_info::get_ethernet_info;

fn main() {
    let ethernet_info_list = get_ethernet_info(None);
    for ethernet_info in ethernet_info_list {
        print!(
            "Interface: {}, Port: {} ",
            ethernet_info.name(),
            ethernet_info.port(),
        );
        print!("Supported: ");
        for supported in ethernet_info.supported() {
            print!("{} ", supported);
        }
        print!("\n");
    }
}
