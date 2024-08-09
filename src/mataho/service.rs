use anyhow::anyhow;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

use crate::{
    api::{controller::TahomaSetupResponse, device::Device},
    cli::model::DeviceGroup,
    Configuration, DeviceTypeFilter, MatchMode,
};

pub struct MatahoService {
    pub devices: Vec<Device>,
    pub groups: Vec<DeviceGroup>,
}

impl MatahoService {
    pub fn new(response: TahomaSetupResponse, config: Configuration) -> MatahoService {
        MatahoService {
            devices: response.devices,
            groups: config.groups,
        }
    }

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

        for command in device.definition().actions().iter() {
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

            if let Some(score) = matcher.fuzzy_match(&device.label().to_lowercase(), &label) {
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
                .map(|tuple| format!("`{}`", tuple.0.label()))
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
