use itertools::Itertools;
use std::collections::BTreeSet;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceMonitorInterfaceInitError {
    #[error("Couldn't initialize systemd D-Bus connection ({0})")]
    DbusConnection(zbus::Error),

    #[error("Couldn't initialize systemd manager proxy ({0})")]
    ManagerProxyConnection(zbus::Error),
}

#[derive(Error, Debug)]
pub enum ServiceMonitorError {
    #[error("D-Bus error ({0})")]
    ZbusError(#[from] zbus::Error),
}

pub struct ServiceMonitorInterface<'a> {
    manager: zbus_systemd::systemd1::ManagerProxy<'a>,
}

impl<'a> ServiceMonitorInterface<'a> {
    pub async fn new() -> Result<ServiceMonitorInterface<'a>, ServiceMonitorInterfaceInitError> {
        let connection = zbus::Connection::system()
            .await
            .map_err(ServiceMonitorInterfaceInitError::DbusConnection)?;

        let manager = zbus_systemd::systemd1::ManagerProxy::new(&connection)
            .await
            .map_err(ServiceMonitorInterfaceInitError::ManagerProxyConnection)?;

        Ok(ServiceMonitorInterface { manager })
    }

    pub async fn unit_file_names(&self) -> Result<Vec<String>, ServiceMonitorError> {
        match self.manager.list_unit_files().await {
            Err(e) => Err(ServiceMonitorError::ZbusError(e)),
            Ok(v) => Ok(v
                .into_iter()
                .map(|(name, _)| name.split('/').next_back().unwrap_or(&name).to_owned())
                .collect_vec()),
        }
    }

    pub async fn disjoint_from_unit_file_names(
        &self,
        query_names: &[String],
    ) -> Result<BTreeSet<String>, ServiceMonitorError> {
        use std::borrow::ToOwned;
        let query_names: BTreeSet<String> = query_names.iter().map(ToOwned::to_owned).collect();
        let unit_file_names: BTreeSet<String> = self
            .unit_file_names()
            .await?
            .iter()
            .map(ToOwned::to_owned)
            .collect();

        Ok(query_names
            .difference(&unit_file_names)
            .map(ToOwned::to_owned)
            .collect())
    }
}
