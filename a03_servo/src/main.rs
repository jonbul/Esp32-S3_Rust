
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
    let mut driver: LedcDriver<'_> = LedcDriver::new(peripherals.ledc.channel0, timer_driver, peripherals.pins.gpio18).unwrap();
    let period_ticks = driver.get_max_duty(); // 16383
    //let min_duty = period_ticks / 20;        // 1ms -> 0°
    //let max_duty = period_ticks * 2 / 20;*/ 
    let max_duty = 2118;    // 2ms -> 180°
    let min_duty = 700;
    log::info!("period_ticks {} min_duty {} max_duty {}", period_ticks, min_duty, max_duty);

    let _ = driver.set_duty(max_duty);
    delay::FreeRtos::delay_ms(1000);
    log::info!("START!");
    /*for i in 700 ..900 {
        //let val = 1000 - i * 10;
        let _ = driver.set_duty(i);
        log::info!("duty {}", i);
        delay::FreeRtos::delay_ms(100);
    }*/

    //let step = (max_duty - min_duty) / 10;
    loop {
        for i in 0 ..=10 {
            //let _ = driver.set_duty(i * step + min_duty);
            move_to_angle(&mut driver, i * 18, min_duty, max_duty);
            delay::FreeRtos::delay_ms(100);
            log::info!("step {} -> angle {}°, duty {}", i, i * 18, driver.get_duty());
        }
        for i in (0 ..=10).rev() {
            //let _ = driver.set_duty(i * step + min_duty);
            move_to_angle(&mut driver, i * 18, min_duty, max_duty);
            delay::FreeRtos::delay_ms(100);
            log::info!("step {} -> angle {}°, duty {}", i, i * 18, driver.get_duty());
        }
    }

    fn move_to_angle(driver: &mut LedcDriver, angle: u8, min_duty: u32, max_duty: u32) {
        let duty = min_duty + ((max_duty - min_duty) as u32 * angle as u32) / 180;
        let _ = driver.set_duty(duty);
    }
    
}
