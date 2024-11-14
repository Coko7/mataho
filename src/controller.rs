use anyhow::{anyhow, Context, Result};
use log::{debug, info};
use serde::Deserialize;
use serde_json::json;

use crate::Configuration;

use super::device::Device;

pub struct TahomaApiController {
    hostname: String,
    port: i32,
    api_token: String,
}

impl TahomaApiController {
    pub fn new(configuration: &Configuration) -> TahomaApiController {
        TahomaApiController {
            hostname: configuration.hostname.clone(),
            port: configuration.port,
            api_token: configuration.api_token.clone(),
        }
    }

    fn get_client() -> reqwest::blocking::Client {
        reqwest::blocking::ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap()
    }

    pub fn get_setup(&self) -> Result<TahomaSetupResponse> {
        let client = Self::get_client();

        let url = self.endpoint("/enduser-mobile-web/1/enduserAPI/setup");
        info!("GET {}", url);

        let res = client
            .get(url)
            .bearer_auth(&self.api_token)
            .send()
            .context("Failed to get setup. Check your configuration file.")?;

        let res = res.json()?;
        debug!("result: {:?}", res);

        Ok(res)
    }

    pub fn execute(&self, device: &Device, command: &str, params: &Vec<String>) -> Result<()> {
        let client = Self::get_client();

        let payload = json!({
            "label": format!("Exec {} on {}", command, device.url()),
            "actions":
            [
                {
                    "commands": [
                        {
                            "name": command,
                            "parameters": params
                        }
                    ],
                    "deviceURL": device.url()
                }
            ]
        });

        let url = self.endpoint("/enduser-mobile-web/1/enduserAPI/exec/apply");
        info!("POST {} {}", url, serde_json::to_string(&payload)?);

        let res = client
            .post(url)
            .bearer_auth(&self.api_token)
            .json(&payload)
            .send()?;

        debug!("result: {:?}", res);

        match res.error_for_status() {
            Ok(_res) => Ok(()),
            Err(err) => Err(anyhow!("Failed to execute command: {}", err)),
        }
    }

    pub fn execute_multiple(
        &self,
        devices: Vec<&Device>,
        command: &str,
        params: &Vec<String>,
    ) -> Result<()> {
        let client = Self::get_client();

        let mut all_actions = Vec::new();
        for device in devices.iter() {
            let action = json!({
                "commands": [
                    {
                        "name": command,
                        "parameters": params
                    }
                ],
                "deviceURL": device.url()
            });

            all_actions.push(action);
        }

        let payload = json!({
            "label": format!("Exec {} on {} devices", command, devices.len()),
            "actions": all_actions
        });

        let url = self.endpoint("/enduser-mobile-web/1/enduserAPI/exec/apply");
        info!("POST {} {}", url, serde_json::to_string(&payload)?);

        let res = client
            .post(url)
            .bearer_auth(&self.api_token)
            .json(&payload)
            .send()?;

        debug!("result: {:?}", res);

        match res.error_for_status() {
            Ok(_res) => Ok(()),
            Err(err) => Err(anyhow!(
                "Failed to execute command on multiple devices: {}",
                err
            )),
        }
    }

    fn endpoint(&self, path: &str) -> String {
        format!("{}:{}/{}", self.hostname, self.port, path)
    }
}

#[derive(Debug, Deserialize)]
pub struct TahomaSetupResponse {
    pub devices: Vec<Device>,
}
