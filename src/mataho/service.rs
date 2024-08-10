use core::panic;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::anyhow;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use xdg::BaseDirectories;

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
        let groups = match MatahoService::read_groups_from_file() {
            Ok(val) => val,
            Err(err) => {
                println!("Error: {}", err);
                Vec::new()
            }
        };

        MatahoService {
            devices: response.devices,
            groups,
        }
    }

    pub fn get_config_dir() -> Result<PathBuf, anyhow::Error> {
        let app_name = "mataho";
        let mataho_config_var = "MATAHO_CONFIG";

        if env::var(mataho_config_var).is_ok() {
            return Ok(PathBuf::from(mataho_config_var));
        }

        if let Ok(xdg_dirs) = BaseDirectories::new() {
            let config_home = xdg_dirs.get_config_home();
            return Ok(config_home.join(app_name));
        }

        if let Ok(home_dir) = env::var("HOME") {
            return Ok(PathBuf::from(home_dir).join(".config").join(app_name));
        }

        Err(anyhow!("Failed to find config dir"))
    }

    pub fn config_file_path() -> Result<PathBuf, anyhow::Error> {
        Ok(Self::get_config_dir()?.join("config.json"))
    }

    pub fn groups_file_path() -> Result<PathBuf, anyhow::Error> {
        Ok(Self::get_config_dir()?.join("groups.json"))
    }

    fn read_groups_from_file() -> Result<Vec<DeviceGroup>, anyhow::Error> {
        let file_path = Self::groups_file_path()?;

        let json = fs::read_to_string(file_path)?;
        let groups: Vec<DeviceGroup> = serde_json::from_str(&json)?;

        Ok(groups)
    }

    fn write_groups_to_file(groups: &Vec<DeviceGroup>) -> Result<(), anyhow::Error> {
        let file_path = Self::groups_file_path()?;

        let json = serde_json::to_string(groups)?;
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
                .map(|device_id| {
                    format!("`{}`", self.find_device_by_id(device_id).unwrap().label())
                })
                .collect::<Vec<String>>()
                .join(", ");

            println!("- {}: {}", group.name(), &device_labels);
        }
    }

    pub fn print_devices(&self, filter: DeviceTypeFilter) {
        for device in &self.devices {
            if filter == DeviceTypeFilter::All || device.has_type(filter) {
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

    pub fn get_group_devices(&self, group: &DeviceGroup) -> Vec<&Device> {
        let mut res: Vec<&Device> = Vec::new();

        for device_id in group.devices() {
            let device = self.find_device_by_id(device_id).unwrap();
            res.push(device);
        }

        res
    }

    pub fn create_group(&mut self, name: &str) -> Result<(), anyhow::Error> {
        if self.find_group_by_name(name).is_some() {
            return Err(anyhow!("There is already a group named `{}`", name));
        }

        let group = DeviceGroup::new(name);
        self.groups.push(group);
        Self::write_groups_to_file(&self.groups)?;

        Ok(())
    }

    pub fn delete_group(&mut self, name: &str) -> Result<(), anyhow::Error> {
        if let Some(pos) = self.groups.iter().position(|group| group.name() == name) {
            self.groups.remove(pos);
            Self::write_groups_to_file(&self.groups)?;
            return Ok(());
        }

        Err(anyhow!("No such group: `{}`", name))
    }

    pub fn add_to_group(&mut self, group_name: &str, device: &str) -> Result<(), anyhow::Error> {
        let device_id = {
            let device = self.find_device(device, MatchMode::Fuzzy)?;
            device.id().to_string()
        };

        if let Some(group) = self.find_group_by_name_mut(group_name) {
            group.add_device(&device_id)?;
            Self::write_groups_to_file(&self.groups)?;
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
            let device = self.find_device(device, MatchMode::Fuzzy)?;
            device.id().to_string()
        };

        if let Some(group) = self.find_group_by_name_mut(group_name) {
            group.remove_device(&device_id)?;
            Self::write_groups_to_file(&self.groups)?;
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

    pub fn find_device_by_label(
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

    pub fn find_device_by_id(&self, id: &str) -> Option<&Device> {
        self.devices.iter().find(|device| device.id() == id)
    }

    pub fn find_device(
        &self,
        identifier: &str,
        match_mode: MatchMode,
    ) -> Result<&Device, anyhow::Error> {
        if let Some(device) = self.find_device_by_id(identifier) {
            return Ok(device);
        }

        self.find_device_by_label(identifier, match_mode)
    }
}
