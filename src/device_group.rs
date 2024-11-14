use anyhow::{anyhow, Result};
use log::error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceGroup {
    id: String,
    name: String,
    devices: Vec<String>,
}

impl DeviceGroup {
    pub fn new(name: &str) -> DeviceGroup {
        DeviceGroup {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            devices: Vec::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn devices(&self) -> &Vec<String> {
        &self.devices
    }

    pub fn has_device(&self, device_id: &str) -> bool {
        self.devices.iter().any(|id| id == device_id)
    }

    pub fn add_device(&mut self, device_id: &str) -> Result<()> {
        if self.has_device(device_id) {
            error!("Device `{}` already in group `{}`", device_id, self.name());
            return Err(anyhow!("Device already in group"));
        }

        Ok(self.devices.push(device_id.to_string()))
    }

    pub fn remove_device(&mut self, device_id: &str) -> Result<()> {
        if !self.has_device(device_id) {
            error!("Device `{}` not in group `{}`", device_id, self.name());
            return Err(anyhow!("Device not in group"));
        }

        Ok(self.devices.retain(|id| id != device_id))
    }
}
