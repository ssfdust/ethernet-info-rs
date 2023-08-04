//! This module contains the SettingsParser struct and its implementation.
//! The SettingsParser struct is used to parse the bit settings of a network
//! interface.
use crate::ethtool_const::*;

/// # SettingsParser
/// The SettingsParser struct is used to parse the bit settings of a network
/// interface.
///
/// ## Arguments
/// * `port` - The port type number of the network interface.
/// * `supported_link_modes` - The supported link modes of the network interface.
pub struct SettingsParser<'a> {
    port: u8,
    supported_link_modes: &'a [u32],
}

impl<'a> SettingsParser<'a> {
    pub fn new(port: u8, supported_link_modes: &'a [u32]) -> SettingsParser<'a> {
        SettingsParser {
            port,
            supported_link_modes,
        }
    }

    /// Returns the supported modes of the network interface.
    pub fn supported_link_modes(&'a self) -> Vec<String> {
        let mut modes = Vec::new();
        for (bit, mode) in ETHTOOL_PORT_BIT_MODES.iter() {
            if ethtool_link_mode_test_bit(*bit, self.supported_link_modes) {
                modes.push(mode.to_string());
            }
        }

        for (bit, mode) in ETHTOOL_SUPPORTED_BIT_MODES.iter() {
            if ethtool_link_mode_test_bit(*bit, self.supported_link_modes) {
                modes.push(mode.to_string());
            }
        }
        modes
    }

    /// Returns the port type of the network interface.
    pub fn port(&'a self) -> String {
        match self.port {
            0 => "Twisted Pair".into(),
            1 => "AUI".into(),
            4 => "BNC".into(),
            2 => "MII".into(),
            3 => "FIBRE".into(),
            5 => "Direct Attach Cable".into(),
            239 => "None".into(),
            255 => "Other".into(),
            _ => "Unknown".into(),
        }
    }
}

/// Test bit nr in mask
fn ethtool_link_mode_test_bit(nr: u32, mask: &[u32]) -> bool {
    if nr >= ETHTOOL_LINK_MODE_MASK_MAX_KERNEL_NBITS {
        false
    } else {
        (mask[(nr / 32) as usize] & (1 << (nr % 32))) != 0
    }
}
