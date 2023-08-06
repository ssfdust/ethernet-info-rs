Ethernet Info
====================
The crate provides a way to get the link information of the interface, including
the port type, supported modes.
The crate is based on the ioctl command, so it can only be used on Linux.

Examples
---------------------
List all the interfaces' ethtool related information.
```rust
use ethernet_info::get_ethernet_info;
let interfaces_eth_info = get_ethernet_info(None);
for interface_info in interfaces_eth_info {
    println!("interface: {}", interface_info.name());
    println!("Port: {}", interface_info.port());
    println!("Supported: {:?}", interface_info.supported());
}
```

Get the ethtool related information of the specified interface.
```rust
use ethernet_info::get_ethernet_info;
let interfaces_eth_info = get_ethernet_info(Some("eth0"));
for interface_info in interfaces_eth_info {
    println!("interface: {}", interface_info.name());
    println!("Port: {}", interface_info.port());
    println!("Supported: {:?}", interface_info.supported());
}
```

Get the ethtool related of the specified interface by EthernetInfo.
```rust
use ethernet_info::EthernetInfo;
if let Ok(interface_info) = EthernetInfo::try_from("eth0") {
    println!("interface: {}", interface_info.name());
    println!("Port: {}", interface_info.port());
    println!("Supported: {:?}", interface_info.supported());
}
```
