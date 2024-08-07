use std::fmt;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TahomaSetup {
    pub devices: Vec<Device>,
}

impl TahomaSetup {
    pub fn get_device_by_label(&self, device_label: &str, invariant_case: bool) -> Option<&Device> {
        for device in self.devices.iter() {
            if invariant_case {
                if device.label.to_lowercase() == device_label.to_lowercase() {
                    return Some(&device);
                }
            } else {
                if device.label == device_label {
                    return Some(&device);
                }
            }
        }

        None
    }

    pub fn get_device_by_id(&self, device_id: &str) -> Option<&Device> {
        for device in self.devices.iter() {
            if device.id() == device_id {
                return Some(&device);
            }
        }

        None
    }

    pub fn get_device(&self, identifier: &str) -> Option<&Device> {
        self.get_device_by_id(identifier)
            .or_else(|| self.get_device_by_label(identifier, true))
    }
}

#[derive(Debug)]
pub enum DeviceType {
    RollerShutter,
    GarageDoor,
    Unknown,
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl DeviceType {
    pub fn full_type(&self) -> &str {
        match self {
            Self::RollerShutter => "io:RollerShutterWithLowSpeedManagementIOComponent",
            Self::GarageDoor => "io:GarageOpenerIOComponent",
            Self::Unknown => "unknown",
        }
    }

    pub fn from_string(value: &str) -> DeviceType {
        match value {
            "io:RollerShutterWithLowSpeedManagementIOComponent" => DeviceType::RollerShutter,
            "io:GarageOpenerIOComponent" => DeviceType::GarageDoor,
            _ => DeviceType::Unknown,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Device {
    pub label: String,

    #[serde(rename = "controllableName")]
    pub controllable_name: String,
    pub definition: DeviceDefinition,

    #[serde(rename = "deviceURL")]
    pub device_url: String,
    pub enabled: bool,
}

impl Device {
    pub fn id(&self) -> &str {
        let parts: Vec<&str> = self.device_url.split('/').collect();
        parts.last().unwrap()
    }

    pub fn device_type(&self) -> DeviceType {
        DeviceType::from_string(&self.controllable_name)
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} ({})", self.id(), self.label, self.device_type())
    }
}

#[derive(Debug, Deserialize)]
pub struct DeviceDefinition {
    pub commands: Vec<DeviceCommand>,
}

#[derive(Debug, Deserialize)]
pub struct DeviceCommand {
    #[serde(rename = "nparams")]
    pub params_count: i32,
    #[serde(rename = "commandName")]
    pub command_name: String,
    #[serde(rename = "paramsSig")]
    pub params_signature: Option<String>,
}
