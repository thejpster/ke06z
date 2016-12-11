//! # GPIO for the Freescale KE06Z
//!
//! Supports GPIO, mapping in a UART and putting pins in Timer mode.

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

use core::intrinsics::{volatile_load, volatile_store};

use cortex_m::asm::nop;

use super::registers as reg;
use super::uart::UartId;

// ****************************************************************************
//
// Public Types
//
// ****************************************************************************

/// Describes a pin within a port
/// This chip has 8 pins per port.
///
/// Internally it actually has three 32-bit ports, but
/// we following the pin naming in the documentation, which is for
/// nine 8-bit ports.
#[derive(PartialEq, Clone, Copy)]
#[allow(missing_docs)]
pub enum Pin {
    Pin0,
    Pin1,
    Pin2,
    Pin3,
    Pin4,
    Pin5,
    Pin6,
    Pin7,
}

/// Describes a Port and a single pin within it
#[derive(PartialEq, Clone, Copy)]
#[allow(missing_docs)]
pub enum PinPort {
    PortA(Pin),
    PortB(Pin),
    PortC(Pin),
    PortD(Pin),
    PortE(Pin),
    PortF(Pin),
    PortG(Pin),
    PortH(Pin),
    PortI(Pin),
}

/// Describes a pin's direction
#[derive(PartialEq, Clone, Copy)]
pub enum PinMode {
    /// An input with a pull-up or pull-down
    InputPull(Level),
    /// An input with no pull
    Input,
    /// A totem-pole output
    Output,
    /// Pin is driven by a peripheral (i.e. is no longer a GPIO)
    Peripheral,
}

/// Describes what a pin can be set to
#[derive(PartialEq, Clone, Copy)]
pub enum Level {
    /// A logic high (i.e. 3.3v)
    High,
    /// A logic low (i.e. 0v)
    Low,
}

// ****************************************************************************
//
// Private Types
//
// ****************************************************************************

enum InternalPort {
    /// Ports A, B, C and D
    Internal0,
    /// Ports E, F, G and H
    Internal1,
    /// Port I
    Internal2
}

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************

/// init() is empty for now, but it might be needed some day.
pub fn init() {}

/// Set the direction (input or output) on a given pin in a port
pub fn set_direction(pinport: PinPort, mode: PinMode) {
    match mode {
        PinMode::InputPull(Level::High) => make_input_pullup(pinport),
        PinMode::InputPull(Level::Low) => make_input_pulldown(pinport),
        PinMode::Input => make_input(pinport),
        PinMode::Output => make_output(pinport, Level::Low),
        PinMode::Peripheral => make_peripheral(pinport),
    }
}

/// Set the output value for an output pin
pub fn set(pinport: PinPort, level: Level) {
    let (iport, mask) = get_internal(pinport);
    match level {
        Level::High => iport.sor.write(mask),
        Level::Low => iport.cor.write(mask),
    }
}

/// Read the level of an input pin
pub fn read(pinport: PinPort) -> Level {
    let (iport, mask) = get_internal(pinport);
    if iport.dir.read() & mask == mask {
        Level::High
    } else {
        Level::Low
    }
}

/// Re-configure the pinmuxing so that the given Uart appears
/// on its normal set of pins.
///
/// Only Uart0 is supported at the moment, and it appears on
/// A0 and A1.
pub fn enable_uart(_id: UartId) {}

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

fn get_internal(pinport: PinPort) -> (&'static mut reg::GpioRegisters, u32) {
    let mask = get_pin_mask(pinport);
    match pinport {
        PinPort::PortA(x) => (get_gpio_registers(InternalPort::Internal0), mask),
        PinPort::PortB(x) => (get_gpio_registers(InternalPort::Internal0), mask << 8),
        PinPort::PortC(x) => (get_gpio_registers(InternalPort::Internal0), mask << 16),
        PinPort::PortD(x) => (get_gpio_registers(InternalPort::Internal0), mask << 24),
        PinPort::PortE(x) => (get_gpio_registers(InternalPort::Internal1), mask),
        PinPort::PortF(x) => (get_gpio_registers(InternalPort::Internal1), mask << 8),
        PinPort::PortG(x) => (get_gpio_registers(InternalPort::Internal1), mask << 16),
        PinPort::PortH(x) => (get_gpio_registers(InternalPort::Internal1), mask << 24),
        PinPort::PortI(x) => (get_gpio_registers(InternalPort::Internal2), mask),
    }
}

/// Convert a GPIO port into a reference to the registers which control that port
fn get_gpio_registers(iport: InternalPort) -> &'static mut reg::GpioRegisters {
    unsafe {
        match iport {
            InternalPort::Internal0 => &mut *(reg::GPIO0_BASE as *mut reg::GpioRegisters),
            InternalPort::Internal1 => &mut *(reg::GPIO1_BASE as *mut reg::GpioRegisters),
            InternalPort::Internal2 => &mut *(reg::GPIO2_BASE as *mut reg::GpioRegisters),
        }
    }
}

fn get_port_register() -> &'static mut reg::PortRegisters {
    unsafe {
        &mut *(reg::PORT_BASE as *mut reg::PortRegisters)
    }
}

/// Convert a port to a bit mask
/// Port A is 1, PortF is 32
fn get_port_mask(port: PinPort) -> u32 {
    match port {
        PinPort::PortA(_) => 1 << 0,
        PinPort::PortB(_) => 1 << 1,
        PinPort::PortC(_) => 1 << 2,
        PinPort::PortD(_) => 1 << 3,
        PinPort::PortE(_) => 1 << 4,
        PinPort::PortF(_) => 1 << 5,
        PinPort::PortG(_) => 1 << 6,
        PinPort::PortH(_) => 1 << 7,
        PinPort::PortI(_) => 1 << 8,
    }
}

/// Get the pin from a pinport
fn get_pin_from_pinport(pinport: PinPort) -> Pin {
    match pinport {
        PinPort::PortA(x) => x,
        PinPort::PortB(x) => x,
        PinPort::PortC(x) => x,
        PinPort::PortD(x) => x,
        PinPort::PortE(x) => x,
        PinPort::PortF(x) => x,
        PinPort::PortG(x) => x,
        PinPort::PortH(x) => x,
        PinPort::PortI(x) => x,
    }
}

/// Convert a pin to a bit mask
/// Pin0 is 0, Pin7 is 128
fn get_pin_mask(pinport: PinPort) -> u32 {
    let pin = get_pin_from_pinport(pinport);
    match pin {
        Pin::Pin0 => 1 << 0,
        Pin::Pin1 => 1 << 1,
        Pin::Pin2 => 1 << 2,
        Pin::Pin3 => 1 << 3,
        Pin::Pin4 => 1 << 4,
        Pin::Pin5 => 1 << 5,
        Pin::Pin6 => 1 << 6,
        Pin::Pin7 => 1 << 7,
    }
}

/// Ports don't seem to need enabling here
fn enable_port(_port: PinPort) {}

fn make_input(pinport: PinPort) {
    enable_port(pinport);
    let (iport, mask) = get_internal(pinport);
    iport.ddr.modify(|x| x & !mask);
}

fn make_peripheral(pinport: PinPort) {}

fn make_input_pullup(pinport: PinPort) {
    enable_port(pinport);
    let (iport, mask) = get_internal(pinport);
    iport.ddr.modify(|x| x & !mask);
    let port_reg = get_port_register();
    match pinport {
        PinPort::PortA(_) |
        PinPort::PortB(_) |
        PinPort::PortC(_) |
        PinPort::PortD(_) => port_reg.pue0.modify(|x| x | mask),
        PinPort::PortE(_) |
        PinPort::PortF(_) |
        PinPort::PortG(_) |
        PinPort::PortH(_) => port_reg.pue1.modify(|x| x | mask),
        PinPort::PortI(_) => port_reg.pue2.modify(|x| x | mask),
    }
}

fn make_input_pulldown(pinport: PinPort) {
    enable_port(pinport);
    let (iport, mask) = get_internal(pinport);
    iport.ddr.modify(|x| x & !mask);
    let port_reg = get_port_register();
    match pinport {
        PinPort::PortA(_) |
        PinPort::PortB(_) |
        PinPort::PortC(_) |
        PinPort::PortD(_) => port_reg.pue0.modify(|x| x & !mask),
        PinPort::PortE(_) |
        PinPort::PortF(_) |
        PinPort::PortG(_) |
        PinPort::PortH(_) => port_reg.pue1.modify(|x| x & !mask),
        PinPort::PortI(_) => port_reg.pue2.modify(|x| x & !mask),
    }
}

fn make_output(pinport: PinPort, level: Level) {
    enable_port(pinport);
    let (iport, mask) = get_internal(pinport);
    iport.ddr.modify(|x| x | mask);
}

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
