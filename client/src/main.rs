use embassy_executor::Spawner;
use esp_idf_hal::{gpio::PinDriver, peripherals::Peripherals};

mod blink;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let _ = spawner;
    esp_idf_svc::sys::link_patches();
    esp_idf_hal::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let mut led = PinDriver::output(peripherals.pins.gpio5).unwrap();

    log::info!("entering loop...");
    loop {
        let _ = blink::alternating_sec(&mut led).await;
        let _ = blink::solid_then_pulse_n(&mut led, 5).await;
    }
}
