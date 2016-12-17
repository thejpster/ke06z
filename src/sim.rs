//! System Integration Module for the KE06Z
//!
//! See KE06 Sub-Family Reference Manual [1]

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

use super::registers as reg;

// ****************************************************************************
//
// Public Types
//
// ****************************************************************************

/// Peripherls that can be enabled or disabled
#[derive(Clone)]
pub enum ClockGatingPeripheral {
    Acmp1,
    Acmp0,
    Adc,
    Irq,
    Kbi1,
    Kbi0,
    Uart2,
    Uart1,
    Uart0,
    Spi1,
    Spi0,
    I2c1,
    I2c0,
    Mscan,
    Swd,
    Flash,
    Crc,
    Fmt2,
    Fmt1,
    Fmt0,
    Pwt,
    Pit,
    Rtc,
}

// ****************************************************************************
//
// Public Data
//
// ****************************************************************************

// None

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

pub fn init() {}

pub fn power_enable(peripheral: ClockGatingPeripheral) {
    power_control(peripheral, true)
}

pub fn power_disable(peripheral: ClockGatingPeripheral) {
    power_control(peripheral, false)
}

pub fn power_control(peripheral: ClockGatingPeripheral, enable: bool) {
    let mask: u32 = match peripheral {
        ClockGatingPeripheral::Acmp1 => (1 << 31),
        ClockGatingPeripheral::Acmp0 => (1 << 30),
        ClockGatingPeripheral::Adc => (1 << 29),
        // 28 is reserved
        ClockGatingPeripheral::Irq => (1 << 27),
        // 26 is reserved
        ClockGatingPeripheral::Kbi1 => (1 << 25),
        ClockGatingPeripheral::Kbi0 => (1 << 24),
        // 23 is reserved
        ClockGatingPeripheral::Uart2 => (1 << 22),
        ClockGatingPeripheral::Uart1 => (1 << 21),
        ClockGatingPeripheral::Uart0 => (1 << 20),
        ClockGatingPeripheral::Spi1 => (1 << 19),
        ClockGatingPeripheral::Spi0 => (1 << 18),
        ClockGatingPeripheral::I2c1 => (1 << 17),
        ClockGatingPeripheral::I2c0 => (1 << 16),
        ClockGatingPeripheral::Mscan => (1 << 15),
        // 14 is reserved
        ClockGatingPeripheral::Swd => (1 << 13),
        ClockGatingPeripheral::Flash => (1 << 12),
        // 11 is reserved
        ClockGatingPeripheral::Crc => (1 << 10),
        // 9 and 8 are reserved
        ClockGatingPeripheral::Fmt2 => (1 << 7),
        ClockGatingPeripheral::Fmt1 => (1 << 6),
        ClockGatingPeripheral::Fmt0 => (1 << 5),
        ClockGatingPeripheral::Pwt => (1 << 4),
        // 3 and 2 are reserved
        ClockGatingPeripheral::Pit => (1 << 2),
        ClockGatingPeripheral::Rtc => (1 << 0),
    };

    let sim = registers();
    if enable {
        sim.scgc.modify(|x| x | mask);
    } else {
        sim.scgc.modify(|x| x & !mask);
    }
}

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

fn registers() -> &'static mut reg::SimRegisters {
    unsafe { &mut *(0x40048000 as *mut reg::SimRegisters) }
}

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
