use stm32f4xx_hal::{
    adc::{
        Adc,
        config::*,
    },
    gpio::{Analog, gpioa::PA3 as Pin},
    stm32::ADC1 as ADC,
};

pub struct AdcInput {
    /// unused but consumed
    _pin: Pin<Analog>,
    pub adc: Adc<ADC>,
}

impl AdcInput {
    pub fn new<MODE>(adc: ADC, pin: Pin<MODE>) -> Self {
        let pin = pin.into_analog();
        let adc_config = AdcConfig::default()
            .scan(Scan::Enabled)
            .continuous(Continuous::Single)
            .end_of_conversion_interrupt(Eoc::Conversion)
            .clock(Clock::Pclk2_div_8);
        let mut adc = Adc::adc1(adc, true, adc_config);

        adc.configure_channel(&pin, Sequence::One, SampleTime::Cycles_480);
        adc.start_conversion();
        
        AdcInput { _pin: pin, adc }
    }

    pub fn read(&mut self) -> u16 {
        let sample = self.adc.current_sample();
        self.adc.start_conversion();
        self.adc.sample_to_millivolts(sample)
    }
}
