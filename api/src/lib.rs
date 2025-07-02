use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

mod unit;

pub use unit::UnitActiveState;
pub use unit::UnitActiveSubState;
pub use unit::UnitLoadState;

pub static DEFAULT_SERVER_PORT: u16 = 8910;
pub static SERVER_ENDPOINT: &str = "/api";

#[derive(Serialize, Deserialize)]
pub struct ServiceStatuses {
    pub map: BTreeMap<
        String,
        (
            unit::UnitLoadState,
            unit::UnitActiveState,
            unit::UnitActiveSubState,
        ),
    >,
}
