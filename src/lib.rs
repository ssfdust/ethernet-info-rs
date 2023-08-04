//! The crate provides a way to get the link information of the interface, including
//! the port type, supported modes.
//! The crate is based on the ioctl command, so it can only be used on Linux.
//!
//! # Examples
//! List all the interfaces' ethtool related information.
//! ```
//! use ethernet_info::get_ethernet_info;
//! let interfaces_eth_info = get_ethernet_info(None);
//! for interface_info in interfaces_eth_info {
//!     println!("interface: {}", interface_info.name());
//!     println!("Port: {}", interface_info.port());
//!     println!("Supported: {:?}", interface_info.supported());
//! }
//! ```
//!
//! Get the ethtool related information of the specified interface.
//! ```
//! use ethernet_info::get_ethernet_info;
//! let interfaces_eth_info = get_ethernet_info(Some("eth0"));
//! for interface_info in interfaces_eth_info {
//!     println!("interface: {}", interface_info.name());
//!     println!("Port: {}", interface_info.port());
//!     println!("Supported: {:?}", interface_info.supported());
//! }
//! ```
//!
//! Get the ethtool related of the specified interface by EthernetInfo.
//! ```
//! use ethernet_info::EthernetInfo;
//! let interface_info = EthernetInfo::from_name("eth0").unwrap();
//! println!("interface: {}", interface_info.name());
//! println!("Port: {}", interface_info.port());
//! println!("Supported: {:?}", interface_info.supported());
//! ```
#![allow(non_upper_case_globals)]

#[macro_use]
extern crate nix;

mod errors;
mod ethtool_const;
mod internal;
mod settings_parser;

use crate::ethtool_const::*;
use crate::settings_parser::SettingsParser;
use internal::{CmdContext, EthtoolCommnad};

pub use errors::EthtoolError;

/// The port information includes the port type, supported modes.
#[derive(Default, Debug, Clone)]
pub struct EthernetInfo {
    name: String,
    port: String,
    supported: Vec<String>,
}

impl EthernetInfo {
    /// Create a EthernetInfo from the SettingsParser
    pub fn from_settings_parser(name: &str, settings_parser: SettingsParser) -> Self {
        let supported = settings_parser.supported_link_modes();
        EthernetInfo {
            name: name.to_string(),
            port: settings_parser.port(),
            supported,
        }
    }

    /// Get the name of the interface
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the port type of the interface
    pub fn port(&self) -> &str {
        &self.port
    }

    /// Get the supported modes of the interface
    pub fn supported(&self) -> &Vec<String> {
        &self.supported
    }

    /// Create a EthernetInfo from the interface name
    pub fn from_name(name: &str) -> Result<Self, EthtoolError> {
        let ctx = CmdContext::new(name)?;
        let port = do_ioctl_get_ethernet_info(ctx)?;
        Ok(port)
    }
}


/// Use ioctl to get the port information of the interface
///
/// The main steps are defined in do_ioctl_ethtool_glinksettings() in ethtool.c
fn do_ioctl_get_ethernet_info(mut ctx: CmdContext) -> Result<EthernetInfo, EthtoolError> {
    let mut ecmd = EthtoolCommnad::new(ETHTOOL_GLINKSETTINGS)?;
    /* Handshake with kernel to determine number of words for link
     * mode bitmaps. When requested number of bitmap words is not
     * the one expected by kernel, the latter returns the integer
     * opposite of what it is expecting. We request length 0 below
     * (aka. invalid bitmap length) to get this info.
     */
    ctx = ctx.send_ioctl(ecmd)?;
    ecmd = ctx.get_ethtool_link_settings();
    if ecmd.req.link_mode_masks_nwords >= 0 || ecmd.req.cmd != ETHTOOL_GLINKSETTINGS {
        return Err(EthtoolError::new(
            "Failed to determine number of words for link mode bitmaps",
        ));
    }

    /* got the real ecmd.req.link_mode_masks_nwords,
     * now send the real request
     */
    ecmd.req.cmd = ETHTOOL_GLINKSETTINGS;
    ecmd.req.link_mode_masks_nwords = -ecmd.req.link_mode_masks_nwords;
    ctx = ctx.send_ioctl(ecmd)?;
    ecmd = ctx.get_ethtool_link_settings();

    /* check the link_mode_masks_nwords again */
    if ecmd.req.link_mode_masks_nwords <= 0 || ecmd.req.cmd != ETHTOOL_GLINKSETTINGS {
        return Err(EthtoolError::new(
            "Failed to check the link_mode_masks_nwords.",
        ));
    }

    ctx.close_socket();

    Ok(ecmd.into_ethernet_info(ctx.ifname()))
}


/// Get the ethtool related information of the interface
/// If devname is None, get all the interfaces' ethtool related information.
/// If devname is Some(&str), get the specified interface's ethtool related information.
pub fn get_ethernet_info(devname: Option<&str>) -> Vec<EthernetInfo> {
    let mut ethernet_info_vec = Vec::new();
    if let Some(devname) = devname {
        let ethernet_info = EthernetInfo::from_name(devname);
        if let Ok(ethernet_info) = ethernet_info {
            ethernet_info_vec.push(ethernet_info);
        }
    } else {
        if let Ok(interfaces) = nix::net::if_::if_nameindex() {
            for iface in interfaces.iter() {
                if let Ok(ethernet_info) = EthernetInfo::from_name(iface.name().to_string_lossy().as_ref()) {
                    ethernet_info_vec.push(ethernet_info);
                }
            };
        }
    }
    ethernet_info_vec
}
