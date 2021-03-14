#![no_std]
#![no_main]
#![feature(asm)]
#![feature(type_alias_impl_trait)]

use core::mem;
use core::sync::atomic::{AtomicUsize, Ordering};
use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use embassy::executor::{task, Executor};
use embassy::util::Forever;
use embassy_rp::{gpio, uart, Peripherals};
use embedded_hal::digital::v2::OutputPin;
use gpio::Gpio25;
use panic_probe as _;
use rp2040_pac2 as pac;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER;

defmt::timestamp! {"{=u64}", {
    static COUNT: AtomicUsize = AtomicUsize::new(0);
    // NOTE(no-CAS) `timestamps` runs with interrupts disabled
    let n = COUNT.load(Ordering::Relaxed);
    COUNT.store(n + 1, Ordering::Relaxed);
    n as u64
}
}

#[task]
async fn run() {
    let p = unsafe { Peripherals::steal() };

    let mut uart = uart::Uart::new(p.uart0, p.gpio0, p.gpio1, p.gpio2, p.gpio3, 115200, 8, 1);
    uart.send("Hello World!\r\n".as_bytes());

    let mut led = gpio::Output::new(p.gpio25, gpio::Level::Low);

    loop {
        info!("led on!");
        uart.send("ON!\r".as_bytes());
        led.set_high().unwrap();
        cortex_m::asm::delay(1_000_000);

        info!("led off!");
        uart.send("Off!\r".as_bytes());
        led.set_low().unwrap();
        cortex_m::asm::delay(4_000_000);
    }
}

static EXECUTOR: Forever<Executor> = Forever::new();

#[entry]
fn main() -> ! {
    info!("Hello World!");

    unsafe { embassy_rp::init::init() };

    let executor = EXECUTOR.put(Executor::new());
    executor.run(|spawner| {
        unwrap!(spawner.spawn(run()));
    });
}
