use anyhow::anyhow;
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

    pub fn get_setup(&self) -> Result<TahomaSetupResponse, reqwest::Error> {
        let client = Self::get_client();

        let res = client
            .get(self.endpoint("/enduser-mobile-web/1/enduserAPI/setup"))
            .bearer_auth(&self.api_token)
            .send()?;

        // println!("{:#?}", res);rus
        Ok(res.json()?)
    }

    pub fn execute(
        &self,
        device: &Device,
        command: &str,
        params: Vec<String>,
    ) -> Result<(), anyhow::Error> {
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

        let res = client
            .post(&self.endpoint("/enduser-mobile-web/1/enduserAPI/exec/apply"))
            .bearer_auth(&self.api_token)
            .json(&payload)
            .send()?;

        // println!("{:#?}", res);

        match res.error_for_status() {
            Ok(_res) => Ok(()),
            Err(err) => Err(anyhow!("Failed to execute command: {}", err)),
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
