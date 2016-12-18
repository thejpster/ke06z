//! # Internal Clock Source driver for the KE06Z

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

use cortex_m::asm::nop;
use super::registers as reg;
use super::osc;

// ****************************************************************************
//
// Public Types
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

/// Configure the chip to run in Fully Engaged External mode at 40 MHz.
///
/// It's the only mode we support at the moment. It assumes an 8 MHz crystal.
/// We also set the clock to divide by 2.
///
/// This sequence is taken from the data sheet section 20.5.3.1
pub fn init() {
    let mut ics = reg::get_ics();
    let mut sim = reg::get_sim();

    // Initialise oscillator
    osc::init();

    // bdiv = divide by 2
    ics.c2.write(0x20);

    // Set the divider for the external clock to get it in range
    ics.c1.write(0x18);

    // Sleep for a bit
    nop();
    nop();

    // Wait for it to settle...
    while (ics.status.read() & reg::ICS_STATUS_IREFST) != 0 {

    }

    // ...and lock
    while (ics.status.read() & reg::ICS_STATUS_LOCK) == 0 {

    }

    // core clock = ICSOUT/1; bus clock = core clock / 2
    sim.clkdiv.write(0x01100000);

    // bdiv = divide by 1
    ics.c2.write(0x00);
}

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
