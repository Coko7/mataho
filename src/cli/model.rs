use std::fmt;

use clap::ValueEnum;
use serde::Deserialize;

use crate::api::device::Device;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub hostname: String,
    pub port: i32,
    // pod: String,
    pub api_token: String,
    pub groups: Vec<DeviceGroup>,
}

#[derive(Debug, Deserialize)]
pub struct DeviceGroup {
    pub id: String,
    pub name: String,
    pub devices: Vec<Device>,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum MatchMode {
    Exact,
    Fuzzy,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum DeviceTypeFilter {
    All,
    GarageDoor,
    Gate,
    RollerShutter,
}

impl DeviceTypeFilter {
    pub fn as_str(&self) -> &str {
        match self {
            DeviceTypeFilter::All => "",
            DeviceTypeFilter::GarageDoor => "io:GarageOpenerIOComponent",
            DeviceTypeFilter::Gate => "io:SlidingDiscreteGateOpenerIOComponent",
            DeviceTypeFilter::RollerShutter => "io:RollerShutterWithLowSpeedManagementIOComponent",
        }
    }
}

impl fmt::Display for DeviceTypeFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}
