
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
    let timer_driver = LedcTimerDriver::new(peripherals.ledc.timer0, &TimerConfig::default().frequency(25.kHz().into())).unwrap();
    let mut driver = LedcDriver::new(peripherals.ledc.channel0, timer_driver, peripherals.pins.gpio1).unwrap();

    let max_duty = driver.get_max_duty();
    let mut duty:u32 = 0;
    log::info!("max duty {}", max_duty);

    let mut up = true;

    loop {
        //log::info!("duty {}", duty);
        driver.set_duty(duty);
        
        duty = if up { duty + 1 } else { duty - 1 };
        if duty >= max_duty || duty <= 0 {
            up = !up;
        }
        delay::FreeRtos::delay_ms(5);

    }
}
