use embedded_svc::http::client::Client as HttpClient;
use esp_idf_hal::io::EspIOError;
use esp_idf_svc::http::{client::EspHttpConnection, Method};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum QueryError {
    #[error("Error while sending http request ({0})")]
    Http(EspIOError),

    #[error("Bad `content-length`")]
    ContentLength,

    #[error("Couldn't parse response body")]
    Parse,
}

pub fn query_services(
    client: &mut HttpClient<EspHttpConnection>,
    url: &str,
) -> Result<api::ServiceStatuses, QueryError> {
    let headers = [("accept", "application/json")];
    let request = client
        .request(Method::Get, url, &headers)
        .map_err(QueryError::Http)?;
    let mut response = request.submit().map_err(QueryError::Http)?;

    let content_length = response
        .header("content-length")
        .map(str::parse::<usize>)
        .transpose()
        .map_err(|_| QueryError::ContentLength)?
        .ok_or(QueryError::ContentLength)?;
    let mut v = vec![0u8; content_length];
    response
        .read(v.as_mut_slice())
        .map_err(|_| QueryError::Parse)?;

    serde_json::from_str(std::str::from_utf8(v.as_slice()).map_err(|_| QueryError::Parse)?)
        .map_err(|_| QueryError::Parse)
}
