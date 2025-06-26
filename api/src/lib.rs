use actix_web::{HttpRequest, HttpResponse, Responder, body::BoxBody, http::header::ContentType};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
pub struct ServiceStatuses {
    pub service_to_alive: BTreeMap<String, bool>,
}

impl Responder for ServiceStatuses {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
