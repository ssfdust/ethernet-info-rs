//! This module contains helper functions for parsering the ethtool command.
use std::process::Command;
use nix::net::if_::if_nameindex;

/// This struct contains the parsed output of the ethtool command.
#[derive(Debug)]
pub struct EthtoolOutput {
    pub interface: String,
    pub port: Option<String>,
    pub supported_ports: Vec<String>,
    pub supported_link_modes: Vec<String>,
}

impl TryFrom<&str> for EthtoolOutput {
    type Error = String;
    fn try_from(interface: &str) -> Result<Self, Self::Error> {
        let port;
        // Get the output of the ethtool command, and conbine the lines that are
        // split by the newline character.
        let output = get_ethtool_output(interface);
        // Get the supported link modes and ports.
        let supported_link_modes = parse_ethtool_output(
            "Supported link modes",
            output.iter().map(|s| s.as_str()).collect(),
        );
        // Get the supported ports. We need to filter out the "[" and "]" characters.
        let supported_ports = parse_ethtool_output(
            "Supported ports",
            output.iter().map(|s| s.as_str()).collect(),
        )
        .into_iter()
        .filter_map(|part| {
            if part == "[" || part == "]" {
                return None;
            } else {
                return Some(part);
            }
        })
        .collect::<Vec<String>>();
        // Get the port. If the port is not reported, we set it to None.
        let port_vec = parse_ethtool_output("Port", output.iter().map(|s| s.as_str()).collect());
        port = if port_vec.is_empty() {
            None
        } else {
            Some(port_vec.join(" "))
        };
        Ok(Self {
            interface: interface.to_string(),
            port,
            supported_ports,
            supported_link_modes,
        })
    }
}

/// Returns the output of the ethtool command.
/// The output is a vector of strings, each string is a line of the output.
/// We conbine the lines that are split by the newline character, which doesn'the
/// contain the colon character.
fn get_ethtool_output(interface: &str) -> Vec<String> {
    let mut line_vec: Vec<String> = vec![];
    let output = Command::new("ethtool")
        .arg(interface)
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8(output.stdout).unwrap();
    for line in output.lines() {
        let line = line.replace("\t", " ");
        if line.contains(": ") {
            if line.contains("Not reported") {
                continue;
            }
            line_vec.push(line.trim().into());
        } else {
            if let Some(last) = line_vec.last() {
                let mut last: String = last.clone();
                last.push_str(" ");
                last.push_str(line.trim());
                line_vec.pop();
                line_vec.push(last);
            }
        }
    }
    line_vec
}

/// Make the output gathered by get_ethernet_info() more structured.
/// We filter the output by the keyword, and then split the line by the colon.
/// Get the second part of the line, which is the information we need.
fn parse_ethtool_output(keyword: &str, lines: Vec<&str>) -> Vec<String> {
    let mut info_list = Vec::new();
    for line in lines {
        if line.contains(keyword) {
            let mut port_vec: Vec<&str> = line.split(": ").collect();
            port_vec.remove(0);
            for p in port_vec[0].split(' ') {
                if !p.trim().is_empty() {
                    info_list.push(p.into());
                }
            }
        }
    }
    if keyword != "Port" {
        info_list.sort();
    }
    info_list
}

/// List all the interfaces that are available on the system.
/// For each interface, we parse the output of the ethtool command.
pub fn list_all_interface_ethtool_output() -> Vec<EthtoolOutput> {
    let mut output_vec = Vec::new();
    for interface in if_nameindex().unwrap().iter() {
        let interface = interface.name().to_str().unwrap();
        if let Ok(output) = EthtoolOutput::try_from(interface) {
            output_vec.push(output);
        }
    }
    output_vec
}
