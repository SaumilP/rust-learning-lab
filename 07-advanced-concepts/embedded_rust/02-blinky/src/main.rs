#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    // Get access to device peripherals
    let dp = pac::Peripherals::take().unwrap();

    // Configure the system clock
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.MHz()).freeze();

    // Get GPIO bank A
    let gpioa = dp.GPIOA.split();

    // Configure PA5 as output (LED on Nucleo boards)
    let mut led = gpioa.pa5.into_push_pull_output();

    // Get delay provider
    let mut delay = dp.TIM1.delay_ms(&clocks);

    // Blink forever
    loop {
        led.set_high();
        delay.delay_ms(1000u32);

        led.set_low();
        delay.delay_ms(1000u32);
    }
}
