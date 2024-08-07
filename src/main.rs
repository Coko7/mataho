use anyhow::{anyhow, Context, Result};
use clap::Parser;
use serde::Deserialize;
use std::fs;
use tahoma::{controller::TahomaController, model::TahomaSetup};

mod tahoma;

fn main() -> Result<()> {
    let args = Cli::parse();
    let config = read_config("config.json")?;

    let controller = TahomaController::new(config);
    let setup = controller.get_setup()?;

    match args.command.as_str() {
        "list" => list_devices(&setup),
        cmd => {
            let device_arg = args.device.expect("Expected a device name");

            let device = setup
                .get_device(&device_arg)
                .ok_or_else(|| anyhow!("No known device matches `{}`", device_arg))?;

            let _ = controller.execute(device, cmd, Vec::new());
        } // _ => println!("Unsupported command: {}", command),
    }

    Ok(())
}

fn list_devices(setup: &TahomaSetup) {
    for device in &setup.devices {
        println!("{}", device);
    }
}

#[derive(Debug, Deserialize)]
struct Configuration {
    hostname: String,
    port: i32,
    // pod: String,
    api_token: String,
}

fn read_config(path: &str) -> Result<Configuration> {
    let json_content =
        fs::read_to_string(path).with_context(|| format!("Failed to read file `{}`", path))?;

    let config: Configuration = serde_json::from_str(&json_content)
        .with_context(|| format!("Failed to parse JSON from `{}`", path))?;

    Ok(config)
}

#[derive(Parser)]
struct Cli {
    /// Name of the command to execute
    #[arg(short = 'c', long = "command")]
    command: String,

    /// Device identifier (either ID or label)
    #[arg(short = 'd', long = "device")]
    device: Option<String>,
    // path: std::path::PathBuf,
}
