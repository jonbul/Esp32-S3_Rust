use esp_idf_svc::hal::{
    delay,
    //ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver, Resolution},
    gpio::PinDriver,
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

    let button = PinDriver::input(peripherals.pins.gpio5).unwrap();
    let mut relay = PinDriver::output(peripherals.pins.gpio18).unwrap();

    let mut last_status = button.is_high();

    loop {
        let status = !button.is_high();

        if last_status != status {
            log::info!("Cambio: {}", if status {"Activado"} else {"Desactivado"});
            last_status = status
        }
        let _ = relay.set_level(to_level(status));
        

        delay::FreeRtos::delay_ms(100);
    }
    fn to_level(b: bool) -> esp_idf_svc::hal::gpio::Level {
        if b {
            esp_idf_svc::hal::gpio::Level::High
        } else {
            esp_idf_svc::hal::gpio::Level::Low
        }
    }
}
