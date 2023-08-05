/// Test EthernetInfo struct
///
/// The tests are based on the output of ethtool <interface>
/// We test the following fields, interface name, port type, supported ports
/// , supported modes, etc.
mod utils;
use ethernet_info::EthernetInfo;
use utils::ethtool_parser::list_all_interface_ethtool_output;

#[test]
fn test_port_type() {
    // we need to find at least one interface that has port type
    let mut found = false;
    for ethtool_output in list_all_interface_ethtool_output() {
        if ethtool_output.port.is_none() {
            continue;
        }
        let ethernet_info = EthernetInfo::from_name(&ethtool_output.interface).unwrap();
        let port_type = ethernet_info.port().to_string();
        assert_eq!(ethtool_output.port.unwrap(), port_type);
        found = true;
    }
    assert!(found);
}

#[test]
fn test_supported_ports() {
    // we need to find at least one interface that has supported ports
    let mut found = false;
    for ethtool_output in list_all_interface_ethtool_output() {
        if ethtool_output.port.is_none() {
            continue;
        }
        let ethernet_info = EthernetInfo::from_name(&ethtool_output.interface).unwrap();
        let mut supported_ports = ethernet_info.ports().iter().map(|x| x.to_string()).collect::<Vec<String>>();
        supported_ports.sort();
        if ethtool_output.supported_ports.len() > 0 {
            found = true;
            assert_eq!(ethtool_output.supported_ports, supported_ports);
        }
    }
    assert!(found);
}

#[test]
fn test_supported_link_modes() {
    // we need to find at least one interface that has supported link modes
    let mut found = false;
    for ethtool_output in list_all_interface_ethtool_output() {
        if ethtool_output.port.is_none() {
            continue;
        }
        let ethernet_info = EthernetInfo::from_name(&ethtool_output.interface).unwrap();
        let mut supported_link_modes = ethernet_info.supported().iter().map(|x| x.to_string()).collect::<Vec<String>>();
        supported_link_modes.sort();
        if ethtool_output.supported_link_modes.len() > 0 {
            found = true;
            assert_eq!(ethtool_output.supported_link_modes, supported_link_modes);
        }
    }
    assert!(found);
}
