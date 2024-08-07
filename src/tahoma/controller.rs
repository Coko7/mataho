use serde_json::json;

use crate::Configuration;

use super::model::{Device, DeviceType, TahomaSetup};

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

        // println!("{:#?}", res);
        Ok(res.json()?)
    }

    pub fn execute(
        &self,
        device: &Device,
        command: &str,
        params: Vec<String>,
    ) -> Result<(), reqwest::Error> {
        let client = Self::get_client();

        let payload = json!({
            "label": format!("Exec {} on {}", command, device.device_url),
            "actions":
            [
                {
                    "commands": [
                        {
                            "name": command,
                            "parameters": params
                        }
                    ],
                    "deviceURL": device.device_url
                }
            ]
        });

        let res = client
            .post(&self.endpoint("/enduser-mobile-web/1/enduserAPI/exec/apply"))
            .bearer_auth(&self.api_token)
            .json(&payload)
            .send()?;

        Ok(())
    }

    pub fn get_devices_with_type(
        &self,
        device_type: DeviceType,
    ) -> Result<Vec<Device>, reqwest::Error> {
        let devices = self.get_setup()?.devices;

        let filtered_devices = devices
            .into_iter()
            .filter(|dev| dev.controllable_name == device_type.full_type());

        Ok(filtered_devices.collect())
    }

    pub fn get_all_devices(&self) -> Result<Vec<Device>, reqwest::Error> {
        Ok(self.get_setup()?.devices)
    }

    fn endpoint(&self, path: &str) -> String {
        format!("{}:{}/{}", self.hostname, self.port, path)
    }
}
