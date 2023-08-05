//! Constants for ethtool
//!
//! This file is copied from the c2rust conversion of ethtool.c
#![allow(non_camel_case_types)]

use enum_iterator::Sequence;
use std::fmt::Display;
pub const ETHTOOL_GLINKSETTINGS: u32 = 0x0000004C;
pub const IFNAMSIZ: usize = 16;
pub const ETHTOOL_LINK_MODE_MASK_MAX_KERNEL_NU32: usize = i8::MAX as usize;
pub const ETHTOOL_LINK_MODE_MASK_MAX_KERNEL_NBITS: u32 = 32 * i8::MAX as u32;

/// The enumerate type represents the bits of the supported port
#[derive(Debug, PartialEq, Sequence, Clone, Copy)]
pub enum EthtoolPortBits {
    ETHTOOL_LINK_MODE_BNC_BIT = 11,
    ETHTOOL_LINK_MODE_FIBRE_BIT = 10,
    ETHTOOL_LINK_MODE_MII_BIT = 9,
    ETHTOOL_LINK_MODE_AUI_BIT = 8,
    ETHTOOL_LINK_MODE_TP_BIT = 7,
    ETHTOOL_LINK_MODE_Backplane_BIT = 16,
}

impl Display for EthtoolPortBits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EthtoolPortBits::ETHTOOL_LINK_MODE_BNC_BIT => write!(f, "BNC"),
            EthtoolPortBits::ETHTOOL_LINK_MODE_FIBRE_BIT => write!(f, "FIBRE"),
            EthtoolPortBits::ETHTOOL_LINK_MODE_MII_BIT => write!(f, "MII"),
            EthtoolPortBits::ETHTOOL_LINK_MODE_AUI_BIT => write!(f, "AUI"),
            EthtoolPortBits::ETHTOOL_LINK_MODE_TP_BIT => write!(f, "TP"),
            EthtoolPortBits::ETHTOOL_LINK_MODE_Backplane_BIT => write!(f, "BACKPLANE"),
        }
    }
}


/// The enumerate type represents the port type of the interface
#[derive(Debug, PartialEq, Sequence, Clone, Copy)]
pub enum EthtoolPort {
    ETHTOOL_LINK_MODE_BNC_BIT = 11,
    ETHTOOL_LINK_MODE_FIBRE_BIT = 10,
    ETHTOOL_LINK_MODE_MII_BIT = 9,
    ETHTOOL_LINK_MODE_AUI_BIT = 8,
    ETHTOOL_LINK_MODE_TP_BIT = 7,
    ETHTOOL_LINK_MODE_Backplane_BIT = 16,
    ETHTOOL_LINK_MODE_None_BIT = 239,
    ETHTOOL_LINK_MODE_Other_BIT = 255,
    ETHTOOL_LINK_MODE_Unknown_BIT = 0,
}

impl Default for EthtoolPort {
    fn default() -> Self {
        EthtoolPort::ETHTOOL_LINK_MODE_Unknown_BIT
    }
}

impl Display for EthtoolPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self as u8 {
            0 => write!(f, "Twisted Pair"),
            1 => write!(f, "AUI"),
            4 => write!(f, "BNC"),
            2 => write!(f, "MII"),
            3 => write!(f, "FIBRE"),
            5 => write!(f, "Direct Attach Cable"),
            239 => write!(f, "None"),
            255 => write!(f, "Other"),
            _ => write!(f, "Unknown"),
        }
    }
}

/// The enumerate type represents the link mode bits of the interface
#[derive(Debug, PartialEq, Sequence, Clone, Copy)]
pub enum EthtoolLinkModeBits {
    ETHTOOL_LINK_MODE_10baseT1S_P2MP_Half_BIT = 101,
    ETHTOOL_LINK_MODE_10baseT1S_Half_BIT = 100,
    ETHTOOL_LINK_MODE_10baseT1S_Full_BIT = 99,
    ETHTOOL_LINK_MODE_800000baseVR8_Full_BIT = 98,
    ETHTOOL_LINK_MODE_800000baseSR8_Full_BIT = 97,
    ETHTOOL_LINK_MODE_800000baseDR8_2_Full_BIT = 96,
    ETHTOOL_LINK_MODE_800000baseDR8_Full_BIT = 95,
    ETHTOOL_LINK_MODE_800000baseKR8_Full_BIT = 94,
    ETHTOOL_LINK_MODE_800000baseCR8_Full_BIT = 93,
    ETHTOOL_LINK_MODE_10baseT1L_Full_BIT = 92,
    ETHTOOL_LINK_MODE_100baseFX_Full_BIT = 91,
    ETHTOOL_LINK_MODE_100baseFX_Half_BIT = 90,
    ETHTOOL_LINK_MODE_400000baseCR4_Full_BIT = 89,
    ETHTOOL_LINK_MODE_400000baseDR4_Full_BIT = 88,
    ETHTOOL_LINK_MODE_400000baseLR4_ER4_FR4_Full_BIT = 87,
    ETHTOOL_LINK_MODE_400000baseSR4_Full_BIT = 86,
    ETHTOOL_LINK_MODE_400000baseKR4_Full_BIT = 85,
    ETHTOOL_LINK_MODE_200000baseCR2_Full_BIT = 84,
    ETHTOOL_LINK_MODE_200000baseDR2_Full_BIT = 83,
    ETHTOOL_LINK_MODE_200000baseLR2_ER2_FR2_Full_BIT = 82,
    ETHTOOL_LINK_MODE_200000baseSR2_Full_BIT = 81,
    ETHTOOL_LINK_MODE_200000baseKR2_Full_BIT = 80,
    ETHTOOL_LINK_MODE_100000baseDR_Full_BIT = 79,
    ETHTOOL_LINK_MODE_100000baseCR_Full_BIT = 78,
    ETHTOOL_LINK_MODE_100000baseLR_ER_FR_Full_BIT = 77,
    ETHTOOL_LINK_MODE_100000baseSR_Full_BIT = 76,
    ETHTOOL_LINK_MODE_100000baseKR_Full_BIT = 75,
    ETHTOOL_LINK_MODE_400000baseCR8_Full_BIT = 73,
    ETHTOOL_LINK_MODE_400000baseDR8_Full_BIT = 72,
    ETHTOOL_LINK_MODE_400000baseLR8_ER8_FR8_Full_BIT = 71,
    ETHTOOL_LINK_MODE_400000baseSR8_Full_BIT = 70,
    ETHTOOL_LINK_MODE_400000baseKR8_Full_BIT = 69,
    ETHTOOL_LINK_MODE_1000baseT1_Full_BIT = 68,
    ETHTOOL_LINK_MODE_100baseT1_Full_BIT = 67,
    ETHTOOL_LINK_MODE_200000baseCR4_Full_BIT = 66,
    ETHTOOL_LINK_MODE_200000baseDR4_Full_BIT = 65,
    ETHTOOL_LINK_MODE_200000baseLR4_ER4_FR4_Full_BIT = 64,
    ETHTOOL_LINK_MODE_200000baseSR4_Full_BIT = 63,
    ETHTOOL_LINK_MODE_200000baseKR4_Full_BIT = 62,
    ETHTOOL_LINK_MODE_100000baseDR2_Full_BIT = 61,
    ETHTOOL_LINK_MODE_100000baseLR2_ER2_FR2_Full_BIT = 60,
    ETHTOOL_LINK_MODE_100000baseCR2_Full_BIT = 59,
    ETHTOOL_LINK_MODE_100000baseSR2_Full_BIT = 58,
    ETHTOOL_LINK_MODE_100000baseKR2_Full_BIT = 57,
    ETHTOOL_LINK_MODE_50000baseDR_Full_BIT = 56,
    ETHTOOL_LINK_MODE_50000baseLR_ER_FR_Full_BIT = 55,
    ETHTOOL_LINK_MODE_50000baseCR_Full_BIT = 54,
    ETHTOOL_LINK_MODE_50000baseSR_Full_BIT = 53,
    ETHTOOL_LINK_MODE_50000baseKR_Full_BIT = 52,
    ETHTOOL_LINK_MODE_5000baseT_Full_BIT = 48,
    ETHTOOL_LINK_MODE_2500baseT_Full_BIT = 47,
    ETHTOOL_LINK_MODE_10000baseER_Full_BIT = 46,
    ETHTOOL_LINK_MODE_10000baseLRM_Full_BIT = 45,
    ETHTOOL_LINK_MODE_10000baseLR_Full_BIT = 44,
    ETHTOOL_LINK_MODE_10000baseSR_Full_BIT = 43,
    ETHTOOL_LINK_MODE_10000baseCR_Full_BIT = 42,
    ETHTOOL_LINK_MODE_1000baseX_Full_BIT = 41,
    ETHTOOL_LINK_MODE_50000baseSR2_Full_BIT = 40,
    ETHTOOL_LINK_MODE_100000baseLR4_ER4_Full_BIT = 39,
    ETHTOOL_LINK_MODE_100000baseCR4_Full_BIT = 38,
    ETHTOOL_LINK_MODE_100000baseSR4_Full_BIT = 37,
    ETHTOOL_LINK_MODE_100000baseKR4_Full_BIT = 36,
    ETHTOOL_LINK_MODE_50000baseKR2_Full_BIT = 35,
    ETHTOOL_LINK_MODE_50000baseCR2_Full_BIT = 34,
    ETHTOOL_LINK_MODE_25000baseSR_Full_BIT = 33,
    ETHTOOL_LINK_MODE_25000baseKR_Full_BIT = 32,
    ETHTOOL_LINK_MODE_25000baseCR_Full_BIT = 31,
    ETHTOOL_LINK_MODE_56000baseLR4_Full_BIT = 30,
    ETHTOOL_LINK_MODE_56000baseSR4_Full_BIT = 29,
    ETHTOOL_LINK_MODE_56000baseCR4_Full_BIT = 28,
    ETHTOOL_LINK_MODE_56000baseKR4_Full_BIT = 27,
    ETHTOOL_LINK_MODE_40000baseLR4_Full_BIT = 26,
    ETHTOOL_LINK_MODE_40000baseSR4_Full_BIT = 25,
    ETHTOOL_LINK_MODE_40000baseCR4_Full_BIT = 24,
    ETHTOOL_LINK_MODE_40000baseKR4_Full_BIT = 23,
    ETHTOOL_LINK_MODE_20000baseKR2_Full_BIT = 22,
    ETHTOOL_LINK_MODE_20000baseMLD2_Full_BIT = 21,
    ETHTOOL_LINK_MODE_10000baseR_FEC_BIT = 20,
    ETHTOOL_LINK_MODE_10000baseKR_Full_BIT = 19,
    ETHTOOL_LINK_MODE_10000baseKX4_Full_BIT = 18,
    ETHTOOL_LINK_MODE_1000baseKX_Full_BIT = 17,
    ETHTOOL_LINK_MODE_2500baseX_Full_BIT = 15,
    ETHTOOL_LINK_MODE_10000baseT_Full_BIT = 12,
    ETHTOOL_LINK_MODE_1000baseT_Full_BIT = 5,
    ETHTOOL_LINK_MODE_1000baseT_Half_BIT = 4,
    ETHTOOL_LINK_MODE_100baseT_Full_BIT = 3,
    ETHTOOL_LINK_MODE_100baseT_Half_BIT = 2,
    ETHTOOL_LINK_MODE_10baseT_Full_BIT = 1,
    ETHTOOL_LINK_MODE_10baseT_Half_BIT = 0,
}

impl Display for EthtoolLinkModeBits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Implement Display for EthtoolLinkModeBits according to
        // ETHTOOL_SUPPORTED_BIT_MODES
        match self {
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_10baseT_Half_BIT => {
                write!(f, "10baseT/Half")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_10baseT_Full_BIT => {
                write!(f, "10baseT/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_100baseT_Half_BIT => {
                write!(f, "100baseT/Half")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_100baseT_Full_BIT => {
                write!(f, "100baseT/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_1000baseT_Half_BIT => {
                write!(f, "1000baseT/Half")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_1000baseT_Full_BIT => {
                write!(f, "1000baseT/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_10000baseT_Full_BIT => {
                write!(f, "10000baseT/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_2500baseX_Full_BIT => {
                write!(f, "2500baseX/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_1000baseKX_Full_BIT => {
                write!(f, "1000baseKX/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_10000baseKX4_Full_BIT => {
                write!(f, "10000baseKX4/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_10000baseKR_Full_BIT => {
                write!(f, "10000baseKR/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_10000baseR_FEC_BIT => {
                write!(f, "10000baseR_FEC")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_20000baseMLD2_Full_BIT => {
                write!(f, "20000baseMLD2/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_20000baseKR2_Full_BIT => {
                write!(f, "20000baseKR2/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_40000baseKR4_Full_BIT => {
                write!(f, "40000baseKR4/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_40000baseCR4_Full_BIT => {
                write!(f, "40000baseCR4/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_40000baseSR4_Full_BIT => {
                write!(f, "40000baseSR4/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_40000baseLR4_Full_BIT => {
                write!(f, "40000baseLR4/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_56000baseKR4_Full_BIT => {
                write!(f, "56000baseKR4/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_56000baseCR4_Full_BIT => {
                write!(f, "56000baseCR4/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_56000baseSR4_Full_BIT => {
                write!(f, "56000baseSR4/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_56000baseLR4_Full_BIT => {
                write!(f, "56000baseLR4/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_25000baseCR_Full_BIT => {
                write!(f, "25000baseCR/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_25000baseKR_Full_BIT => {
                write!(f, "25000baseKR/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_25000baseSR_Full_BIT => {
                write!(f, "25000baseSR/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_50000baseCR2_Full_BIT => {
                write!(f, "50000baseCR2/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_50000baseKR2_Full_BIT => {
                write!(f, "50000baseKR2/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_100000baseKR4_Full_BIT => {
                write!(f, "100000baseKR4/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_100000baseSR4_Full_BIT => {
                write!(f, "100000baseSR4/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_100000baseCR4_Full_BIT => {
                write!(f, "100000baseCR4/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_100000baseLR4_ER4_Full_BIT => {
                write!(f, "100000baseLR4_ER4/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_50000baseSR2_Full_BIT => {
                write!(f, "50000baseSR2/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_1000baseX_Full_BIT => {
                write!(f, "1000baseX/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_10000baseCR_Full_BIT => {
                write!(f, "10000baseCR/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_10000baseSR_Full_BIT => {
                write!(f, "10000baseSR/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_10000baseLR_Full_BIT => {
                write!(f, "10000baseLR/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_10000baseLRM_Full_BIT => {
                write!(f, "10000baseLRM/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_10000baseER_Full_BIT => {
                write!(f, "10000baseER/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_2500baseT_Full_BIT => {
                write!(f, "2500baseT/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_5000baseT_Full_BIT => {
                write!(f, "5000baseT/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_50000baseKR_Full_BIT => {
                write!(f, "50000baseKR/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_50000baseSR_Full_BIT => {
                write!(f, "50000baseSR/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_50000baseCR_Full_BIT => {
                write!(f, "50000baseCR/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_50000baseLR_ER_FR_Full_BIT => {
                write!(f, "50000baseLR_ER_FR/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_50000baseDR_Full_BIT => {
                write!(f, "50000baseDR/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_100000baseKR2_Full_BIT => {
                write!(f, "100000baseKR2/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_100000baseSR2_Full_BIT => {
                write!(f, "100000baseSR2/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_100000baseCR2_Full_BIT => {
                write!(f, "100000baseCR2/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_100000baseLR2_ER2_FR2_Full_BIT => {
                write!(f, "100000baseLR2_ER2_FR2/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_100000baseDR2_Full_BIT => {
                write!(f, "100000baseDR2/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_200000baseKR4_Full_BIT => {
                write!(f, "200000baseKR4/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_200000baseSR4_Full_BIT => {
                write!(f, "200000baseSR4/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_200000baseLR4_ER4_FR4_Full_BIT => {
                write!(f, "200000baseLR4_ER4_FR4/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_200000baseDR4_Full_BIT => {
                write!(f, "200000baseDR4/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_200000baseCR4_Full_BIT => {
                write!(f, "200000baseCR4/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_100baseT1_Full_BIT => {
                write!(f, "100baseT1/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_1000baseT1_Full_BIT => {
                write!(f, "1000baseT1/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_400000baseKR8_Full_BIT => {
                write!(f, "400000baseKR8/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_400000baseSR8_Full_BIT => {
                write!(f, "400000baseSR8/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_400000baseLR8_ER8_FR8_Full_BIT => {
                write!(f, "400000baseLR8_ER8_FR8/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_400000baseDR8_Full_BIT => {
                write!(f, "400000baseDR8/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_400000baseCR8_Full_BIT => {
                write!(f, "400000baseCR8/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_100000baseKR_Full_BIT => {
                write!(f, "100000baseKR/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_100000baseSR_Full_BIT => {
                write!(f, "100000baseSR/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_100000baseLR_ER_FR_Full_BIT => {
                write!(f, "100000baseLR_ER_FR/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_100000baseDR_Full_BIT => {
                write!(f, "100000baseDR/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_100000baseCR_Full_BIT => {
                write!(f, "100000baseCR/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_200000baseKR2_Full_BIT => {
                write!(f, "200000baseKR2/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_200000baseSR2_Full_BIT => {
                write!(f, "200000baseSR2/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_200000baseLR2_ER2_FR2_Full_BIT => {
                write!(f, "200000baseLR2_ER2_FR2/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_200000baseDR2_Full_BIT => {
                write!(f, "200000baseDR2/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_200000baseCR2_Full_BIT => {
                write!(f, "200000baseCR2/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_400000baseKR4_Full_BIT => {
                write!(f, "400000baseKR4/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_400000baseSR4_Full_BIT => {
                write!(f, "400000baseSR4/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_400000baseLR4_ER4_FR4_Full_BIT => {
                write!(f, "400000baseLR4_ER4_FR4/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_400000baseDR4_Full_BIT => {
                write!(f, "400000baseDR4/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_400000baseCR4_Full_BIT => {
                write!(f, "400000baseCR4/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_100baseFX_Half_BIT => {
                write!(f, "100baseFX/Half")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_100baseFX_Full_BIT => {
                write!(f, "100baseFX/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_10baseT1L_Full_BIT => {
                write!(f, "10baseT1L/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_800000baseCR8_Full_BIT => {
                write!(f, "800000baseCR8/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_800000baseKR8_Full_BIT => {
                write!(f, "800000baseKR8/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_800000baseDR8_Full_BIT => {
                write!(f, "800000baseDR8/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_800000baseDR8_2_Full_BIT => {
                write!(f, "800000baseDR8_2/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_800000baseSR8_Full_BIT => {
                write!(f, "800000baseSR8/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_800000baseVR8_Full_BIT => {
                write!(f, "800000baseVR8/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_10baseT1S_Full_BIT => {
                write!(f, "10baseT1S/Full")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_10baseT1S_Half_BIT => {
                write!(f, "10baseT1S/Half")
            }
            EthtoolLinkModeBits::ETHTOOL_LINK_MODE_10baseT1S_P2MP_Half_BIT => {
                write!(f, "10baseT1S/Half")
            }
        }
    }
}
