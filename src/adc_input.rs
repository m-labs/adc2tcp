use stm32f4xx_hal::{
    adc::{
        Adc,
        config::*,
    },
    gpio::{Analog, gpioa::PA3 as Pin},
    stm32::ADC1 as ADC,
};

/// ADC Input
pub struct AdcInput {
    /// unused but consumed
    _pin: Pin<Analog>,
    adc: Adc<ADC>,
}

impl AdcInput {
    /// Configure pin into analog mode
    pub fn new<MODE>(adc: ADC, pin: Pin<MODE>) -> Self {
        let pin = pin.into_analog();
        let adc_config = AdcConfig::default()
            .scan(Scan::Enabled)
            .continuous(Continuous::Single)
            .clock(Clock::Pclk2_div_2);
        let mut adc = Adc::adc1(adc, true, adc_config);

        adc.configure_channel(&pin, Sequence::One, SampleTime::Cycles_480);
        
        AdcInput { _pin: pin, adc }
    }

    /// Enable the ADC,
    /// run a conversion
    /// disable the ADC
    pub fn read(&mut self) -> u16 {
        let adc = &mut self.adc;
        adc.enable();
        adc.clear_end_of_conversion_flag();
        adc.start_conversion();
        let sample = adc.current_sample();
        let result = adc.sample_to_millivolts(sample);
        adc.wait_for_conversion_sequence();
        adc.disable();
        result
    }
}
