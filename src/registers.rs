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

/// Internal Clock Source
#[repr(C, packed)]
pub struct IcsRegisters {
    /// ICS Control Register 1
    pub c1: RW<u8>,
    /// ICS Control Register 2
    pub c2: RW<u8>,
    /// ICS Control Register 3
    pub c3: RW<u8>,
    /// ICS Control Register 4
    pub c4: RW<u8>,
    /// ICS Status Register
    pub status: RW<u8>,
}

/// Oscillator
#[repr(C, packed)]
pub struct OscRegisters {
    pub cr: RW<u8>,
}


// ****************************************************************************
//
// Public Data
//
// ****************************************************************************

pub const SIM_BASE: usize = 0x40048000;
pub const PORT_BASE: usize = 0x40049000;
pub const ICS_BASE: usize = 0x40064000;
pub const OSC_BASE: usize = 0x40065000;
pub const UART0_BASE: usize = 0x4006A000;
pub const UART1_BASE: usize = 0x4006B000;
pub const UART2_BASE: usize = 0x4006C000;
pub const GPIO0_BASE: usize = 0xF8000000;
pub const GPIO1_BASE: usize = 0xF8000040;
pub const GPIO2_BASE: usize = 0xF8000080;

pub const ICS_C1_IREFSTEN: u8 = 1 << 0;
pub const ICS_C1_IRCLKEN: u8 = 1 << 1;
pub const ICS_C1_IREFS: u8 = 1 << 2;
pub const ICS_C1_RDIV: u8 = 0b111 << 3;
pub fn ics_c1_rdiv(x: u8) -> u8 {
    (x & 0b111) << 3
}
pub const ICS_C1_CLKS: u8 = 0b11 << 6;
pub fn ics_c1_clks(x: u8) -> u8 {
    (x & 0b11) << 6
}

pub const ICS_C2_LP: u8 = 1 << 4;
pub const ICS_C2_BDIV: u8 = 0b111 << 5;
pub fn ics_c2_bdiv(x: u8) -> u8 {
    (x & 0b111) << 5
}

pub const ICS_C4_SCFTRIM: u8 = 1 << 0;
pub const ICS_C4_CME: u8 = 1 << 5;
pub const ICS_C4_LOLIE: u8 = 1 << 7;

pub const ICS_STATUS_CLKST: u8 = 0b11 << 2;
pub const ICS_STATUS_IREFST: u8 = 1 << 4;
pub const ICS_STATUS_LOCK: u8 = 1 << 6;
pub const ICS_STATUS_LOLS: u8 = 1 << 7;

pub const UART_C1_PT: u8 = 1 << 0;
pub const UART_C1_PE: u8 = 1 << 1;
pub const UART_C1_ILT: u8 = 1 << 2;
pub const UART_C1_WAKE: u8 = 1 << 3;
pub const UART_C1_M: u8 = 1 << 4;
pub const UART_C1_RSRC: u8 = 1 << 5;
pub const UART_C1_UARTSWAI: u8 = 1 << 6;
pub const UART_C1_LOOPS: u8 = 1 << 7;

pub const UART_C2_SBK: u8 = 1 << 0;
pub const UART_C2_RWU: u8 = 1 << 1;
pub const UART_C2_RE: u8 = 1 << 2;
pub const UART_C2_TE: u8 = 1 << 3;
pub const UART_C2_ILIE: u8 = 1 << 4;
pub const UART_C2_RIE: u8 = 1 << 5;
pub const UART_C2_TCIE: u8 = 1 << 6;
pub const UART_C2_TIE: u8 = 1 << 7;

pub const UART_S1_PF: u8 = 1 << 0;
pub const UART_S1_FE: u8 = 1 << 1;
pub const UART_S1_NF: u8 = 1 << 2;
pub const UART_S1_OR: u8 = 1 << 3;
pub const UART_S1_IDLE: u8 = 1 << 4;
pub const UART_S1_RDRF: u8 = 1 << 5;
pub const UART_S1_TC: u8 = 1 << 6;
pub const UART_S1_TDRE: u8 = 1 << 7;

pub const UART_S2_RAF: u8 = 1 << 0;
pub const UART_S2_LBKDE: u8 = 1 << 1;
pub const UART_S2_BRK13: u8 = 1 << 2;
pub const UART_S2_RWUID: u8 = 1 << 3;
pub const UART_S2_RXINV: u8 = 1 << 4;
pub const UART_S2_RXEDGIF: u8 = 1 << 6;
pub const UART_S2_LBKDIF: u8 = 1 << 7;

pub const UART_C3_PEIE: u8 = 1 << 0;
pub const UART_C3_FEIE: u8 = 1 << 1;
pub const UART_C3_NEIE: u8 = 1 << 2;
pub const UART_C3_ORIE: u8 = 1 << 3;
pub const UART_C3_TXINV: u8 = 1 << 4;
pub const UART_C3_TXDIR: u8 = 1 << 5;
pub const UART_C3_T8: u8 = 1 << 6;
pub const UART_C3_R8: u8 = 1 << 7;

pub const SIM_SCGC_RTC: u32 = 1 << 0;
pub const SIM_SCGC_PIT: u32 = 1 << 1;
pub const SIM_SCGC_PWT: u32 = 1 << 4;
pub const SIM_SCGC_FTM0: u32 = 1 << 5;
pub const SIM_SCGC_FTM1: u32 = 1 << 6;
pub const SIM_SCGC_FTM2: u32 = 1 << 7;
pub const SIM_SCGC_CRC: u32 = 1 << 10;
pub const SIM_SCGC_FLASH: u32 = 1 << 12;
pub const SIM_SCGC_SWD: u32 = 1 << 13;
pub const SIM_SCGC_MSCAN: u32 = 1 << 15;
pub const SIM_SCGC_I2C0: u32 = 1 << 16;
pub const SIM_SCGC_I2C1: u32 = 1 << 17;
pub const SIM_SCGC_SPI0: u32 = 1 << 18;
pub const SIM_SCGC_SPI1: u32 = 1 << 19;
pub const SIM_SCGC_UART0: u32 = 1 << 20;
pub const SIM_SCGC_UART1: u32 = 1 << 21;
pub const SIM_SCGC_UART2: u32 = 1 << 22;
pub const SIM_SCGC_KBI0: u32 = 1 << 24;
pub const SIM_SCGC_KBI1: u32 = 1 << 25;
pub const SIM_SCGC_IRQ: u32 = 1 << 27;
pub const SIM_SCGC_ADC: u32 = 1 << 29;
pub const SIM_SCGC_ACMP0: u32 = 1 << 30;
pub const SIM_SCGC_ACMP1: u32 = 1 << 31;

pub const OSC_CR_OSCINIT: u8 = 1 << 0;
pub const OSC_CR_HGO: u8 = 1 << 1;
pub const OSC_CR_RANGE: u8 = 1 << 2;
pub const OSC_CR_OSCOS: u8 = 1 << 4;
pub const OSC_CR_OSCSTEN: u8 = 1 << 5;
pub const OSC_CR_OSCEN: u8 = 1 << 7;

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

pub fn get_uart0() -> &'static mut UartRegisters {
    unsafe { &mut *(UART0_BASE as *mut UartRegisters) }
}

pub fn get_uart1() -> &'static mut UartRegisters {
    unsafe { &mut *(UART1_BASE as *mut UartRegisters) }
}

pub fn get_uart2() -> &'static mut UartRegisters {
    unsafe { &mut *(UART2_BASE as *mut UartRegisters) }
}

pub fn get_gpio0() -> &'static mut GpioRegisters {
    unsafe { &mut *(GPIO0_BASE as *mut GpioRegisters) }
}

pub fn get_gpio1() -> &'static mut GpioRegisters {
    unsafe { &mut *(GPIO1_BASE as *mut GpioRegisters) }
}

pub fn get_gpio2() -> &'static mut GpioRegisters {
    unsafe { &mut *(GPIO2_BASE as *mut GpioRegisters) }
}

pub fn get_sim() -> &'static mut SimRegisters {
    unsafe { &mut *(SIM_BASE as *mut SimRegisters) }
}

pub fn get_osc() -> &'static mut OscRegisters {
    unsafe { &mut *(OSC_BASE as *mut OscRegisters) }
}

pub fn get_ics() -> &'static mut IcsRegisters {
    unsafe { &mut *(ICS_BASE as *mut IcsRegisters) }
    // ********************************  ********************************************
}

// ****************************************************************************
//
// Private Functions
//

// None

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
