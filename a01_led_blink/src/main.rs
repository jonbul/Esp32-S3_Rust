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
    
    let peripherals = Peripherals::take().unwrap();
    let mut in_pin = PinDriver::output(peripherals.pins.gpio2).unwrap();
    let mut r_pin = PinDriver::output(peripherals.pins.gpio4).unwrap();
    let mut g_pin = PinDriver::output(peripherals.pins.gpio1).unwrap();
    let mut b_pin = PinDriver::output(peripherals.pins.gpio3).unwrap();
    log::info!("Pins declared");
    let mut blink = false;

    let mut turn = 0;
    loop {            
        if blink {
            let _ = in_pin.set_low();
            blink = false;
            log::info!("SI");
        } else {
            let _ = in_pin.set_high();
            blink = true;
            log::info!("NO");
        }
        if turn == 0 {
            log::info!("rojo");
            let _ = r_pin.set_high();
            let _ = g_pin.set_low();
            let _ = b_pin.set_low();
        } else if turn == 1 {
            log::info!("verde");
            let _ = r_pin.set_low();
            let _ = g_pin.set_high();
            let _ = b_pin.set_low();
        } else if turn == 2 {
            log::info!("azul");
            let _ = r_pin.set_low();
            let _ = g_pin.set_low();
            let _ = b_pin.set_high();
        }
        turn += 1
        if turn == 3 {
            turn = 0;
        } 

        delay::FreeRtos::delay_ms(500);
    }
}

fn 

/*
    let mut r_pin = PinDriver::output(Peripherals::take().unwrap().pins.gpio2).unwrap();
    let mut g_pin = PinDriver::output(Peripherals::take().unwrap().pins.gpio14).unwrap();
    let mut b_pin = PinDriver::output(Peripherals::take().unwrap().pins.gpio19).unwrap();

    loop {                
        log::info!("1");
        let _ = r_pin.set_high();
        let _ = g_pin.set_low();
        let _ = b_pin.set_low();
        delay::FreeRtos::delay_ms(500);
        log::info!("2");
        let _ = r_pin.set_low();
        let _ = g_pin.set_high();
        let _ = b_pin.set_low();
        delay::FreeRtos::delay_ms(500);
        log::info!("2");
        let _ = r_pin.set_low();
        let _ = g_pin.set_low();
        let _ = b_pin.set_high();
        delay::FreeRtos::delay_ms(500);
    }
}
*/