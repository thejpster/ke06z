//! Register definitions for the Freescale Kinetis KE06Z

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

use volatile_register::{RO, RW, WO};

// ****************************************************************************
//
// Public Types
//
// ****************************************************************************

/// UART Module
#[repr(C, packed)]
pub struct UartRegisters {
    /// Baud rate high
    pub bdh: RW<u8>,
    /// Baud rate low
    pub bdl: RW<u8>,
    /// Control register 1
    pub c1: RW<u8>,
    /// Control register 2
    pub c2: RW<u8>,
    /// Status register 1
    pub s1: RO<u8>,
    /// Status register 2
    pub s2: RW<u8>,
    /// Control register 3
    pub c3: RW<u8>,
    /// Data register
    pub data: RW<u8>,
}

/// GPIO Module.
///
/// The GPIO is represented as three 32-bit banks.
#[repr(C, packed)]
pub struct GpioRegisters {
    /// Port Data Output
    pub dor: RW<u32>,
    /// Port Set Output
    pub sor: WO<u32>,
    /// Port Clear Output
    pub cor: WO<u32>,
    /// Port Toggle Output
    pub tor: WO<u32>,
    /// Port Data Input
    pub dir: RO<u32>,
    /// Port Data Direction
    pub ddr: RW<u32>,
    /// Port Input Disable
    pub idr: RW<u32>,
}

/// PORT Module.
///
/// The PORT module controls pull-ups and input filtering
pub struct PortRegisters {
    pub ioflt0: RW<u32>,
    pub ioflt1: RW<u32>,
    pub pue0: RW<u32>,
    pub pue1: RW<u32>,
    pub pue2: RW<u32>,
    pub hdrve: RW<u32>,
}

/// System Integration Module
#[repr(C, packed)]
pub struct SimRegisters {
    /// System Reset Status and ID Register
    pub srsid: RO<u32>,
    /// System Options Register 0
    pub sopt0: RW<u32>,
    /// System Options Register
    pub sopt1: RW<u32>,
    /// Pin Selection Register 0
    pub pinsel: RW<u32>,
    /// Pin Selection Register 1
    pub pinsel1: RW<u32>,
    /// System Clock Gating Control Register
    pub scgc: RW<u32>,
    /// Universally Unique Identifier Low Register
    pub uuidl: RO<u32>,
    /// Universally Unique Identifier Middle Low Register
    pub uuidh: RO<u32>,
    /// Universally Unique Identifier Middle High Register
    pub uuidmh: RO<u32>,
    /// Clock Divider Register
    pub clkdiv: RW<u32>,
}

// ****************************************************************************
//
// Public Data
//
// ****************************************************************************

pub const UART0_BASE:usize = 0x4006A000;
pub const UART1_BASE:usize = 0x4006B000;
pub const UART2_BASE:usize = 0x4006C000;
pub const GPIO0_BASE:usize = 0xF8000000;
pub const GPIO1_BASE:usize = 0xF8000040;
pub const GPIO2_BASE:usize = 0xF8000080;
pub const PORT_BASE:usize = 0x40049000;
pub const SIM_ADDR:usize = 0x40048000;

// ****************************************************************************
//
// Private Types
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Private Data
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

// None

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
