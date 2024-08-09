use anyhow::{anyhow, Context, Result};
use clap::Parser;
use cli::{
    model::{Configuration, DeviceTypeFilter, MatchMode},
    parser::{Cli, Commands, GroupCommands},
};
use mataho::service::MatahoService;
use std::{env, fs};

use api::controller::TahomaApiController;

mod api;
mod cli;
mod mataho;

fn main() -> Result<()> {
    let args = Cli::parse();

    let config_path = match env::var("MATAHO_CONFIG") {
        Ok(path) => path,
        Err(_) => {
            let config_home = env::var("XDG_CONFIG_HOME").expect("XDG_CONFIG_HOME not set");
            format!("{}/mataho/config.json", config_home)
        }
    };

    let config = read_config(&config_path)?;
    let controller = TahomaApiController::new(&config);

    let mataho_service = MatahoService::new(controller.get_setup()?, config);
    process_args(args, &controller, &mataho_service)?;

    Ok(())
}

fn read_config(path: &str) -> Result<Configuration> {
    let json_content =
        fs::read_to_string(path).with_context(|| format!("Failed to read file `{}`", path))?;

    let config: Configuration = serde_json::from_str(&json_content)
        .with_context(|| format!("Failed to parse JSON from `{}`", path))?;

    Ok(config)
}

fn process_args(
    args: Cli,
    controller: &TahomaApiController,
    mataho_service: &MatahoService,
) -> Result<()> {
    match args.command {
        Commands::List { filter } => Ok(mataho_service.print_devices(filter)),
        Commands::Info { device, match_mode } => {
            match mataho_service.get_device(&device.to_string_lossy(), match_mode) {
                Ok(device) => Ok(mataho_service.print_device_info(&device)),
                Err(err) => Err(err),
            }
        }
        Commands::Exec {
            command,
            device,
            match_mode,
        } => execute_on_device(
            &controller,
            &mataho_service,
            &device.to_string_lossy(),
            &command.to_string_lossy(),
            match_mode,
        ),
        Commands::Group { command } => match command {
            GroupCommands::List {} => {
                println!("{} groups:", mataho_service.groups.len());
                for group in mataho_service.groups.iter() {
                    println!("- {}", group.name)
                }
                Ok(())
            }
            _ => Ok(()),
        },
    }
}

fn execute_on_device(
    controller: &TahomaApiController,
    matho_service: &MatahoService,
    device_identifier: &str,
    command: &str,
    match_mode: MatchMode,
) -> Result<()> {
    match matho_service.get_device(&device_identifier, match_mode) {
        Ok(device) => {
            if let Some(command_obj) = device
                .definition()
                .actions()
                .iter()
                .find(|cmd| cmd.name() == command)
            {
                controller.execute(&device, command, Vec::new())?;
                println!("Executing `{}` on `{}`...", command, device.label());

                return Ok(());
            }

            Err(anyhow!(
                "Device `{}` does not support the `{}` command",
                device_identifier,
                command
            ))
        }
        Err(err) => Err(err),
    }
}
