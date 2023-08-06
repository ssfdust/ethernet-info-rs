Ethernet Info
====================
[![crates.io](https://img.shields.io/crates/v/ethernet-info)](https://crates.io/crates/ethernet-info)
[![docs.rs](https://img.shields.io/docsrs/ethernet-info)](https://img.shields.io/docsrs/ethernet-info)
[![ci](https://github.com/ssfdust/ethernet-info-rs/actions/workflows/coverage.yml/badge.svg)](https://github.com/ssfdust/ethernet-info-rs/actions/workflows/coverage.yml)
[![coverage](https://raw.githubusercontent.com/ssfdust/ethernet-info-rs/ci/assets/flat.svg)](https://github.com/ssfdust/ethernet-info-rs/actions/workflows/coverage.yml)

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
