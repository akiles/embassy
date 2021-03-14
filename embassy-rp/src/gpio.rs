use crate::pac;
use crate::pac::generic::{Reg, R, RW, W};
use crate::pac::SIO;

use embedded_hal::digital::v2::{InputPin, OutputPin, StatefulOutputPin};

// TODO this trait should be sealed
pub trait Pin {
    fn pin_bank(&self) -> u8;
}

pub struct AnyPin {
    pin_bank: u8,
}

impl Pin for AnyPin {
    fn pin_bank(&self) -> u8 {
        self.pin_bank
    }
}

macro_rules! gpio {
    ($name:ident, $bank:expr, $pin_num:expr) => {
        pub struct $name {
            pub(crate) _private: (),
        }

        impl Pin for $name {
            fn pin_bank(&self) -> u8 {
                ($bank as u8) * 32 + $pin_num
            }
        }
    };
}

gpio!(Gpio0, Bank::Bank0, 0);
gpio!(Gpio1, Bank::Bank0, 1);
gpio!(Gpio2, Bank::Bank0, 2);
gpio!(Gpio3, Bank::Bank0, 3);
gpio!(Gpio4, Bank::Bank0, 4);
gpio!(Gpio5, Bank::Bank0, 5);
gpio!(Gpio6, Bank::Bank0, 6);
gpio!(Gpio7, Bank::Bank0, 7);
gpio!(Gpio8, Bank::Bank0, 8);
gpio!(Gpio9, Bank::Bank0, 9);
gpio!(Gpio10, Bank::Bank0, 10);
gpio!(Gpio11, Bank::Bank0, 11);
gpio!(Gpio12, Bank::Bank0, 12);
gpio!(Gpio13, Bank::Bank0, 13);
gpio!(Gpio14, Bank::Bank0, 14);
gpio!(Gpio15, Bank::Bank0, 15);
gpio!(Gpio16, Bank::Bank0, 16);
gpio!(Gpio17, Bank::Bank0, 17);
gpio!(Gpio18, Bank::Bank0, 18);
gpio!(Gpio19, Bank::Bank0, 19);
gpio!(Gpio20, Bank::Bank0, 20);
gpio!(Gpio21, Bank::Bank0, 21);
gpio!(Gpio22, Bank::Bank0, 22);
gpio!(Gpio23, Bank::Bank0, 23);
gpio!(Gpio24, Bank::Bank0, 24);
gpio!(Gpio25, Bank::Bank0, 25);
gpio!(Gpio26, Bank::Bank0, 26);
gpio!(Gpio27, Bank::Bank0, 27);
gpio!(Gpio28, Bank::Bank0, 28);
gpio!(Gpio29, Bank::Bank0, 29);

gpio!(QspiSclk, Bank::Qspi, 0);
gpio!(QspiSs, Bank::Qspi, 1);
gpio!(QspiSd0, Bank::Qspi, 2);
gpio!(QspiSd1, Bank::Qspi, 3);
gpio!(QspiSd2, Bank::Qspi, 4);
gpio!(QspiSd3, Bank::Qspi, 5);

/// Represents a digital input or output level.
#[derive(Debug, Eq, PartialEq)]
pub enum Level {
    Low,
    High,
}

/// Represents a pull setting for an input.
#[derive(Debug, Eq, PartialEq)]
pub enum Pull {
    None,
    Up,
    Down,
}

/// A GPIO bank with up to 32 pins.
#[derive(Debug, Eq, PartialEq)]
pub enum Bank {
    Bank0 = 0,
    Qspi = 1,
}

// TODO: If Pin is sealed, maybe these can be default methods in Pin instead.
// TODO: some of these shouldn't be public
pub trait PinExt {
    fn pin(&self) -> u8;
    fn bank(&self) -> Bank;

    // TODO: should these take &mut self and/or be unsafe?
    fn io(&self) -> pac::io::Gpio;
    fn pad_ctrl(&self) -> Reg<pac::pads::regs::GpioCtrl, RW>;
    fn sio_out(&self) -> pac::sio::Gpio;
    fn sio_oe(&self) -> pac::sio::Gpio;
    fn sio_in(&self) -> Reg<u32, RW>;

    /// Degrade to a generic pin struct, which can be used with peripherals
    fn degrade(self) -> AnyPin;
}

impl<T: Pin> PinExt for T {
    #[inline]
    fn pin(&self) -> u8 {
        self.pin_bank() & 0x1f
    }

    #[inline]
    fn bank(&self) -> Bank {
        if self.pin_bank() & 0x20 == 0 {
            Bank::Bank0
        } else {
            Bank::Qspi
        }
    }

    fn io(&self) -> pac::io::Gpio {
        let block = match self.bank() {
            Bank::Bank0 => crate::pac::IO_BANK0,
            Bank::Qspi => crate::pac::IO_QSPI,
        };
        block.gpio(self.pin() as _)
    }

    fn pad_ctrl(&self) -> Reg<pac::pads::regs::GpioCtrl, RW> {
        let block = match self.bank() {
            Bank::Bank0 => crate::pac::PADS_BANK0,
            Bank::Qspi => crate::pac::PADS_QSPI,
        };
        block.gpio(self.pin() as _)
    }
    fn sio_out(&self) -> pac::sio::Gpio {
        SIO.gpio_out(self.bank() as _)
    }
    fn sio_oe(&self) -> pac::sio::Gpio {
        SIO.gpio_oe(self.bank() as _)
    }
    fn sio_in(&self) -> Reg<u32, RW> {
        SIO.gpio_in(self.bank() as _)
    }

    fn degrade(self) -> AnyPin {
        AnyPin {
            pin_bank: self.pin_bank(),
        }
    }
}

pub struct Input<T: Pin> {
    pin: T,
}

impl<T: Pin> Input<T> {
    pub fn new(pin: T, pull: Pull) -> Self {
        // todo

        Self { pin }
    }
}

impl<T: Pin> Drop for Input<T> {
    fn drop(&mut self) {
        // todo
    }
}

impl<T: Pin> InputPin for Input<T> {
    type Error = !;

    fn is_high(&self) -> Result<bool, Self::Error> {
        self.is_low().map(|v| !v)
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        // todo
        Ok(true)
    }
}

pub struct Output<T: Pin> {
    pin: T,
}

impl<T: Pin> Output<T> {
    // TODO opendrain
    pub fn new(pin: T, initial_output: Level) -> Self {
        // todo
        unsafe {
            match initial_output {
                Level::High => pin.sio_out().value_set().write_value(1 << pin.pin()),
                Level::Low => pin.sio_out().value_clr().write_value(1 << pin.pin()),
            }
            pin.sio_oe().value_set().write_value(1 << pin.pin());

            pin.io().ctrl().write(|w| {
                w.set_funcsel(pac::io::vals::Gpio0CtrlFuncsel::SIO_0.0);
            });
        }

        Self { pin }
    }
}

impl<T: Pin> Drop for Output<T> {
    fn drop(&mut self) {
        // todo
    }
}

impl<T: Pin> OutputPin for Output<T> {
    type Error = !;

    /// Set the output as high.
    fn set_high(&mut self) -> Result<(), Self::Error> {
        let val = 1 << self.pin.pin();
        unsafe { self.pin.sio_out().value_set().write_value(val) };
        Ok(())
    }

    /// Set the output as low.
    fn set_low(&mut self) -> Result<(), Self::Error> {
        let val = 1 << self.pin.pin();
        unsafe { self.pin.sio_out().value_clr().write_value(val) };
        Ok(())
    }
}

impl<T: Pin> StatefulOutputPin for Output<T> {
    /// Is the output pin set as high?
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        self.is_set_low().map(|v| !v)
    }

    /// Is the output pin set as low?
    fn is_set_low(&self) -> Result<bool, Self::Error> {
        // todo
        Ok(true)
    }
}
