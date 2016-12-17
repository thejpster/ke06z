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

/// Configure the chip to run in Fully Engaged External mode at 20 MHz.
///
/// It's the only mode we support at the moment. It assumes an 8 MHz crystal.
/// We also set the clock to divide by 2.
pub fn init() {
    let mut ics = reg::get_ics();

    // Initialise oscillator
    osc::init();

    // Set the divider for the external clock to get it in range
    // 20 MHz / 512 = 39062.5 Hz
    ics.c1.modify(|x| (x & !reg::ICS_C1_RDIV) | reg::ics_c1_rdiv(4));

    // Change reference clock to external
    ics.c1.modify(|x| x & !reg::ICS_C1_IREFS);

    // Sleep for a bit
    nop();
    nop();

    // Wait for it to settle...
    while (ics.status.read() & reg::ICS_STATUS_IREFST) != 0 {

    }

    // ...and lock
    while (ics.status.read() & reg::ICS_STATUS_LOCK) == 0 {

    }

    // Set multiplier
    ics.c2.modify(|x| (x & !reg::ICS_C2_BDIV) | reg::ics_c2_bdiv(0));

    // Clear loss of lock sticky bit
    ics.status.modify(|x| x & reg::ICS_STATUS_LOLS);
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
