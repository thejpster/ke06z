//! Modules specific to the TI LM4F120H5QR Cortex-M4F microcontroller

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

pub mod gpio;
pub mod pll;
pub mod registers;
pub mod uart;
pub mod timer;
pub mod systick;

extern "C" {
    fn _stack_top();
}

use board;

pub use cpu::cortex_m4f::fpu as fpu;

// ****************************************************************************
//
// Public Types
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
// Public Data
//
// ****************************************************************************

#[link_section=".nvic_table"]
#[no_mangle]
pub static ISR_VECTORS: [Option<unsafe extern "C" fn()>; 155] = [// Stack pointer
                                                                 Some(_stack_top),
                                                                 // Reset
                                                                 Some(::common::startup::startup),
                                                                 // NMI
                                                                 Some(isr_nmi),
                                                                 // Hard Fault
                                                                 Some(isr_hardfault),
                                                                 // CM3 Memory Management Fault
                                                                 Some(isr_mmfault),
                                                                 // CM3 Bus Fault
                                                                 Some(isr_busfault),
                                                                 // CM3 Usage Fault
                                                                 Some(isr_usagefault),
                                                                 // Reserved - Used as NXP Checksum
                                                                 None,
                                                                 // Reserved
                                                                 None,
                                                                 // Reserved
                                                                 None,
                                                                 // Reserved
                                                                 None,
                                                                 // SVCall
                                                                 Some(isr_svcall),
                                                                 // Reserved for debug
                                                                 Some(isr_debugmon),
                                                                 // Reserved
                                                                 None,
                                                                 // PendSV
                                                                 Some(isr_pendsv),
                                                                 // SysTick
                                                                 Some(isr_systick),
                                                                 // GPIO Port A                      16
                                                                 Some(isr_empty_def),
                                                                 // GPIO Port B                      17
                                                                 Some(isr_empty_def),
                                                                 // GPIO Port C                      18
                                                                 Some(isr_empty_def),
                                                                 // GPIO Port D                      19
                                                                 Some(isr_empty_def),
                                                                 // GPIO Port E                      20
                                                                 Some(isr_empty_def),
                                                                 // UART 0                           21
                                                                 Some(uart::uart0_isr),
                                                                 // UART 1                           22
                                                                 Some(isr_empty_def),
                                                                 // SSI 0                            23
                                                                 Some(isr_empty_def),
                                                                 // I2C 0                            24
                                                                 Some(isr_empty_def),
                                                                 // Reserved                         25
                                                                 None,
                                                                 // Reserved                         26
                                                                 None,
                                                                 // Reserved                         27
                                                                 None,
                                                                 // Reserved                         28
                                                                 None,
                                                                 // Reserved                         29
                                                                 None,
                                                                 // ADC 0 Seq 0                      30
                                                                 Some(isr_empty_def),
                                                                 // ADC 0 Seq 1                      31
                                                                 Some(isr_empty_def),
                                                                 // ADC 0 Seq 2                      32
                                                                 Some(isr_empty_def),
                                                                 // ADC 0 Seq 3                      33
                                                                 Some(isr_empty_def),
                                                                 // WDT 0 and 1                      34
                                                                 Some(isr_empty_def),
                                                                 // 16/32 bit timer 0 A              35
                                                                 Some(isr_empty_def),
                                                                 // 16/32 bit timer 0 B              36
                                                                 Some(isr_empty_def),
                                                                 // 16/32 bit timer 1 A              37
                                                                 Some(isr_empty_def),
                                                                 // 16/32 bit timer 1 B              38
                                                                 Some(isr_empty_def),
                                                                 // 16/32 bit timer 2 A              39
                                                                 Some(isr_empty_def),
                                                                 // 16/32 bit timer 2 B              40
                                                                 Some(isr_empty_def),
                                                                 // Analog comparator 0              41
                                                                 Some(isr_empty_def),
                                                                 // Analog comparator 1              42
                                                                 Some(isr_empty_def),
                                                                 // Reserved                         43
                                                                 None,
                                                                 // System control                   44
                                                                 Some(isr_empty_def),
                                                                 // Flash + EEPROM control           45
                                                                 Some(isr_empty_def),
                                                                 // GPIO Port F                      46
                                                                 Some(isr_empty_def),
                                                                 // Reserved                         47
                                                                 None,
                                                                 // Reserved                         48
                                                                 None,
                                                                 // UART 2                           49
                                                                 Some(isr_empty_def),
                                                                 // SSI 1                            50
                                                                 Some(isr_empty_def),
                                                                 // 16/32 bit timer 3 A              51
                                                                 Some(isr_empty_def),
                                                                 // 16/32 bit timer 3 B              52
                                                                 Some(isr_empty_def),
                                                                 // I2C 1                            53
                                                                 Some(isr_empty_def),
                                                                 // Reserved                         54
                                                                 None,
                                                                 // CAN 0                            55
                                                                 Some(isr_empty_def),
                                                                 // Reserved                         56
                                                                 None,
                                                                 // Reserved                         57
                                                                 None,
                                                                 // Reserved                         58
                                                                 None,
                                                                 // Hibernation module               59
                                                                 Some(isr_empty_def),
                                                                 // USB                              60
                                                                 Some(isr_empty_def),
                                                                 // Reserved                         61
                                                                 None,
                                                                 // UDMA SW                          62
                                                                 Some(isr_empty_def),
                                                                 // UDMA Error                       63
                                                                 Some(isr_empty_def),
                                                                 // ADC 1 Seq 0                      64
                                                                 Some(isr_empty_def),
                                                                 // ADC 1 Seq 1                      65
                                                                 Some(isr_empty_def),
                                                                 // ADC 1 Seq 2                      66
                                                                 Some(isr_empty_def),
                                                                 // ADC 1 Seq 3                      67
                                                                 Some(isr_empty_def),
                                                                 // Reserved                         68
                                                                 None,
                                                                 // Reserved                         69
                                                                 None,
                                                                 // Reserved                         70
                                                                 None,
                                                                 // Reserved                         71
                                                                 None,
                                                                 // Reserved                         72
                                                                 None,
                                                                 // SSI 2                            73
                                                                 Some(isr_empty_def),
                                                                 // SSI 2                            74
                                                                 Some(isr_empty_def),
                                                                 // UART 3                           75
                                                                 Some(isr_empty_def),
                                                                 // UART 4                           76
                                                                 Some(isr_empty_def),
                                                                 // UART 5                           77
                                                                 Some(isr_empty_def),
                                                                 // UART 6                           78
                                                                 Some(isr_empty_def),
                                                                 // UART 7                           79
                                                                 Some(isr_empty_def),
                                                                 // Reserved                         80
                                                                 None,
                                                                 // Reserved                         81
                                                                 None,
                                                                 // Reserved                         82
                                                                 None,
                                                                 // Reserved                         83
                                                                 None,
                                                                 // I2C 2                            84
                                                                 Some(isr_empty_def),
                                                                 // I2C 4                            85
                                                                 Some(isr_empty_def),
                                                                 // 16/32 bit timer 4 A              86
                                                                 Some(isr_empty_def),
                                                                 // 16/32 bit timer 4 B              87
                                                                 Some(isr_empty_def),
                                                                 // Reserved                         88
                                                                 None,
                                                                 // Reserved                         89
                                                                 None,
                                                                 // Reserved                         90
                                                                 None,
                                                                 // Reserved                         91
                                                                 None,
                                                                 // Reserved                         92
                                                                 None,
                                                                 // Reserved                         93
                                                                 None,
                                                                 // Reserved                         94
                                                                 None,
                                                                 // Reserved                         95
                                                                 None,
                                                                 // Reserved                         96
                                                                 None,
                                                                 // Reserved                         97
                                                                 None,
                                                                 // Reserved                         98
                                                                 None,
                                                                 // Reserved                         99
                                                                 None,
                                                                 // Reserved                         100
                                                                 None,
                                                                 // Reserved                         101
                                                                 None,
                                                                 // Reserved                         102
                                                                 None,
                                                                 // Reserved                         103
                                                                 None,
                                                                 // Reserved                         104
                                                                 None,
                                                                 // Reserved                         105
                                                                 None,
                                                                 // Reserved                         106
                                                                 None,
                                                                 // Reserved                         107
                                                                 None,
                                                                 // 16/32 bit timer 5 A              108
                                                                 Some(isr_empty_def),
                                                                 // 16/32 bit timer 5 B              109
                                                                 Some(isr_empty_def),
                                                                 // 32/64 bit timer 0 A              110
                                                                 Some(isr_empty_def),
                                                                 // 32/64 bit timer 0 B              111
                                                                 Some(isr_empty_def),
                                                                 // 32/64 bit timer 1 A              112
                                                                 Some(isr_empty_def),
                                                                 // 32/64 bit timer 1 B              113
                                                                 Some(isr_empty_def),
                                                                 // 32/64 bit timer 2 A              114
                                                                 Some(isr_empty_def),
                                                                 // 32/64 bit timer 2 B              115
                                                                 Some(isr_empty_def),
                                                                 // 32/64 bit timer 3 A              116
                                                                 Some(isr_empty_def),
                                                                 // 32/64 bit timer 3 B              117
                                                                 Some(isr_empty_def),
                                                                 // 32/64 bit timer 4 A              118
                                                                 Some(isr_empty_def),
                                                                 // 32/64 bit timer 4 B              119
                                                                 Some(isr_empty_def),
                                                                 // 32/64 bit timer 5 A              120
                                                                 Some(isr_empty_def),
                                                                 // 32/64 bit timer 5 B              121
                                                                 Some(isr_empty_def),
                                                                 // System Exception                 122
                                                                 Some(isr_empty_def),
                                                                 // Reserved                         123
                                                                 None,
                                                                 // Reserved                         124
                                                                 None,
                                                                 // Reserved                         125
                                                                 None,
                                                                 // Reserved                         126
                                                                 None,
                                                                 // Reserved                         127
                                                                 None,
                                                                 // Reserved                         128
                                                                 None,
                                                                 // Reserved                         129
                                                                 None,
                                                                 // Reserved                         130
                                                                 None,
                                                                 // Reserved                         131
                                                                 None,
                                                                 // Reserved                         132
                                                                 None,
                                                                 // Reserved                         133
                                                                 None,
                                                                 // Reserved                         134
                                                                 None,
                                                                 // Reserved                         135
                                                                 None,
                                                                 // Reserved                         136
                                                                 None,
                                                                 // Reserved                         137
                                                                 None,
                                                                 // Reserved                         138
                                                                 None,
                                                                 // Reserved                         139
                                                                 None,
                                                                 // Reserved                         140
                                                                 None,
                                                                 // Reserved                         141
                                                                 None,
                                                                 // Reserved                         142
                                                                 None,
                                                                 // Reserved                         143
                                                                 None,
                                                                 // Reserved                         144
                                                                 None,
                                                                 // Reserved                         145
                                                                 None,
                                                                 // Reserved                         146
                                                                 None,
                                                                 // Reserved                         147
                                                                 None,
                                                                 // Reserved                         148
                                                                 None,
                                                                 // Reserved                         149
                                                                 None,
                                                                 // Reserved                         150
                                                                 None,
                                                                 // Reserved                         151
                                                                 None,
                                                                 // Reserved                         152
                                                                 None,
                                                                 // Reserved                         153
                                                                 None,
                                                                 // Reserved                         154
                                                                 None];

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************

/// Busy-waits for the given period.
///
/// Currently this function spins with a empirical number
/// of NOPS per millisecond. It should really use a timer.
///
/// * `ms` - The period to wait, in milliseconds
pub fn delay(ms: i32) {
    for _ in 0..ms * 250 {
        unsafe {
            asm!("NOP");
        }
    }
}

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

/// A Non Maskable Interrupt (NMI) can be signalled by a peripheral or
/// triggered by software. This is the highest priority exception other than
/// reset. It is permanently enabled and has a fixed priority of -2. NMIs
/// cannot be:
/// * masked or prevented from activation by any other exception
/// * preempted by any exception other than Reset.
#[no_mangle]
pub unsafe extern "C" fn isr_nmi() {
    // Do nothing
}

/// A HardFault is an exception that occurs because of an error during
/// exception processing, or because an exception cannot be managed by any
/// other exception mechanism. HardFaults have a fixed priority of -1, meaning
/// they have higher priority than any exception with configurable priority.
#[no_mangle]
pub unsafe extern "C" fn isr_hardfault() {

     // Todo see https://community.arm.com/servlet/JiveServlet/previewBody/7835-102-2-12371/Developing%20a%20Generic%20Hard%20Fault%20handler%20for%20ARM.pdf
     // Use ITM to emit serial chars and then bkpt into debugger
     bkpt!();
     board::panic();
}

/// A MemManage fault is an exception that occurs because of a memory
/// protection related fault. The the fixed memory protection constraints
/// determines this fault, for both instruction and data memory transactions.
/// This fault is always used to abort instruction accesses to Execute Never
/// (XN) memory regions.
#[no_mangle]
pub unsafe extern "C" fn isr_mmfault() {
    board::panic();
}

/// A BusFault is an exception that occurs because of a memory related fault
/// for an instruction or data memory transaction. This might be from an error
/// detected on a bus in the memory system.
#[no_mangle]
pub unsafe extern "C" fn isr_busfault() {
    board::panic();
}

/// A UsageFault is an exception that occurs because of a fault related to instruction execution. This includes:
/// * an undefined instruction
/// * an illegal unaligned access
/// * invalid state on instruction execution
/// * an error on exception return.
/// The following can cause a UsageFault when the core is configured to report them:
/// * an unaligned address on word and halfword memory access
/// * division by zero.
#[no_mangle]
pub unsafe extern "C" fn isr_usagefault() {
    board::panic();
}

/// A supervisor call (SVC) is an exception that is triggered by the SVC
/// instruction. In an OS environment, applications can use SVC instructions
/// to access OS kernel functions and device drivers.
#[no_mangle]
pub unsafe extern "C" fn isr_svcall() {
    // Nothing
}

/// Debug monitor interrupt handler.
#[no_mangle]
pub unsafe extern "C" fn isr_debugmon() {
    // Nothing
}

/// PendSV is an interrupt-driven request for system-level service. In an OS
/// environment, use PendSV for context switching when no other exception is
/// active.
#[no_mangle]
pub unsafe extern "C" fn isr_pendsv() {
    // Nothing
}

/// A SysTick exception is an exception the system timer generates when it
/// reaches zero. Software can also generate a SysTick exception. In an OS
/// environment, the processor can use this exception as system tick.
#[no_mangle]
pub unsafe extern "C" fn isr_systick() {
    systick::isr();
}

/// A place-holder ISR used when we have nothing better to use.
#[no_mangle]
pub unsafe extern "C" fn isr_empty_def() {
    board::panic();
}

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
