use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

use anyhow::anyhow;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

use crate::{
    api::{controller::TahomaSetupResponse, device::Device},
    cli::model::{DeviceGroup, DeviceTypeFilter, MatchMode},
};

pub struct MatahoService {
    devices: Vec<Device>,
    groups: Vec<DeviceGroup>,
}

impl MatahoService {
    pub fn new(response: TahomaSetupResponse) -> MatahoService {
        MatahoService {
            devices: response.devices,
            groups: MatahoService::read_from_file(),
        }
    }

    fn groups_file_path() -> String {
        match env::var("MATAHO_CONFIG") {
            Ok(path) => path,
            Err(_) => {
                let config_home = env::var("XDG_CONFIG_HOME").expect("XDG_CONFIG_HOME not set");
                format!("{}/mataho/groups.json", config_home)
            }
        }
    }

    pub fn read_from_file() -> Vec<DeviceGroup> {
        let file_path = Self::groups_file_path();

        if !Path::new(&file_path).exists() {
            return Vec::new();
        }

        let json = fs::read_to_string(file_path).expect("Failed to rerad file");
        let groups: Vec<DeviceGroup> = serde_json::from_str(&json).expect("cwewe");

        groups
    }

    pub fn write_to_file(&self) -> std::io::Result<()> {
        let file_path = Self::groups_file_path();

        let json = serde_json::to_string(&self.groups).expect("Failed to serialize groups");
        fs::write(file_path, json)?;
        Ok(())
    }

    pub fn print_groups(&self) {
        if self.groups.len() == 0 {
            println!("No group");
            return;
        }

        println!("{} groups:", self.groups.len());
        for group in self.groups.iter() {
            let device_labels: String = group
                .devices()
                .iter()
                .map(|device_id| format!("`{}`", self.get_device_by_id(device_id).unwrap().label()))
                .collect::<Vec<String>>()
                .join(", ");

            println!("- {}: {}", group.name(), &device_labels);
        }
    }

    pub fn print_devices(&self, filter: DeviceTypeFilter) {
        for device in &self.devices {
            if filter == DeviceTypeFilter::All || device.matches(filter) {
                println!("{}", device);
            }
        }
    }

    pub fn find_group_by_name(&self, name: &str) -> Option<&DeviceGroup> {
        self.groups.iter().find(|group| group.name() == name)
    }

    pub fn find_group_by_name_mut(&mut self, name: &str) -> Option<&mut DeviceGroup> {
        self.groups.iter_mut().find(|group| group.name() == name)
    }

    pub fn create_group(&mut self, name: &str) -> Result<(), anyhow::Error> {
        let duplicate = self.find_group_by_name(name);
        if duplicate.is_some() {
            return Err(anyhow!(
                "Cannot create group because `{}` is already being used",
                name
            ));
        }

        let group = DeviceGroup::new(name);
        self.groups.push(group);
        self.write_to_file()?;

        Ok(())
    }

    pub fn delete_group(&mut self, name: &str) -> Result<(), anyhow::Error> {
        if let Some(index) = self.groups.iter().position(|group| group.name() == name) {
            self.groups.remove(index);
            self.write_to_file()?;
            return Ok(());
        }

        Err(anyhow!("No such group: `{}`", name))
    }

    pub fn add_to_group(&mut self, group_name: &str, device: &str) -> Result<(), anyhow::Error> {
        let device_id = {
            let device = self.get_device(device, MatchMode::Fuzzy)?;
            device.id().to_string()
        };

        if let Some(group) = self.find_group_by_name_mut(group_name) {
            group.add_device(&device_id)?;
            self.write_to_file()?;
            return Ok(());
        }

        Err(anyhow!("No such group: `{}`", group_name))
    }

    pub fn remove_from_group(
        &mut self,
        group_name: &str,
        device: &str,
    ) -> Result<(), anyhow::Error> {
        let device_id = {
            let device = self.get_device(device, MatchMode::Fuzzy)?;
            device.id().to_string()
        };

        if let Some(group) = self.find_group_by_name_mut(group_name) {
            group.remove_device(&device_id)?;
            self.write_to_file()?;
            return Ok(());
        }

        Err(anyhow!("No such group: `{}`", group_name))
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
