//! # UART for the KE06Z

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

use core::fmt;
use cortex_m::asm::nop;
use embedded_serial::{BlockingTx, NonBlockingRx};

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
    nl_mode: NewlineMode,
    reg: &'static mut reg::UartRegisters,
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
// Private Data
//
// ****************************************************************************

/// The Freedom board has an 8 MHz external oscillator
/// We set the internal clock to 20 MHz
const CLOCK_SPEED: u32 = 20_000_000;

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************

/// Represents a single UART on the KE06Z
impl Uart {
    /// Create a new Uart object. The caller is responsible for ensuring
    /// that only one object exists per UartId. The UART is set to
    /// 8 data bits, 1 stop bit, no parity and is not configurable.
    /// Optionally, newline translation can be performed on outbound data
    /// - this will cause writeln!() to emit a CRLF.
    pub fn new(id: UartId, baud: u32, nl_mode: NewlineMode) -> Uart {
        let mut uart = Uart {
            nl_mode: nl_mode,
            reg: get_uart_registers(id),
        };

        // Enable the UART peripheral
        reg::get_sim().scgc.modify(|x| {
            x |
            match id {
                UartId::Uart0 => reg::SIM_SCGC_UART0,
                UartId::Uart1 => reg::SIM_SCGC_UART1,
                UartId::Uart2 => reg::SIM_SCGC_UART2,
            }
        });

        // Stop it receiving or transmitting
        uart.reg.c2.modify(|x| x & !(reg::UART_C2_TE | reg::UART_C2_RE));

        // 8/N/1
        uart.reg.c1.write(0);

        // Set the baud rate
        let baud_div = ((CLOCK_SPEED >> 4) + (baud / 2)) / baud;
        uart.reg.bdh.write((baud_div >> 8) as u8);
        uart.reg.bdl.write((baud_div & 0xFF) as u8);

        // Turn the receiver and transmitter back on
        uart.reg.c2.modify(|x| x | (reg::UART_C2_TE | reg::UART_C2_RE));

        uart
    }
}

impl BlockingTx for Uart {
    type Error = ();

    /// Emit a single octet, first busy-waiting if the data register
    /// is not yet empty.
    /// Never returns `Err`.
    fn putc(&mut self, value: u8) -> Result<(), Self::Error> {
        while (self.reg.s1.read() & reg::UART_S1_TDRE) == 0 {
            nop();
        }
        self.reg.data.write(value);
        Ok(())
    }
}

impl NonBlockingRx for Uart {
    type Error = ();

    /// Attempts to read from the UART. Returns `Err(())`
    /// if the data register isn't full, or `Ok(octet)`.
    fn getc_try(&mut self) -> Result<u8, Self::Error> {
        if (self.reg.s1.read() & reg::UART_S1_RDRF) == 0 {
            Err(())
        } else {
            Ok(self.reg.data.read())
        }
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

/// Get a reference to the UART control register struct in the chip.
fn get_uart_registers(uart_id: UartId) -> &'static mut reg::UartRegisters {
    match uart_id {
        UartId::Uart0 => reg::get_uart0(),
        UartId::Uart1 => reg::get_uart1(),
        UartId::Uart2 => reg::get_uart2(),
    }
}

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
