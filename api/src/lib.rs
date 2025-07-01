use actix_web::{HttpRequest, HttpResponse, Responder, body::BoxBody, http::header::ContentType};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use strum_macros::{Display, EnumString};

pub static DEFAULT_SERVER_PORT: u16 = 8910;
pub static SERVER_ENDPOINT: &str = "/api";

#[derive(Serialize, Deserialize)]
pub struct ServiceStatuses {
    pub map: BTreeMap<String, (UnitLoadState, UnitActiveState, UnitActiveSubState)>,
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

#[derive(EnumString, Display, Debug, Serialize, Deserialize)]
#[strum(serialize_all = "kebab-case")]
pub enum UnitLoadState {
    Stub,
    Loaded,
    NotFound,
    BadSetting,
    Error,
    Merged,
    Masked,
}

#[derive(EnumString, Display, Debug, Serialize, Deserialize)]
#[strum(serialize_all = "kebab-case")]
pub enum UnitActiveState {
    Active,
    Reloading,
    Inactive,
    Failed,
    Activating,
    Deactivating,
    Maintenance,
    Refreshing,
}

#[derive(EnumString, Display, Debug, Serialize, Deserialize)]
#[strum(serialize_all = "kebab-case")]
pub enum UnitActiveSubState {
    Dead,
    Condition,
    StartPre,
    Start,
    StartPost,
    Running,
    Exited,
    Reload,
    ReloadSignal,
    ReloadNotify,
    Mounting,
    Stop,
    StopWatchdog,
    StopSigterm,
    StopSigkill,
    StopPost,
    FinalWatchdog,
    FinalSigterm,
    FinalSigkill,
    Failed,
    DeadBeforeAutoRestart,
    FailedBeforeAutoRestart,
    DeadResourcesPinned,
    AutoRestart,
    AutoRestartQueued,
    Cleaning,
}
