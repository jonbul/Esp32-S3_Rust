
use esp_idf_hal::ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_svc::hal::delay;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");
    

    let peripherals = Peripherals::take().unwrap();
    let timer_driver_r = LedcTimerDriver::new(peripherals.ledc.timer0, &TimerConfig::default().frequency(25.kHz().into())).unwrap();
    let timer_driver_g = LedcTimerDriver::new(peripherals.ledc.timer1, &TimerConfig::default().frequency(25.kHz().into())).unwrap();
    let timer_driver_b = LedcTimerDriver::new(peripherals.ledc.timer2, &TimerConfig::default().frequency(25.kHz().into())).unwrap();
    
    let r_driver = LedcDriver::new(peripherals.ledc.channel0, timer_driver_r, peripherals.pins.gpio4).unwrap();
    let g_driver = LedcDriver::new(peripherals.ledc.channel1, timer_driver_g, peripherals.pins.gpio1).unwrap();
    let b_driver = LedcDriver::new(peripherals.ledc.channel2, timer_driver_b, peripherals.pins.gpio3).unwrap();

    let mut r_pwm_driver = PwmDriver::new(r_driver, 0);
    let mut g_pwm_driver = PwmDriver::new(g_driver, 0);
    let mut b_pwm_driver = PwmDriver::new(b_driver, 0);
    //let mut g_pwm_driver = PwmDriver::new(g_driver, 64);
    //let mut b_pwm_driver = PwmDriver::new(b_driver, 128);

    loop {
        r_pwm_driver.breath();
        g_pwm_driver.breath();
        b_pwm_driver.breath();
        
        delay::FreeRtos::delay_ms(5);
    }

    /*let mut r_count:u8 = 0;
    let mut g_count:u8 = 0;
    let mut b_count:u8 = 0;
    loop {
        log::info!("r {} g {} b {}", r_count, g_count, b_count);

        r_pwm_driver.set(r_count.into());
        g_pwm_driver.set(g_count.into());
        b_pwm_driver.set(b_count.into());

        if (r_count == 255) {
            r_count = 0;
            
            if g_count == 255 {
                g_count = 0;

                if b_count == 255 {
                    b_count = 0;
                } else {
                    b_count += 1;
                }
            } else {
                g_count += 1;
            }
            
        } else {
            r_count += 1;
        }

        //delay::FreeRtos::delay_ms(5);
    }*/
}

struct PwmDriver<'a> {
    driver: LedcDriver<'a>,
    duty: u32,
    up: bool,
    max_duty: u32,
}
impl<'a> PwmDriver<'a> {
    fn new(driver: LedcDriver<'a>, duty: u32) -> Self {
        let max_duty = driver.get_max_duty();
        Self {
            driver,
            duty,
            up: true,
            max_duty,
        }
    }

    fn breath(&mut self) {
        let _ = self.driver.set_duty(self.duty);

        self.duty = if self.up { self.duty + 1 } else { self.duty - 1 };

        if self.duty >= self.max_duty || self.duty == 0 {
            self.up = !self.up;
        }
    }

    fn set(&mut self, duty: u32) {
        let _ = self.driver.set_duty(duty);
    }
}
