use dotenvy_macro::dotenv;
use embassy_time::Timer;
use embedded_svc::http::client::Client as HttpClient;
use esp_idf_hal::{io::EspIOError, sys::EspError};
use esp_idf_svc::http::{
    client::{Configuration, EspHttpConnection},
    Method,
};
use std::mem::discriminant;
use thiserror::Error;

use crate::blink::{self, BlinkCommand};

#[derive(Error, Debug)]
pub enum QueryError {
    #[error("Error while sending http request ({0})")]
    Http(EspIOError),

    #[error("Couldn't parse server response")]
    Parse,

    #[error("Couldn't create client")]
    ClientInit(EspError),
}

#[allow(clippy::unused_async)]
async fn query_services(url: &str) -> Result<api::ServiceStatuses, QueryError> {
    let mut client = HttpClient::wrap(
        EspHttpConnection::new(&Configuration::default()).map_err(QueryError::ClientInit)?,
    );

    let mut response = client
        .request(Method::Get, url, &[("accept", "application/json")])
        .map_err(QueryError::Http)?
        .submit()
        .map_err(QueryError::Http)?;

    let content_length = response
        .header("content-length")
        .map(str::parse::<usize>)
        .transpose()
        .map_err(|_| QueryError::Parse)?
        .ok_or(QueryError::Parse)?;
    let mut v = vec![0u8; content_length];
    response.read(v.as_mut_slice()).map_err(QueryError::Http)?;

    serde_json::from_str(std::str::from_utf8(v.as_slice()).map_err(|_| QueryError::Parse)?)
        .map_err(|_| QueryError::Parse)
}

#[embassy_executor::task]
pub async fn task(query_interval_seconds: u64) {
    let sender = crate::BLINK_CHANNEL.sender();
    let mut previously_sent = blink::BlinkCommand::Off;

    let mut send_if_prev_diff = |cmd: BlinkCommand| {
        if discriminant(&previously_sent) != discriminant(&cmd) {
            sender.try_send(cmd.clone()).ok();
            previously_sent = cmd;
        }
    };

    loop {
        let down_services = query_services(
            format!(
                "http://{}:{}{}",
                dotenv!("SERVER_IP"),
                api::DEFAULT_SERVER_PORT,
                api::SERVER_ENDPOINT
            )
            .as_str(),
        )
        .await
        .map(|s| {
            s.map
                .into_iter()
                .filter(|(_, (load_state, active_state, _))| {
                    !load_state.is_loaded() || !active_state.is_active()
                })
                .collect::<Vec<_>>()
        });

        match down_services {
            Ok(s) => {
                if s.is_empty() {
                    send_if_prev_diff(blink::BlinkCommand::Off);
                } else {
                    log::error!("The following units are down {s:#?}");
                    send_if_prev_diff(blink::BlinkCommand::SolidThenPulseN(s.len()));
                }
            }
            Err(e) => match e {
                QueryError::Parse | QueryError::ClientInit(_) => {
                    log::error!("{e}");
                    send_if_prev_diff(blink::BlinkCommand::Solid);
                }
                QueryError::Http(e) => {
                    log::error!("Http request failed ({e})");
                    send_if_prev_diff(blink::BlinkCommand::AlternateEveryMilli(1000));
                }
            },
        }

        Timer::after_secs(query_interval_seconds).await;
    }
}
