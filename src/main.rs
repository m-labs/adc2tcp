#![no_std]
#![no_main]

// extern crate cortex_m;
// extern crate cortex_m_rt;
// extern crate cortex_m_semihosting;
extern crate stm32f429 as board;
// extern crate stm32_eth as eth;
// extern crate smoltcp;
#[allow(unused_extern_crates)]
extern crate panic_itm;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    loop {}
}
