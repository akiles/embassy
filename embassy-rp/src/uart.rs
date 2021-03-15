use defmt::{assert, *};
use gpio::{sealed::Pin as _, Pin};

use crate::{gpio, pac};

pub struct Uart<T: Instance> {
    inner: T,
}

impl<T: Instance> Uart<T> {
    pub fn new(
        inner: T,
        tx: impl TxPin<T>,
        rx: impl RxPin<T>,
        cts: impl CtsPin<T>,
        rts: impl RtsPin<T>,
        baudrate: u32,
        data_bits: u8,
        stop_bits: u8,
    ) -> Self {
        unsafe {
            let p = inner.regs();

            // todo get this from somewhere
            let clk_base = 12_000_000;

            let baud_rate_div = (8 * clk_base) / baudrate;
            let mut baud_ibrd = baud_rate_div >> 7;
            let mut baud_fbrd = ((baud_rate_div & 0x7f) + 1) / 2;

            if baud_ibrd == 0 {
                baud_ibrd = 1;
                baud_fbrd = 0;
            } else if baud_ibrd >= 65535 {
                baud_ibrd = 65535;
                baud_fbrd = 0;
            }

            // Load PL011's baud divisor registers
            p.uartibrd()
                .write_value(pac::uart::regs::Uartibrd(baud_ibrd));
            p.uartfbrd()
                .write_value(pac::uart::regs::Uartfbrd(baud_fbrd));

            p.uartlcr_h().write(|w| {
                w.set_wlen(data_bits - 5);
                w.set_stp2(stop_bits == 2);
                w.set_pen(false);
                w.set_eps(false);
                w.set_fen(true);
            });

            p.uartcr().write(|w| {
                w.set_uarten(true);
                w.set_rxe(true);
                w.set_txe(true);
            });

            tx.io().ctrl().write(|w| w.set_funcsel(2));
            rx.io().ctrl().write(|w| w.set_funcsel(2));
            cts.io().ctrl().write(|w| w.set_funcsel(2));
            rts.io().ctrl().write(|w| w.set_funcsel(2));
        }
        Self { inner }
    }

    pub fn send(&mut self, data: &[u8]) {
        unsafe {
            let p = self.inner.regs();

            for &byte in data {
                if !p.uartfr().read().txff() {
                    p.uartdr().write(|w| w.set_data(byte));
                }
            }
        }
    }
}

mod sealed {
    pub trait Instance {}
    pub trait TxPin<T: Instance> {}
    pub trait RxPin<T: Instance> {}
    pub trait CtsPin<T: Instance> {}
    pub trait RtsPin<T: Instance> {}
}

pub struct Uart0 {
    pub(crate) _private: (),
}
pub struct Uart1 {
    pub(crate) _private: (),
}

pub trait Instance: sealed::Instance {
    fn regs(&self) -> pac::uart::Uart;
}

impl sealed::Instance for Uart0 {}
impl Instance for Uart0 {
    fn regs(&self) -> pac::uart::Uart {
        pac::UART0
    }
}
impl sealed::Instance for Uart1 {}
impl Instance for Uart1 {
    fn regs(&self) -> pac::uart::Uart {
        pac::UART1
    }
}

pub trait TxPin<T: Instance>: sealed::TxPin<T> + Pin {}
pub trait RxPin<T: Instance>: sealed::RxPin<T> + Pin {}
pub trait CtsPin<T: Instance>: sealed::CtsPin<T> + Pin {}
pub trait RtsPin<T: Instance>: sealed::RtsPin<T> + Pin {}

impl sealed::TxPin<Uart0> for gpio::Gpio0 {}
impl TxPin<Uart0> for gpio::Gpio0 {}
impl sealed::RxPin<Uart0> for gpio::Gpio1 {}
impl RxPin<Uart0> for gpio::Gpio1 {}
impl sealed::CtsPin<Uart0> for gpio::Gpio2 {}
impl CtsPin<Uart0> for gpio::Gpio2 {}
impl sealed::RtsPin<Uart0> for gpio::Gpio3 {}
impl RtsPin<Uart0> for gpio::Gpio3 {}
impl sealed::TxPin<Uart1> for gpio::Gpio4 {}
impl TxPin<Uart1> for gpio::Gpio4 {}
impl sealed::RxPin<Uart1> for gpio::Gpio5 {}
impl RxPin<Uart1> for gpio::Gpio5 {}
impl sealed::CtsPin<Uart1> for gpio::Gpio6 {}
impl CtsPin<Uart1> for gpio::Gpio6 {}
impl sealed::RtsPin<Uart1> for gpio::Gpio7 {}
impl RtsPin<Uart1> for gpio::Gpio7 {}
impl sealed::TxPin<Uart1> for gpio::Gpio8 {}
impl TxPin<Uart1> for gpio::Gpio8 {}
impl sealed::RxPin<Uart1> for gpio::Gpio9 {}
impl RxPin<Uart1> for gpio::Gpio9 {}
impl sealed::CtsPin<Uart1> for gpio::Gpio10 {}
impl CtsPin<Uart1> for gpio::Gpio10 {}
impl sealed::RtsPin<Uart1> for gpio::Gpio11 {}
impl RtsPin<Uart1> for gpio::Gpio11 {}
impl sealed::TxPin<Uart0> for gpio::Gpio12 {}
impl TxPin<Uart0> for gpio::Gpio12 {}
impl sealed::RxPin<Uart0> for gpio::Gpio13 {}
impl RxPin<Uart0> for gpio::Gpio13 {}
impl sealed::CtsPin<Uart0> for gpio::Gpio14 {}
impl CtsPin<Uart0> for gpio::Gpio14 {}
impl sealed::RtsPin<Uart0> for gpio::Gpio15 {}
impl RtsPin<Uart0> for gpio::Gpio15 {}
impl sealed::TxPin<Uart0> for gpio::Gpio16 {}
impl TxPin<Uart0> for gpio::Gpio16 {}
impl sealed::RxPin<Uart0> for gpio::Gpio17 {}
impl RxPin<Uart0> for gpio::Gpio17 {}
impl sealed::CtsPin<Uart0> for gpio::Gpio18 {}
impl CtsPin<Uart0> for gpio::Gpio18 {}
impl sealed::RtsPin<Uart0> for gpio::Gpio19 {}
impl RtsPin<Uart0> for gpio::Gpio19 {}
impl sealed::TxPin<Uart1> for gpio::Gpio20 {}
impl TxPin<Uart1> for gpio::Gpio20 {}
impl sealed::RxPin<Uart1> for gpio::Gpio21 {}
impl RxPin<Uart1> for gpio::Gpio21 {}
impl sealed::CtsPin<Uart1> for gpio::Gpio22 {}
impl CtsPin<Uart1> for gpio::Gpio22 {}
impl sealed::RtsPin<Uart1> for gpio::Gpio23 {}
impl RtsPin<Uart1> for gpio::Gpio23 {}
impl sealed::TxPin<Uart1> for gpio::Gpio24 {}
impl TxPin<Uart1> for gpio::Gpio24 {}
impl sealed::RxPin<Uart1> for gpio::Gpio25 {}
impl RxPin<Uart1> for gpio::Gpio25 {}
impl sealed::CtsPin<Uart1> for gpio::Gpio26 {}
impl CtsPin<Uart1> for gpio::Gpio26 {}
impl sealed::RtsPin<Uart1> for gpio::Gpio27 {}
impl RtsPin<Uart1> for gpio::Gpio27 {}
impl sealed::TxPin<Uart0> for gpio::Gpio28 {}
impl TxPin<Uart0> for gpio::Gpio28 {}
impl sealed::RxPin<Uart0> for gpio::Gpio29 {}
impl RxPin<Uart0> for gpio::Gpio29 {}
