/// Test EthernetInfo struct
///
/// The tests are based on the output of ethtool <interface>
/// We test the following fields, interface name, port type, supported ports
/// , supported modes, etc.
mod utils;
use ethernet_info::{get_ethernet_info, EthernetInfo};
use utils::ethtool_parser::list_all_interface_ethtool_output;

#[test]
fn test_port_type() {
    // we need to find at least one interface that has port type
    let mut found = false;
    for ethtool_output in list_all_interface_ethtool_output() {
        if ethtool_output.port.is_none() {
            continue;
        }
        let ethernet_info = EthernetInfo::try_from(ethtool_output.interface.as_ref()).unwrap();
        let port_type = ethernet_info.port().to_string();
        assert_eq!(ethtool_output.port.unwrap(), port_type);
        found = true;
    }
    assert!(found);
}

#[test]
fn test_advertised_ports() {
    // we need to find at least one interface that has supported ports
    let mut found = false;
    for ethtool_output in list_all_interface_ethtool_output() {
        if ethtool_output.port.is_none() {
            continue;
        }
        let ethernet_info = EthernetInfo::try_from(ethtool_output.interface.as_ref()).unwrap();
        let mut advertised_link_modes = ethernet_info
            .advertised()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        advertised_link_modes.sort();
        if ethtool_output.advertised_link_modes.len() > 0 {
            found = true;
            assert_eq!(ethtool_output.advertised_link_modes, advertised_link_modes);
        }
    }
    if !std::env::var("NO_SUPPORTED_PORTS").is_ok() {
        assert!(found);
    }
}

#[test]
fn test_supported_ports() {
    // we need to find at least one interface that has supported ports
    let mut found = false;
    for ethtool_output in list_all_interface_ethtool_output() {
        if ethtool_output.port.is_none() {
            continue;
        }
        let ethernet_info = EthernetInfo::try_from(ethtool_output.interface.as_ref()).unwrap();
        let mut supported_ports = ethernet_info
            .ports()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        supported_ports.sort();
        dbg!(&ethtool_output.supported_ports);
        dbg!(supported_ports.clone());
        if ethtool_output.supported_ports.len() > 0 {
            found = true;
            assert_eq!(ethtool_output.supported_ports, supported_ports);
        }
    }
    if !std::env::var("NO_SUPPORTED_PORTS").is_ok() {
        assert!(found);
    }
}

#[test]
fn test_supported_link_modes() {
    // we need to find at least one interface that has supported link modes
    let mut found = false;
    for ethtool_output in list_all_interface_ethtool_output() {
        if ethtool_output.port.is_none() {
            continue;
        }
        let ethernet_info = EthernetInfo::try_from(ethtool_output.interface.as_ref()).unwrap();
        let mut supported_link_modes = ethernet_info
            .supported()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        supported_link_modes.sort();
        if ethtool_output.supported_link_modes.len() > 0 {
            found = true;
            assert_eq!(ethtool_output.supported_link_modes, supported_link_modes);
        }
    }
    assert!(found);
}

#[test]
fn test_get_ethernet_info_just_type_check() {
    let mut found = false;
    for ethtool_output in list_all_interface_ethtool_output() {
        if ethtool_output.port.is_none() {
            continue;
        }
        let ethernet_info = get_ethernet_info(Some(&ethtool_output.interface));
        let ethernet_info = ethernet_info.get(0).unwrap();
        let port_type = ethernet_info.port().to_string();
        assert_eq!(ethtool_output.port.unwrap(), port_type);
        found = true;
    }
    assert!(found);
}

#[test]
fn test_get_all_ethernet() {
    let ifacenames: Vec<String> = list_all_interface_ethtool_output()
        .iter()
        .filter(|x| x.port.is_some())
        .map(|x| x.interface.clone())
        .collect();
    let ethernet_info_list = get_ethernet_info(None);
    for ethernet_info in ethernet_info_list {
        let ethernet_info = ethernet_info.clone();
        assert!(ifacenames.contains(&ethernet_info.name().to_string()));
    }
}
