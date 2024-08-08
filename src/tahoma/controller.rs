use anyhow::anyhow;
use serde_json::json;

use crate::Configuration;

use super::model::{Device, TahomaSetup};

pub struct TahomaController {
    hostname: String,
    port: i32,
    api_token: String,
}

impl TahomaController {
    pub fn new(config: Configuration) -> TahomaController {
        TahomaController {
            hostname: config.hostname,
            port: config.port,
            api_token: config.api_token,
        }
    }

    fn get_client() -> reqwest::blocking::Client {
        reqwest::blocking::ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap()
    }

    pub fn get_setup(&self) -> Result<TahomaSetup, reqwest::Error> {
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

        match res.error_for_status() {
            Ok(_res) => Ok(()),
            Err(err) => Err(anyhow!("Failed to execute command: {}", err)),
        }

        // println!("{:#?}", res);
    }

    fn endpoint(&self, path: &str) -> String {
        format!("{}:{}/{}", self.hostname, self.port, path)
    }
}
