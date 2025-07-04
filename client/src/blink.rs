use embassy_time::Timer;
use esp_idf_hal::{
    gpio::{Output, PinDriver},
    sys::EspError,
};

type LedPin = PinDriver<'static, esp_idf_hal::gpio::Gpio5, Output>;

pub enum BlinkCommand {
    Off,
    Solid,
    AlternateEveryMilli(u64),
    SolidThenPulseN(usize),
}

pub async fn alternating_sec(led: &mut LedPin) -> Result<(), EspError> {
    led.set_high()?;
    Timer::after_secs(1).await;
    led.set_low()?;
    Timer::after_secs(1).await;
    Ok(())
}

pub async fn solid_then_pulse_n(led: &mut LedPin, n: usize) -> Result<(), EspError> {
    led.set_high()?;
    Timer::after_secs(2).await;
    led.set_low()?;
    Timer::after_millis(500).await;
    for _ in 0..n {
        led.toggle()?;
        Timer::after_millis(250).await;
        led.toggle()?;
        Timer::after_millis(250).await;
    }
    Timer::after_secs(1).await;

    Ok(())
}
