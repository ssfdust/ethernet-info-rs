Nic Port Info
---------------------
The crate provides a way to get the link information of the interface, including
the port type, supported modes.
The crate is based on the ioctl command, so it can only be used on Linux.

# Examples
List all the interfaces' ethtool related information.
```
use ethernet_info::get_ethernet_info;
let interfaces_eth_info = get_ethernet_info(None);
for interface_info in interfaces_eth_info {
    println!("interface: {}", interface_info.name());
    println!("Port: {}", interface_info.port());
    println!("Supported: {:?}", interface_info.supported());
}
```

Get the ethtool related information of the specified interface.
```
use ethernet_info::get_ethernet_info;
let interfaces_eth_info = get_ethernet_info(Some("eth0"));
for interface_info in interfaces_eth_info {
    println!("interface: {}", interface_info.name());
    println!("Port: {}", interface_info.port());
    println!("Supported: {:?}", interface_info.supported());
}
```

Get the ethtool related of the specified interface by EthernetInfo.
```
use ethernet_info::EthernetInfo;
let interface_info = EthernetInfo::from_name("eth0").unwrap();
println!("interface: {}", interface_info.name());
println!("Port: {}", interface_info.port());
println!("Supported: {:?}", interface_info.supported());
```
