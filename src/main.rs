//use esp_backtrace as _;

// https://docs.esp-rs.org/esp-idf-hal/esp_idf_hal/gpio/index.html

//use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use esp_idf_svc::hal::delay;

use esp_idf_hal::{
    gpio::PinDriver,
    peripherals::Peripherals
};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();
    
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");
    
    let mut r_pin = PinDriver::output(Peripherals::take().unwrap().pins.gpio2).unwrap();

    loop {                
        log::info!("Arriba");
        r_pin.set_high();
        delay::FreeRtos::delay_ms(500);
        log::info!("Abajo");
        r_pin.set_low();
        delay::FreeRtos::delay_ms(500);
    }





}
