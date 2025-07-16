#![no_std]
#![no_main]

use cortex_m_rt::entry;
use teensy4_bsp as bsp;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let peripherals = bsp::hal::Peripherals::take().unwrap();
    let mut pins = bsp::t40::into_pins(peripherals.iomuxc);

    let mut led = bsp::hal::gpio::GPIO::new(pins.p13).output(); // Teensy 4.0 onboard LED

    let mut delay = bsp::hal::timer::Blocking::<_, bsp::hal::timer::RollingTimer>::from_pit(
        peripherals.pit,
        &mut peripherals.ccm.handle,
    );

    loop {
        led.toggle();
        delay.delay_ms(500u32);
    }
