
pub mod pin_controller {
    use esp_idf_hal::{
        gpio::{PinDriver, Gpio1, Gpio2, Gpio3, Gpio4, Output},
        peripherals::Peripherals
    };

    pub struct Leds {
        in_pin: PinDriver<'static, Gpio2, Output>,
        r_pin: PinDriver<'static, Gpio4, Output>,
        g_pin: PinDriver<'static, Gpio1, Output>,
        b_pin: PinDriver<'static, Gpio3, Output>
    }

    impl Leds {
        pub fn new(peripherals:Peripherals) -> Leds {
            //let peripherals:Peripherals = Peripherals::take().unwrap();
            let pins = peripherals.pins;
            Leds {
                in_pin: PinDriver::output(pins.gpio2).unwrap(),
                r_pin: PinDriver::output(pins.gpio4).unwrap(),
                g_pin: PinDriver::output(pins.gpio1).unwrap(),
                b_pin: PinDriver::output(pins.gpio3).unwrap()
            }
        }

        pub fn set_leds(&mut self, r:bool, g:bool, b:bool, i:bool){
            
            let _ = self.r_pin.set_level(to_level(r));
            let _ = self.g_pin.set_level(to_level(g));
            let _ = self.b_pin.set_level(to_level(b));
            let _ = self.b_pin.set_level(to_level(i));
        }

        pub fn set_leds_and_blink(&mut self, r:bool, g:bool, b:bool){
            
            let _ = self.r_pin.set_level(to_level(r));
            let _ = self.g_pin.set_level(to_level(g));
            let _ = self.b_pin.set_level(to_level(b));
            let _ = self.in_pin.toggle();
        }

        
    }

    fn to_level(b:bool) -> esp_idf_hal::gpio::Level {
        if b {
            esp_idf_hal::gpio::Level::High
        } else {
            esp_idf_hal::gpio::Level::Low
        }
    }
}


