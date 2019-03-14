#![no_std]
#![no_main]
// Enable returning `!`
#![feature(never_type)]

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
use smoltcp::time::Instant;

use core::fmt::Write;
use cortex_m_semihosting::hio;

mod adc_input;
mod net;
mod server;
use server::Server;
mod timer;

const OUTPUT_INTERVAL: u32 = 1000;

#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "adc2tcp").unwrap();

    let mut cp = CorePeripherals::take().unwrap();
    cp.SCB.enable_icache();
    cp.SCB.enable_dcache(&mut cp.CPUID);

    let dp = Peripherals::take().unwrap();
    stm32_eth::setup(&dp.RCC, &dp.SYSCFG);
    let clocks = dp.RCC.constrain()
        .cfgr
        .sysclk(84.mhz())
        .hclk(84.mhz())
        .pclk1(16.mhz())
        .pclk2(32.mhz())
        .freeze();

    let mut wd = IndependentWatchdog::new(dp.IWDG);
    wd.start(8000u32.ms());
    wd.feed();

    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    let gpioc = dp.GPIOC.split();
    let gpiog = dp.GPIOG.split();

    writeln!(stdout, "ADC init").unwrap();
    adc_input::setup(&mut cp.NVIC, dp.ADC1, gpioa.pa3);

    writeln!(stdout, "Eth setup").unwrap();
    stm32_eth::setup_pins(
        gpioa.pa1, gpioa.pa2, gpioa.pa7, gpiob.pb13, gpioc.pc1,
        gpioc.pc4, gpioc.pc5, gpiog.pg11, gpiog.pg13
    );

    writeln!(stdout, "Timer setup").unwrap();
    timer::setup(cp.SYST, clocks);

    writeln!(stdout, "Net startup").unwrap();
    net::run(&mut cp.NVIC, dp.ETHERNET_MAC, dp.ETHERNET_DMA, |net| {
        let mut server = Server::new(net);

        let mut last_output = 0_u32;
        loop {
            let now = timer::now().0;
            let instant = Instant::from_millis(now as i64);
            server.poll(instant);

            if now - last_output >= OUTPUT_INTERVAL {
                let adc_value = adc_input::read();
                adc_value.map(|adc_value| {
                    write!(server, "t={},pa3={}\r\n", now, adc_value).unwrap();
                });
                last_output = now;
            }

            // Update watchdog
            wd.feed();
            // Wait for interrupts
            // if net.is_pending() {
                wfi();
            // }
        }
    })
}
