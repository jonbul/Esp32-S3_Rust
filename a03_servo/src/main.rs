use esp_idf_svc::hal::prelude::*;
use esp_idf_svc::hal::{
    delay,
    ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver, Resolution},
    peripherals::Peripherals,
};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();

    let timer_driver = LedcTimerDriver::new(
        peripherals.ledc.timer0,
        &TimerConfig::default()
            .frequency(50.Hz().into())
            .resolution(Resolution::Bits14),
    )
    .unwrap();
    let mut driver: LedcDriver<'_> = LedcDriver::new(
        peripherals.ledc.channel0,
        timer_driver,
        peripherals.pins.gpio18,
    )
    .unwrap();
    let period_ticks = driver.get_max_duty(); // 16383
                                              //let min_duty = period_ticks / 20;        // 1ms -> 0°
                                              //let max_duty = period_ticks * 2 / 20;*/

    // Those should be calculated but servo is SHIT and is not correctly calibrated
    let max_duty = 2118; // 2ms -> 180°
    let min_duty = 700;
    log::info!(
        "period_ticks {} min_duty {} max_duty {}",
        period_ticks,
        min_duty,
        max_duty
    );

    let _ = driver.set_duty(max_duty);
    delay::FreeRtos::delay_ms(1000);
    log::info!("START!");
    /*for i in 700 ..900 {
        //let val = 1000 - i * 10;
        let _ = driver.set_duty(i);
        log::info!("duty {}", i);
        delay::FreeRtos::delay_ms(100);
    }*/

    let loop_time: u32 = 2000;
    let total_steps = 40;
    let delay_time = loop_time / total_steps;
    let mut dir = true;

    loop {
        for i in 0..=total_steps {
            //let _ = driver.set_duty(i * step + min_duty);
            let multiplier = if dir { i } else { total_steps - i };
            let angle: u8 = (180 / total_steps * multiplier)
                .try_into()
                .unwrap_or_else(|_| {
                    log::error!("angle overflow");
                    0
                });
            move_to_angle(&mut driver, angle.into(), min_duty, max_duty);
            delay::FreeRtos::delay_ms(delay_time);
            log::info!("step {} -> angle {}°, duty {}", i, angle, driver.get_duty());
        }
        dir = !dir;
    }

    fn move_to_angle(driver: &mut LedcDriver, angle: u8, min_duty: u32, max_duty: u32) {
        let duty = min_duty + ((max_duty - min_duty) as u32 * angle as u32) / 180;
        let _ = driver.set_duty(duty);
    }
}
