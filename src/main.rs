#![no_std]
#![no_main]
#![deny(warnings, unused)]

#[allow(unused_extern_crates)]
#[cfg(not(feature = "semihosting"))]
extern crate panic_abort;
#[cfg(feature = "semihosting")]
extern crate panic_semihosting;

#[macro_use]
extern crate log;

use core::fmt::Write;
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

mod adc_input;
use adc_input::AdcInput;
mod net;
mod server;
use server::Server;
mod timer;
mod led;
use led::Led;

/// Interval at which to sample the ADC input and broadcast to all
/// clients.
///
/// This should be a multiple of the `TIMER_RATE`.
const OUTPUT_INTERVAL: u32 = 1000;

#[cfg(not(feature = "semihosting"))]
fn init_log() {}

#[cfg(feature = "semihosting")]
fn init_log() {
    use log::LevelFilter;
    use cortex_m_log::log::{Logger, init};
    use cortex_m_log::printer::semihosting::{InterruptOk, hio::HStdout};
    static mut LOGGER: Option<Logger<InterruptOk<HStdout>>> = None;
    let logger = Logger {
        inner: InterruptOk::<_>::stdout().expect("semihosting stdout"),
        level: LevelFilter::Info,
    };
    let logger = unsafe {
        LOGGER.get_or_insert(logger)
    };

    init(logger).expect("set logger");
}

/// Initialization and main loop
#[entry]
fn main() -> ! {
    init_log();
    info!("adc2tcp");

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
    wd.start(1000u32.ms());
    wd.feed();

    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    let gpioc = dp.GPIOC.split();
    let gpiog = dp.GPIOG.split();

    let mut led_green = Led::green(gpiob.pb0.into_push_pull_output());
    let mut led_blue = Led::blue(gpiob.pb7.into_push_pull_output());
    let mut led_red = Led::red(gpiob.pb14.into_push_pull_output());

    info!("ADC init");
    let mut adc_input = AdcInput::new(dp.ADC1, gpioa.pa3);

    info!("Eth setup");
    stm32_eth::setup_pins(
        gpioa.pa1, gpioa.pa2, gpioa.pa7, gpiob.pb13, gpioc.pc1,
        gpioc.pc4, gpioc.pc5, gpiog.pg11, gpiog.pg13
    );

    info!("Timer setup");
    timer::setup(cp.SYST, clocks);

    info!("Net startup");
    net::run(&mut cp.NVIC, dp.ETHERNET_MAC, dp.ETHERNET_DMA, |iface| {
        Server::run(iface, |server| {
            let mut last_output = 0_u32;
            loop {
                let now = timer::now().0;
                let instant = Instant::from_millis(now as i64);
                led_blue.on();
                cortex_m::interrupt::free(net::clear_pending);
                server.poll(instant)
                    .unwrap_or_else(|e| {
                        warn!("poll: {:?}", e);
                    });
                led_blue.off();

                let now = timer::now().0;
                if now - last_output >= OUTPUT_INTERVAL {
                    led_red.on();
                    let adc_value = adc_input.read();
                    write!(server, "t={},pa3={}\r\n", now, adc_value).unwrap();
                    last_output = now;
                    led_red.off();
                }

                // Update watchdog
                wd.feed();

                cortex_m::interrupt::free(|cs| {
                    if !net::is_pending(cs) {
                        led_green.on();
                        // Wait for interrupts
                        wfi();
                        led_green.off();
                    }
                });
            }
        });
    });

    unreachable!()
}
