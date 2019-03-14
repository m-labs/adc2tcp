use embedded_hal::digital::OutputPin;
use stm32f4xx_hal::gpio::{
    Output, PushPull,
    gpiob::{PB0, PB7, PB14},
};

type GreenPin = PB0<Output<PushPull>>;
type BluePin = PB7<Output<PushPull>>;
type RedPin = PB14<Output<PushPull>>;

pub struct Led<PIN> {
    pin: PIN,
}

impl<PIN: OutputPin> Led<PIN> {
    fn new(pin: PIN) -> Self {
        Led { pin }
    }

    pub fn on(&mut self) {
        self.pin.set_high();
    }

    pub fn off(&mut self) {
        self.pin.set_low();
    }
}


impl Led<GreenPin> {
    pub fn green(pin: GreenPin) -> Self {
        Self::new(pin)
    }
}

impl Led<BluePin> {
    pub fn blue(pin: BluePin) -> Self {
        Self::new(pin)
    }
}

impl Led<RedPin> {
    pub fn red(pin: RedPin) -> Self {
        Self::new(pin)
    }
}
