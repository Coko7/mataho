use std::fmt;

use anyhow::anyhow;
use clap::ValueEnum;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use serde::Deserialize;

use crate::MatchMode;

#[derive(Debug, Deserialize)]
pub struct TahomaSetup {
    pub devices: Vec<Device>,
}

impl TahomaSetup {
    pub fn print_devices(&self, filter: DeviceTypeFilter) {
        for device in &self.devices {
            if filter == DeviceTypeFilter::All || device.matches(filter) {
                println!("{}", device);
            }
        }
    }

    pub fn print_device_info(&self, device: &Device) {
        println!("- label: {}", device.label());
        println!("- url: {}", device.url());
        println!("- id: {} (last part of URL)", device.id());
        println!("- commands:");

        for command in device.definition().commands().iter() {
            println!("\t- {}", command);
        }
    }

    pub fn get_device_by_label(
        &self,
        label: &str,
        match_mode: MatchMode,
    ) -> Result<&Device, anyhow::Error> {
        let label = label.to_lowercase();

        let matcher = SkimMatcherV2::default();

        let mut best_score = -1;
        let mut devices_scores: Vec<(&Device, i64)> = Vec::new();

        for device in self.devices.iter() {
            if device.label().to_lowercase() == label {
                return Ok(device);
            }

            // Skip fuzzy matching when match mode is `exact`
            if match_mode == MatchMode::Exact {
                continue;
            }

            if let Some(score) = matcher.fuzzy_match(&device.label.to_lowercase(), &label) {
                // println!("{} -> {}", device.label, score);
                devices_scores.push((device, score));
                if score > best_score {
                    best_score = score;
                }
            }
        }

        let mut best_candidates = devices_scores.iter().filter(|tuple| tuple.1 == best_score);

        // More than one candidate
        if best_candidates.clone().count() > 1 {
            let candidates_labels: String = best_candidates
                .map(|tuple| format!("`{}`", tuple.0.label))
                .collect::<Vec<String>>()
                .join(", ");

            return Err(anyhow!(
                "Failed to find a single best match, there are several candidates: {}",
                candidates_labels
            ));
        }

        if let Some(best_match) = best_candidates.next() {
            return Ok(best_match.0);
        }

        Err(anyhow!("Failed to find a device that matches: `{}`", label))
    }

    pub fn get_device_by_id(&self, device_id: &str) -> Option<&Device> {
        for device in self.devices.iter() {
            if device.id() == device_id {
                return Some(&device);
            }
        }

        None
    }

    pub fn get_device(
        &self,
        identifier: &str,
        match_mode: MatchMode,
    ) -> Result<&Device, anyhow::Error> {
        if let Some(device) = self.get_device_by_id(identifier) {
            return Ok(device);
        }

        self.get_device_by_label(identifier, match_mode)
    }
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum DeviceTypeFilter {
    All,
    GarageDoor,
    Gate,
    RollerShutter,
}

impl fmt::Display for DeviceTypeFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
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
    commands: Vec<DeviceCommand>,
}

impl DeviceDefinition {
    pub fn commands(&self) -> &Vec<DeviceCommand> {
        &self.commands
    }
}

#[derive(Debug, Deserialize)]
pub struct DeviceCommand {
    #[serde(rename = "nparams")]
    params_count: i32,
    #[serde(rename = "commandName")]
    name: String,
    #[serde(rename = "paramsSig")]
    params_signature: Option<String>,
}

impl DeviceCommand {
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

impl fmt::Display for DeviceCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(params) = &self.params_signature {
            return write!(f, "{}: [{}] ({})", self.name, params, self.params_count);
        }

        write!(f, "{}", self.name)
    }
}
