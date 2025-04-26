
use esp_idf_svc::hal::{
    ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver, Resolution},
    peripherals::Peripherals,
    delay
};
use esp_idf_svc::hal::prelude::*;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();

    let timer_driver = LedcTimerDriver::new(peripherals.ledc.timer0, &TimerConfig::default().frequency(50.Hz().into()).resolution(Resolution::Bits14)).unwrap();
    let mut driver: LedcDriver<'_> = LedcDriver::new(peripherals.ledc.channel0, timer_driver, peripherals.pins.gpio6).unwrap();
    let max_duty = driver.get_max_duty() * 10 / 100; // 0º
    let min_duty = max_duty * 5 / 100; // 180º

    log::info!("max duty {} min duty {}", max_duty, min_duty);

    let _ = driver.set_duty(min_duty);
    delay::FreeRtos::delay_ms(1);

    let step = (max_duty - min_duty) / 10;
    loop {
        for i in 1 ..=10 {
            let _ = driver.set_duty(i * step + min_duty);
            delay::FreeRtos::delay_ms(100);
            log::info!("{} duty {}", i, driver.get_duty());
        }
        for i in (1 ..=10).rev() {
            let _ = driver.set_duty(i * step + min_duty);
            delay::FreeRtos::delay_ms(100);
            log::info!("{} duty {}", i, driver.get_duty());
        }
    }

    /*loop {

        for i in min_duty..=max_duty {
            let _ = driver.set_duty(i);
            delay::FreeRtos::delay_ms(20);
            log::info!("{} duty {}", i, driver.get_duty());
        }
        for i in (min_duty..=max_duty).rev() {
            let _ = driver.set_duty(i);
            delay::FreeRtos::delay_ms(20);
            log::info!("{} duty {}", i, driver.get_duty());
        }
    }*/
    
}
