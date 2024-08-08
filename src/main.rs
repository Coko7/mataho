use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::{ffi::OsString, fs};
use tahoma::{
    controller::TahomaController,
    model::{DeviceTypeFilter, TahomaSetup},
};

mod tahoma;

fn main() -> Result<()> {
    let args = Cli::parse();

    let config = read_config("config.json")?;
    let controller = TahomaController::new(config);

    let setup = controller.get_setup()?;

    process_args(args, &controller, &setup)?;

    Ok(())
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

#[derive(Debug, Parser)]
#[command(name = "taho")]
#[command(about = "Interact with your Tahoma box in the terminal", long_about = None)]
struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// List local devices
    List {
        /// Only display a subcategory of devices
        #[arg(
            long, 
            require_equals = true,
            value_name = "TYPE",
            num_args = 0..=1,
            default_value_t = DeviceTypeFilter::All,
            default_missing_value = "all",
            value_enum)]
        filter: DeviceTypeFilter,
    },
    /// List all commands supported by a device
    #[command(name = "list-cmds")]
    ListCommands {
        /// Label, ID or URL of a device
        device: OsString,
    },
    /// Trigger 'Open' action on device
    Open {
        /// Label, ID or URL of a device
        device: OsString,
    },
    /// Trigger 'Close' action on device
    Close {
        /// Label, ID or URL of a device
        device: OsString,
    },
    /// Trigger 'Stop' action on device
    Stop {
        /// Label, ID or URL of a device
        device: OsString,
    }
}

fn process_args(args: Cli, controller: &TahomaController, setup: &TahomaSetup) -> Result<()> {
    match args.command {
        Commands::List { filter } => {
            return Ok(setup.print_devices(filter));
        }
        Commands::ListCommands { device } => {
            if let Some(device) = setup.get_device(&device.to_string_lossy()) {
                return Ok(setup.print_device_commands(&device));
            }

            return Ok(());
        }
        Commands::Open { device } => {
            return execute_on_device(&controller, &setup, &device.to_string_lossy(), "open");
        }
        Commands::Close { device } => {
            return execute_on_device(&controller, &setup, &device.to_string_lossy(), "close");
        }
        Commands::Stop { device } => {
            return execute_on_device(&controller, &setup, &device.to_string_lossy(), "stop");
        }
    }
}

fn execute_on_device(controller: &TahomaController, setup: &TahomaSetup, device_identifier: &str, command: &str) -> Result<()> {
    if let Some(device) = setup.get_device(&device_identifier) {
        controller.execute(&device, command, Vec::new())?;
        println!("Executing `{}` on `{}`...", command, device.label());
        return Ok(());
    }

    Err(anyhow!("Unknown device `{}`", device_identifier))
}
