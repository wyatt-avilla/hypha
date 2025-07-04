use embassy_futures::select::{select, Either};
use embassy_time::Timer;
use esp_idf_hal::gpio::{Output, PinDriver};

type LedPin = PinDriver<'static, esp_idf_hal::gpio::Gpio5, Output>;

#[derive(Clone)]
pub enum BlinkCommand {
    Off,
    Solid,
    AlternateEveryMilli(u64),
    SolidThenPulseN(usize),
}

async fn solid_then_pulse_n(led: &mut LedPin, n: usize) -> ! {
    loop {
        let _ = led.set_high();
        Timer::after_secs(2).await;

        for _ in 0..n {
            let _ = led.set_low();
            Timer::after_millis(250).await;
            let _ = led.set_high();
            Timer::after_millis(250).await;
        }

        let _ = led.set_low();
        Timer::after_millis(500).await;
    }
}

pub async fn alternate_ms(led: &mut LedPin, ms: u64) -> ! {
    loop {
        let _ = led.toggle();
        Timer::after_millis(ms).await;
        let _ = led.toggle();
        Timer::after_millis(ms).await;
    }
}

#[embassy_executor::task]
pub async fn task(mut led: LedPin) {
    let receiver = crate::BLINK_CHANNEL.receiver();
    let mut cmd = BlinkCommand::Off;

    loop {
        match cmd {
            BlinkCommand::Off => {
                cmd = receiver.receive().await;
            }
            BlinkCommand::Solid => {
                let _ = led.set_high();
                cmd = receiver.receive().await;
            }
            BlinkCommand::AlternateEveryMilli(ms) => {
                match select(alternate_ms(&mut led, ms), receiver.receive()).await {
                    Either::First(_) => unreachable!(),
                    Either::Second(new_cmd) => cmd = new_cmd,
                }
            }
            BlinkCommand::SolidThenPulseN(n) => {
                match select(solid_then_pulse_n(&mut led, n), receiver.receive()).await {
                    Either::First(_) => unreachable!(),
                    Either::Second(new_cmd) => cmd = new_cmd,
                }
            }
        }
    }
}
