use serde::Deserialize;
use std::fmt;

use crate::DeviceTypeFilter;

#[derive(Debug, Deserialize)]
pub struct Device {
    label: String,

    #[serde(rename = "controllableName")]
    controllable_name: String,
    definition: DeviceDefinition,

    #[serde(rename = "deviceURL")]
    url: String,
    enabled: bool,
}

impl Device {
    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn definition(&self) -> &DeviceDefinition {
        &self.definition
    }

    pub fn id(&self) -> &str {
        let parts: Vec<&str> = self.url.split('/').collect();
        parts.last().unwrap()
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn matches(&self, filter: DeviceTypeFilter) -> bool {
        if filter == DeviceTypeFilter::All {
            return true;
        }

        self.controllable_name == filter.as_str()
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} ({})",
            self.id(),
            self.label,
            self.controllable_name
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct DeviceDefinition {
    #[serde(rename = "commands")]
    actions: Vec<DeviceAction>,
}

impl DeviceDefinition {
    pub fn actions(&self) -> &Vec<DeviceAction> {
        &self.actions
    }
}

#[derive(Debug, Deserialize)]
pub struct DeviceAction {
    #[serde(rename = "nparams")]
    params_count: i32,
    #[serde(rename = "commandName")]
    name: String,
    #[serde(rename = "paramsSig")]
    params_signature: Option<String>,
}

impl DeviceAction {
    pub fn params_count(&self) -> i32 {
        self.params_count
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn params_signature(&self) -> Option<&str> {
        self.params_signature.as_deref()
    }
}

impl fmt::Display for DeviceAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(params) = &self.params_signature {
            return write!(f, "{}: [{}] ({})", self.name, params, self.params_count);
        }

        write!(f, "{}", self.name)
    }
}
