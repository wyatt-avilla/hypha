use dotenvy_macro::dotenv;
use embassy_executor::Spawner;
use embedded_svc::http::client::Client as HttpClient;
use esp_idf_hal::{gpio::PinDriver, peripherals::Peripherals};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    http::client::{Configuration, EspHttpConnection},
    nvs::EspDefaultNvsPartition,
};

mod blink;
mod http;
mod wifi;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let _ = spawner;
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let mut peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    let _wifi = wifi::connect_to(
        dotenv!("WIFI_SSID"),
        dotenv!("WIFI_PASSWORD"),
        &mut peripherals.modem,
        sys_loop.clone(),
        nvs.clone(),
    )
    .unwrap();

    let mut client = HttpClient::wrap(EspHttpConnection::new(&Configuration::default()).unwrap());
    log::info!("built client...");

    let statuses = http::query_services(
        &mut client,
        format!(
            "http://{}:{}{}",
            dotenv!("SERVER_IP"),
            api::DEFAULT_SERVER_PORT,
            api::SERVER_ENDPOINT
        )
        .as_str(),
    )
    .unwrap();

    let mut led = PinDriver::output(peripherals.pins.gpio5).unwrap();

    log::info!("entering loop...");
    loop {
        let _ = blink::alternating_sec(&mut led).await;
        let _ = blink::solid_then_pulse_n(&mut led, 5).await;
    }
}
