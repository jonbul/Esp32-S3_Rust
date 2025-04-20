//use esp_backtrace as _;

// https://docs.esp-rs.org/esp-idf-hal/esp_idf_hal/gpio/index.html

//use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

pub mod pin_controller;

use esp_idf_svc::hal::delay;


use esp_idf_hal::peripherals::Peripherals;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();
    
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");
    
    let peripherals:Peripherals = Peripherals::take().unwrap();
    log::info!("Pins declared");

    let mut leds = pin_controller::pin_controller::Leds::new(peripherals);
    let mut counter:u8 = 0;
    loop {

        let r = counter & 0b0001 != 0;
        let g = counter & 0b0010 != 0;
        let b = counter & 0b0100 != 0;

        log::info!("counter {} -> r {} g {} b {}", counter, r, g, b);
        let _ = leds.set_leds_and_blink(r, g, b);
        delay::FreeRtos::delay_ms(500);

        if counter < 7 {
            counter += 1;
        } else {
            counter = 0;
        }
    }

}
