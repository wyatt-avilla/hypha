use embassy_executor::Spawner;
use embassy_time::Timer;
use esp_idf_hal::{gpio::PinDriver, peripherals::Peripherals};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let _ = spawner;
    esp_idf_svc::sys::link_patches();
    esp_idf_hal::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio2).unwrap();

    log::info!("entering loop...");
    loop {
        Timer::after_secs(1).await;
        let _ = led.set_low();
        Timer::after_secs(1).await;
        let _ = led.set_high();
    }
}
