use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

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

impl UnitLoadState {
    pub fn is_loaded(&self) -> bool {
        matches!(self, Self::Loaded)
    }
}

// https://www.freedesktop.org/software/systemd/man/latest/systemd.html#Units
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

impl UnitActiveState {
    pub fn is_active(&self) -> bool {
        matches!(
            self,
            Self::Active | Self::Activating | Self::Reloading | Self::Refreshing
        )
    }
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
