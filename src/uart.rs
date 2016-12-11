//! # UART for the LM4F120H5QR

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

use core::fmt;
use core::intrinsics::{volatile_load, volatile_store};

use embedded_serial::{self, BlockingTx, NonBlockingRx};
use cortex_m::asm::nop;

use super::gpio;
use super::registers as reg;


// ****************************************************************************
//
// Public Types
//
// ****************************************************************************

/// This chip has 8 UARTs
#[derive(PartialEq, Clone, Copy)]
#[allow(missing_docs)]
pub enum UartId {
    Uart0,
    Uart1,
    Uart2,
}

/// Controls a single UART
/// Only supports 8/N/1 - who needs anything else?
pub struct Uart {
    id: UartId,
    baud: u32,
    nl_mode: NewlineMode, // reg: &'static mut reg::UartRegisters,
}

/// writeln!() emits LF chars, so this is useful
/// if you're writing text with your UART
#[derive(PartialEq, Clone, Copy)]
pub enum NewlineMode {
    /// Emit octets as received
    Binary,
    /// Emit an extra CR before every LF
    SwapLFtoCRLF,
}

// ****************************************************************************
//
// Private Types
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Data
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************

/// Represents a single UART on the LM4F120
impl Uart {
    /// Create a new Uart object. The caller is responsible for ensuring
    /// that only one object exists per UartId. The UART is set to
    /// 8 data bits, 1 stop bit, no parity and is not configurable.
    /// Optionally, newline translation can be performed on outbound data
    /// - this will cause writeln!() to emit a CRLF.
    pub fn new(id: UartId, baud: u32, nl_mode: NewlineMode) -> Uart {
        let mut uart = Uart {
            id: id,
            baud: baud,
            nl_mode: nl_mode, // reg: get_uart_registers(id),
        };
        uart.init();
        uart
    }

    /// Configure the hardware
    fn init(&mut self) -> () {}
}

impl embedded_serial::BlockingTx for Uart {
    type Error = ();

    /// Emit a single octet, busy-waiting if the FIFO is full.
    /// Never returns `Err`.
    fn putc(&mut self, value: u8) -> Result<(), Self::Error> {
        // while (self.reg.rf.read() & reg::UART_FR_TXFF) != 0 {
        //     nop();
        // }
        // self.reg.data.write(value as usize);
        // Ok(())
        Err(())
    }
}

impl embedded_serial::NonBlockingRx for Uart {
    type Error = ();

    /// Attempts to read from the UART. Returns `Err(())`
    /// if the FIFO is empty, or `Ok(octet)`.
    fn getc_try(&mut self) -> Result<u8, Self::Error> {
        // if (self.reg.rf.read() & reg::UART_FR_RXFE) != 0 {
        Err(())
        // } else {
        //     Ok(self.reg.data.read() as u8)
        // }
    }
}

/// Allows the Uart to be passed to 'write!()' and friends.
impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match self.nl_mode {
            NewlineMode::Binary => {
                for byte in s.bytes() {
                    self.putc(byte).unwrap()
                }
            }
            NewlineMode::SwapLFtoCRLF => {
                for byte in s.bytes() {
                    if byte == 0x0A {
                        // Prefix every \n with a \r
                        self.putc(0x0D).unwrap()
                    }
                    self.putc(byte).unwrap()
                }
            }
        }
        Ok(())
    }
}

/// Called when UART 0 interrupt fires
pub unsafe extern "C" fn uart0_isr() {}

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

// /// Get a reference to the UART control register struct in the chip.
// fn get_uart_registers(uart_id: UartId) -> &'static mut reg::UartRegisters {
//     // unsafe {
//     //     match uart_id {
//     //         UartId::Uart0 => &mut *(reg::UART0_DR_R as *mut reg::UartRegisters),
//     //         UartId::Uart1 => &mut *(reg::UART1_DR_R as *mut reg::UartRegisters),
//     //         UartId::Uart2 => &mut *(reg::UART2_DR_R as *mut reg::UartRegisters),
//     //     }
//     // }
// }

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
