#![no_std]
#![no_main]

#[allow(unused_extern_crates)]
extern crate panic_abort;

use cortex_m::asm::wfi;
use cortex_m_rt::entry;
use embedded_hal::watchdog::{WatchdogEnable, Watchdog};
use stm32f4xx_hal::{
    rcc::RccExt,
    gpio::GpioExt,
    watchdog::IndependentWatchdog,
    time::U32Ext,
    stm32::{CorePeripherals, Peripherals},
};

use core::fmt::Write;
use cortex_m_semihosting::hio;

mod adc_input;

#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Hello").unwrap();

    let mut cp = CorePeripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();
    stm32_eth::setup(&dp.RCC, &dp.SYSCFG);
    let _clocks = dp.RCC.constrain()
        .cfgr
        .sysclk(168.mhz())
        .hclk(84.mhz())
        .pclk1(32.mhz())
        .pclk2(64.mhz())
        .freeze();

    let mut wd = IndependentWatchdog::new(dp.IWDG);
    wd.start(8000u32.ms());
    wd.feed();

    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    let gpioc = dp.GPIOC.split();
    let gpiog = dp.GPIOG.split();

    stm32_eth::setup_pins(
        gpioa.pa1, gpioa.pa2, gpioa.pa7, gpiob.pb13, gpioc.pc1,
        gpioc.pc4, gpioc.pc5, gpiog.pg11, gpiog.pg13
    );

    adc_input::setup(&mut cp.NVIC, dp.ADC1, gpioa.pa3);

    loop {
        let adc_value = adc_input::read();
        adc_value.map(|adc_value| {
            writeln!(stdout, "pa3: {}mV", adc_value).unwrap();
        });
        wd.feed();
        wfi();
    }
}
