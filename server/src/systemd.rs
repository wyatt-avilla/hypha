use itertools::Itertools;
use std::collections::BTreeSet;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceMonitorError {
    #[error("Couldn't initialize systemd D-Bus connection ({0})")]
    DbusConnection(zbus::Error),

    #[error("Couldn't initialize systemd manager proxy ({0})")]
    ManagerProxyConnection(zbus::Error),

    #[error("Invalid service names [{0}]")]
    InvalidServiceName(String),

    #[error("Couldn't parse '{0}' into a state enum")]
    StateParseError(String),

    #[error("D-Bus error ({0})")]
    ZbusError(#[from] zbus::Error),
}

pub struct ServiceMonitorInterface<'a> {
    manager: zbus_systemd::systemd1::ManagerProxy<'a>,
    monitored_services: Vec<String>,
}

impl<'a> ServiceMonitorInterface<'a> {
    pub async fn unit_file_names(
        manager: zbus_systemd::systemd1::ManagerProxy<'a>,
    ) -> Result<Vec<String>, ServiceMonitorError> {
        match manager.list_unit_files().await {
            Err(e) => Err(ServiceMonitorError::ZbusError(e)),
            Ok(v) => Ok(v
                .into_iter()
                .map(|(name, _)| name.split('/').next_back().unwrap_or(&name).to_owned())
                .collect_vec()),
        }
    }

    pub async fn disjoint_from_unit_file_names(
        manager: zbus_systemd::systemd1::ManagerProxy<'a>,
        query_names: &[String],
    ) -> Result<BTreeSet<String>, ServiceMonitorError> {
        use std::borrow::ToOwned;
        let query_names: BTreeSet<String> = query_names.iter().map(ToOwned::to_owned).collect();
        let unit_file_names: BTreeSet<String> = ServiceMonitorInterface::unit_file_names(manager)
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

impl<'a> ServiceMonitorInterface<'a> {
    pub async fn new(
        services_to_monitor: &[String],
    ) -> Result<ServiceMonitorInterface<'a>, ServiceMonitorError> {
        let connection = zbus::Connection::system()
            .await
            .map_err(ServiceMonitorError::DbusConnection)?;

        let manager = zbus_systemd::systemd1::ManagerProxy::new(&connection)
            .await
            .map_err(ServiceMonitorError::ManagerProxyConnection)?;

        let invalid_services = ServiceMonitorInterface::disjoint_from_unit_file_names(
            manager.clone(),
            services_to_monitor,
        )
        .await?;

        if !invalid_services.is_empty() {
            return Err(ServiceMonitorError::InvalidServiceName(
                invalid_services.into_iter().join(", "),
            ));
        }

        Ok(ServiceMonitorInterface {
            manager,
            monitored_services: services_to_monitor
                .iter()
                .map(std::borrow::ToOwned::to_owned)
                .collect(),
        })
    }

    pub fn monitored_services(&self) -> impl Iterator<Item = &String> {
        self.monitored_services.iter()
    }

    pub async fn get_service_statuses(&self) -> Result<api::ServiceStatuses, ServiceMonitorError> {
        Ok(api::ServiceStatuses {
            map: self
                .manager
                .list_units_by_names(self.monitored_services.clone())
                .await?
                .into_iter()
                .map(|t| -> Result<_, ServiceMonitorError> {
                    Ok((
                        t.0,
                        (
                            t.2.parse::<api::UnitLoadState>()
                                .map_err(|_| ServiceMonitorError::StateParseError(t.2))?,
                            t.3.parse::<api::UnitActiveState>()
                                .map_err(|_| ServiceMonitorError::StateParseError(t.3))?,
                            t.4.parse::<api::UnitActiveSubState>()
                                .map_err(|_| ServiceMonitorError::StateParseError(t.4))?,
                        ),
                    ))
                })
                .collect::<Result<_, _>>()?,
        })
    }
}
