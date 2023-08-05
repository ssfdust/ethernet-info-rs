//! This module contains the SettingsParser struct and its implementation.
//! The SettingsParser struct is used to parse the bit settings of a network
//! interface.
use crate::ethtool_const::{
    EthtoolLinkModeBits, EthtoolPortBits, ETHTOOL_LINK_MODE_MASK_MAX_KERNEL_NBITS, EthtoolPort,
};
use enum_iterator::all;

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

    /// Returns the supported port types of the network interface.
    pub fn supported_ports(&'a self) -> Vec<EthtoolPortBits> {
        let mut modes = Vec::new();

        for mode in all::<EthtoolPortBits>() {
            if ethtool_link_mode_test_bit(mode as u32, self.supported_link_modes) {
                modes.push(mode);
            }
        }
        modes
    }

    /// Returns the supported modes of the network interface.
    pub fn supported_link_modes(&'a self) -> Vec<EthtoolLinkModeBits> {
        let mut modes = Vec::new();

        for mode in all::<EthtoolLinkModeBits>() {
            if ethtool_link_mode_test_bit(mode as u32, self.supported_link_modes) {
                modes.push(mode);
            }
        }
        modes
    }

    /// Returns the port type of the network interface.
    pub fn port(&'a self) -> EthtoolPort {
        let mode = EthtoolPort::ETHTOOL_LINK_MODE_Unknown_BIT;
        for mode in all::<EthtoolPort>() {
            if mode as u8 == self.port {
                return mode
            }
        }
        mode
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
