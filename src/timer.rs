use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::exception;
use stm32f4xx_hal::{
    rcc::Clocks,
    time::{U32Ext, MilliSeconds},
    timer::{Timer, Event as TimerEvent},
    stm32::SYST,
};

/// Rate in Hz
const TIMER_RATE: u32 = 10;
/// Interval duration in milliseconds
const TIMER_DELTA: u32 = 1000 / TIMER_RATE;
/// Elapsed time in milliseconds
static TIMER_MS: Mutex<RefCell<u32>> = Mutex::new(RefCell::new(0));

/// Setup SysTick exception
pub fn setup(syst: SYST, clocks: Clocks) {
    let mut timer = Timer::syst(syst, TIMER_RATE.hz(), clocks);
    timer.listen(TimerEvent::TimeOut);
}

/// SysTick exception (Timer)
#[exception]
fn SysTick() {
    cortex_m::interrupt::free(|cs| {
        *TIMER_MS.borrow(cs)
            .borrow_mut() += TIMER_DELTA;
    });
}

/// Obtain current time in milliseconds
pub fn now() -> MilliSeconds {
    let ms = cortex_m::interrupt::free(|cs| {
        *TIMER_MS.borrow(cs)
            .borrow()
    });
    ms.ms()
}
