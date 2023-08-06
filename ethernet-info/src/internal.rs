//! ## Internal module
//!
//! The internal module contains the unsafe code to interact with the kernel.
//! It is not intended to be used directly by the user.
//!
//! The module uses ioctl via SIOCETHTOOL to send commands to the kernel,
//! and read from response to get the link settings.
//!
//! This module use EthtoolLinkSettings and EthtoolCommnad to emulate the
//! C struct used by the kernel, which is defined in ethtool.h.
//! The struct version is linux 6.1 lts, tested on Centos 7.9 , Rocky Linux 8.8
//! and Arch Linux with linux 6.1 lts.
use crate::errors::EthtoolError;
use crate::ethtool_const::*;
use crate::settings_parser::SettingsParser;
use crate::EthernetInfo;
use libc::ifreq;
use nix::sys::socket::{socket, AddressFamily, SockFlag, SockType};
use nix::unistd::close;
use std::mem::{transmute, zeroed};
use std::os::unix::io::RawFd;

ioctl_readwrite_bad!(ethtool_ioctl, libc::SIOCETHTOOL, ifreq);

/// CmdContext is emulating the C struct defined in ethtool.c
/// It should be used with the struct EthtoolCommnad to send ioctl command.
///
/// Arguments:
/// * `devname` - The device name to send the ioctl command.
/// * `fd` - The file descriptor of the socket.
/// * `ifr` - The ifreq struct to send the ioctl command.
///
/// Methods:
/// * `new` - Create a new CmdContext.
/// * `get_ethtool_link_settings` - Get the EthtoolCommnad from the ifreq struct.
/// * `update_ifr_from_ethtool_cmd` - Update the ifreq struct from the EthtoolCommnad.
/// * `send_ioctl` - Send the ioctl command to the kernel.
/// * `close_socket` - Close the socket.
#[repr(C)]
#[derive(Clone)]
pub struct CmdContext {
    devname: String,
    fd: RawFd,
    ifr: ifreq,
}

/// # EthtoolLinkSettings
///
/// EthtoolLinkSettings is emulating the C struct defined in ethtool.h
/// It should be used with the struct EthtoolCommnad to send ioctl command.
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct EthtoolLinkSettings {
    pub cmd: u32,
    pub speed: u32,
    pub duplex: u8,
    pub port: u8,
    pub phy_address: u8,
    pub autoneg: u8,
    pub mdio_support: u8,
    pub eth_tp_mdix: u8,
    pub eth_tp_mdix_ctrl: u8,
    pub link_mode_masks_nwords: i8,
    pub transceiver: u8,
    pub master_slave_cfg: u8,
    pub master_slave_state: u8,
    pub rate_matching: u8,
    pub reserved: [u32; 7],
    pub link_mode_masks: [u32; 0],
}

/// # EthtoolCommnad
/// EthtoolCommnad is emulating the C struct defined in ethtool.h
/// It should be used with the struct CmdContext to send ioctl command.
///
/// It contains the EthtoolLinkSettings and the link_mode_data, the kernel
/// will write the data to the EthtoolCommnad, and the field of link_mode_data
/// will be filled with the link mode data, which we need to parse.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EthtoolCommnad {
    pub req: EthtoolLinkSettings,
    pub link_mode_data: [u32; 3 * ETHTOOL_LINK_MODE_MASK_MAX_KERNEL_NU32],
}

impl EthtoolCommnad {
    pub fn new(cmd: u32) -> Result<Self, EthtoolError> {
        let mut ecmd: Self = EthtoolCommnad {
            req: EthtoolLinkSettings::default(),
            link_mode_data: [0u32; 3 * ETHTOOL_LINK_MODE_MASK_MAX_KERNEL_NU32],
        };
        ecmd.req.cmd = cmd;
        Ok(ecmd)
    }

    /// Convert the EthtoolCommnad to EthernetInfo.
    ///
    /// The EthtoolCommnad is got from ioctl command, so it doesn't contain
    /// the device name, which should be provided as an argument.
    pub fn into_ethernet_info(self, devname: &str) -> EthernetInfo {
        unsafe {
            let mut supported_link_modes_u32 = [0u32; ETHTOOL_LINK_MODE_MASK_MAX_KERNEL_NU32];
            let mut advertised_link_modes_u32 = [0u32; ETHTOOL_LINK_MODE_MASK_MAX_KERNEL_NU32];

            let mut link_mode_data_ptr = self.link_mode_data.as_ptr();

            // read the supported link modes from the link_mode_data pointer
            // and copy it to the array.
            let supported_link_modes = std::slice::from_raw_parts(
                link_mode_data_ptr,
                self.req.link_mode_masks_nwords as usize,
            );
            supported_link_modes_u32[..supported_link_modes.len()]
                .copy_from_slice(supported_link_modes);

            // move the pointer to the advertised link modes. The size of the
            // supported link modes is link_mode_masks_nwords.
            link_mode_data_ptr =
                link_mode_data_ptr.offset(self.req.link_mode_masks_nwords as isize);

            // read the advertised link modes from the link_mode_data pointer
            // and copy it to the array.
            let advertised_link_modes = std::slice::from_raw_parts(
                link_mode_data_ptr,
                self.req.link_mode_masks_nwords as usize,
            );
            advertised_link_modes_u32[..advertised_link_modes.len()]
                .copy_from_slice(advertised_link_modes);

            let settings_parser = SettingsParser::new(
                self.req.port,
                &supported_link_modes_u32,
                &advertised_link_modes_u32,
            );

            EthernetInfo::from_settings_parser(devname, settings_parser)
        }
    }
}

/// Convert string to i8 array.
fn str_to_i8_arr(string: &str) -> [i8; IFNAMSIZ] {
    let mut arr_u8 = [0i8; IFNAMSIZ];
    for (idx, ch) in string.bytes().enumerate() {
        arr_u8[idx] = ch as i8;
    }
    arr_u8
}

impl CmdContext {
    /// Create a new CmdContext.
    /// The device name should be provided as an argument, and the device name
    /// should be shorter than IFNAMSIZ.
    /// The socket which is used to send ioctl command will be created, and
    /// the file descriptor of the socket must be closed after all the ioctl
    /// commands
    pub fn new(dev_name: &str) -> Result<Self, EthtoolError> {
        if dev_name.len() > IFNAMSIZ {
            return Err(EthtoolError::new("The device name is too long."));
        }
        let socket_ret = socket(
            AddressFamily::Inet,
            SockType::Datagram,
            SockFlag::empty(),
            None,
        );
        if socket_ret.is_err() {
            return Err(EthtoolError::new("Failed to create socket."));
        }
        let fd = socket_ret.unwrap();

        Ok(CmdContext {
            devname: dev_name.to_string(),
            fd,
            ifr: unsafe { zeroed() },
        })
    }

    /// Convert the ifreq struct which is filled by kernel to the EthtoolCommnad.
    pub fn get_ethtool_link_settings(&self) -> EthtoolCommnad {
        unsafe {
            let ecmd = self.ifr.ifr_ifru.ifru_data as *mut EthtoolCommnad;
            *ecmd
        }
    }

    /// Close the socket, which is initialized in the CmdContext::new.
    pub fn close_socket(&self) {
        close(self.fd).expect("Error close socket");
    }

    /// Get the device name.
    pub fn ifname(&self) -> &str {
        &self.devname
    }

    /// Send ioctl command to kernel, which will fill the ifreq struct.
    pub fn send_ioctl(mut self, mut ecmd: EthtoolCommnad) -> Result<CmdContext, EthtoolError> {
        self.ifr = ifreq {
            ifr_name: str_to_i8_arr(&self.devname),
            ifr_ifru: libc::__c_anonymous_ifr_ifru {
                ifru_data: unsafe { transmute(&mut ecmd as *mut _) },
            },
        };
        let ret = unsafe { ethtool_ioctl(self.fd, &mut self.ifr) };
        if ret.is_ok() {
            Ok(self)
        } else {
            Err(EthtoolError::new("Failed to send EthtoolCommnad"))
        }
    }
}
