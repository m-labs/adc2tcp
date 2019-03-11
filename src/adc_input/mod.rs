use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use stm32f4xx_hal::{
    gpio::{gpioa::PA3 as Pin},
    stm32::{NVIC, ADC1 as ADC, interrupt, Interrupt},
};

mod input;
use input::AdcInput;

static ADC_INPUT: Mutex<RefCell<Option<AdcInput>>> = Mutex::new(RefCell::new(None));
static ADC_VALUE: Mutex<RefCell<Option<u16>>> = Mutex::new(RefCell::new(None));

pub fn setup<MODE>(nvic: &mut NVIC, adc: ADC, pin: Pin<MODE>) {
    let adc_input = AdcInput::new(adc, pin);
    cortex_m::interrupt::free(|cs| {
        ADC_INPUT.borrow(cs)
            .replace(Some(adc_input))
    });
    nvic.enable(Interrupt::ADC);
}

pub fn read() -> Option<u16> {
    cortex_m::interrupt::free(|cs| {
        ADC_VALUE.borrow(cs).borrow_mut().take()
    })
}

#[interrupt]
fn ADC() {
    cortex_m::interrupt::free(|cs| {
        let mut adc_input = ADC_INPUT.borrow(cs)
            .borrow_mut();
        let value = adc_input.as_mut()
            .map(|adc_input| adc_input.read());
        *ADC_VALUE.borrow(cs)
            .borrow_mut() = value;
    });
}
