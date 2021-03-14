#![no_std]
#![feature(generic_associated_types)]
#![feature(asm)]
#![feature(type_alias_impl_trait)]
#![feature(never_type)]

pub use rp2040_pac2 as pac;

// This mod MUST go first, so that the others see its macros.
pub(crate) mod fmt;
pub(crate) mod util;

pub mod interrupt;

pub mod gpio;
pub mod init;
pub mod pll;
pub mod resets;
pub mod uart;

pub struct Peripherals {
    pub gpio0: gpio::Gpio0,
    pub gpio1: gpio::Gpio1,
    pub gpio2: gpio::Gpio2,
    pub gpio3: gpio::Gpio3,
    pub gpio4: gpio::Gpio4,
    pub gpio5: gpio::Gpio5,
    pub gpio6: gpio::Gpio6,
    pub gpio7: gpio::Gpio7,
    pub gpio8: gpio::Gpio8,
    pub gpio9: gpio::Gpio9,
    pub gpio10: gpio::Gpio10,
    pub gpio11: gpio::Gpio11,
    pub gpio12: gpio::Gpio12,
    pub gpio13: gpio::Gpio13,
    pub gpio14: gpio::Gpio14,
    pub gpio15: gpio::Gpio15,
    pub gpio16: gpio::Gpio16,
    pub gpio17: gpio::Gpio17,
    pub gpio18: gpio::Gpio18,
    pub gpio19: gpio::Gpio19,
    pub gpio20: gpio::Gpio20,
    pub gpio21: gpio::Gpio21,
    pub gpio22: gpio::Gpio22,
    pub gpio23: gpio::Gpio23,
    pub gpio24: gpio::Gpio24,
    pub gpio25: gpio::Gpio25,
    pub gpio26: gpio::Gpio26,
    pub gpio27: gpio::Gpio27,
    pub gpio28: gpio::Gpio28,
    pub gpio29: gpio::Gpio29,
    pub qspi_sclk: gpio::QspiSclk,
    pub qspi_ss: gpio::QspiSs,
    pub qspi_sd0: gpio::QspiSd0,
    pub qspi_sd1: gpio::QspiSd1,
    pub qspi_sd2: gpio::QspiSd2,
    pub qspi_sd3: gpio::QspiSd3,

    pub uart0: uart::Uart0,
    pub uart1: uart::Uart1,
}

impl Peripherals {
    pub unsafe fn steal() -> Self {
        Self {
            gpio0: gpio::Gpio0 { _private: () },
            gpio1: gpio::Gpio1 { _private: () },
            gpio2: gpio::Gpio2 { _private: () },
            gpio3: gpio::Gpio3 { _private: () },
            gpio4: gpio::Gpio4 { _private: () },
            gpio5: gpio::Gpio5 { _private: () },
            gpio6: gpio::Gpio6 { _private: () },
            gpio7: gpio::Gpio7 { _private: () },
            gpio8: gpio::Gpio8 { _private: () },
            gpio9: gpio::Gpio9 { _private: () },
            gpio10: gpio::Gpio10 { _private: () },
            gpio11: gpio::Gpio11 { _private: () },
            gpio12: gpio::Gpio12 { _private: () },
            gpio13: gpio::Gpio13 { _private: () },
            gpio14: gpio::Gpio14 { _private: () },
            gpio15: gpio::Gpio15 { _private: () },
            gpio16: gpio::Gpio16 { _private: () },
            gpio17: gpio::Gpio17 { _private: () },
            gpio18: gpio::Gpio18 { _private: () },
            gpio19: gpio::Gpio19 { _private: () },
            gpio20: gpio::Gpio20 { _private: () },
            gpio21: gpio::Gpio21 { _private: () },
            gpio22: gpio::Gpio22 { _private: () },
            gpio23: gpio::Gpio23 { _private: () },
            gpio24: gpio::Gpio24 { _private: () },
            gpio25: gpio::Gpio25 { _private: () },
            gpio26: gpio::Gpio26 { _private: () },
            gpio27: gpio::Gpio27 { _private: () },
            gpio28: gpio::Gpio28 { _private: () },
            gpio29: gpio::Gpio29 { _private: () },
            qspi_sclk: gpio::QspiSclk { _private: () },
            qspi_ss: gpio::QspiSs { _private: () },
            qspi_sd0: gpio::QspiSd0 { _private: () },
            qspi_sd1: gpio::QspiSd1 { _private: () },
            qspi_sd2: gpio::QspiSd2 { _private: () },
            qspi_sd3: gpio::QspiSd3 { _private: () },
            uart0: uart::Uart0 { _private: () },
            uart1: uart::Uart1 { _private: () },
        }
    }
}
