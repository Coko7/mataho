use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use serde::Deserialize;
use std::{env, ffi::OsString, fs};
use tahoma::{
    controller::TahomaController,
    model::{DeviceTypeFilter, TahomaSetup},
};

mod tahoma;

fn main() -> Result<()> {
    let args = Cli::parse();

    let config_path = match env::var("TAHOMA_CLI_CONFIG") {
        Ok(path) => path,
        Err(_) => {
            let config_home = env::var("XDG_CONFIG_HOME").expect("XDG_CONFIG_HOME not set");
            format!("{}/tahoma-cli/config.json", config_home)
        }
    };

    let config = read_config(&config_path)?;
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
#[command(name = "tahoma")]
#[command(about = "Interact with your Tahoma box in the terminal", long_about = None)]
struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Print the list of known local devices
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
    /// Get information about a particular device (id, label, supported commands, etc.)
    Info {
        /// Label, ID or URL of a device
        device: OsString,
        /// Match mode for the device 
        #[arg(
            long, 
            require_equals = true,
            value_name = "MODE",
            num_args = 0..=1,
            default_value_t = MatchMode::Fuzzy,
            default_missing_value = "fuzzy",
            value_enum)]
        match_mode: MatchMode
    },
    /// Execute a Tahoma command on a single device
    Exec {
        /// ID or label of the device. See match-mode for label matching
        device: OsString,
        /// Match mode for the device 
        #[arg(
            long, 
            require_equals = true,
            value_name = "MODE",
            num_args = 0..=1,
            default_value_t = MatchMode::Fuzzy,
            default_missing_value = "fuzzy",
            value_enum)]
        match_mode: MatchMode,
        /// Name of the command (see list-cmds for help)
        command: OsString
    },
    // RunMulti {
    //     /// Name of the command (see list-cmds for help)
    //     command: OsString,
    //     /// A string 
    //     devices_query: OsString,
    // }
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum MatchMode {
    Exact,
    Fuzzy
}

fn process_args(args: Cli, controller: &TahomaController, setup: &TahomaSetup) -> Result<()> {
    match args.command {
        Commands::List { filter } => {
            Ok(setup.print_devices(filter))
        }
        Commands::Info { device, match_mode } => {
            match setup.get_device(&device.to_string_lossy(), match_mode) {
                Ok(device) => Ok(setup.print_device_info(&device)),
                Err(err) => Err(err)
            }
        }
        Commands::Exec { command, device, match_mode } => {
            execute_on_device(&controller, &setup, &device.to_string_lossy(), &command.to_string_lossy(), match_mode)
        }
    }
}

fn execute_on_device(controller: &TahomaController, setup: &TahomaSetup, device_identifier: &str, command: &str, match_mode: MatchMode) -> Result<()> {
    match setup.get_device(&device_identifier, match_mode) {
        Ok(device) => {
            if let Some(command_obj) = device.definition().commands().iter().find(|cmd| cmd.name() == command) {
                controller.execute(&device, command, Vec::new())?;
                println!("Executing `{}` on `{}`...", command, device.label());

                return Ok(());
            }

            Err(anyhow!("Device `{}` does not support the `{}` command", device_identifier, command))
        }
        Err(err) => Err(err)
    }
}
