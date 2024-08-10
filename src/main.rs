use anyhow::{anyhow, Context, Result};
use clap::Parser;
use cli::{
    model::{Configuration, DeviceTypeFilter, MatchMode},
    parser::{Cli, Commands, GroupCommands},
};
use mataho::service::MatahoService;
use std::fs;

use api::controller::TahomaApiController;

mod api;
mod cli;
mod mataho;

fn main() -> Result<()> {
    let args = Cli::parse();

    let config_path = MatahoService::config_file_path();
    let config = read_config(&config_path)?;
    let controller = TahomaApiController::new(&config);

    let mut mataho_service = MatahoService::new(controller.get_setup()?);
    process_args(args, &controller, &mut mataho_service)?;

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
    mataho_service: &mut MatahoService,
) -> Result<()> {
    match args.command {
        Commands::List { filter } => Ok(mataho_service.print_devices(filter)),
        Commands::Info { device, match_mode } => {
            let device = mataho_service.find_device(&device.to_string_lossy(), match_mode)?;
            Ok(mataho_service.print_device_info(&device))
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
        Commands::Group { command } => {
            match command {
                GroupCommands::List {} => Ok(mataho_service.print_groups()),
                GroupCommands::Create { name } => {
                    Ok(mataho_service.create_group(&name.to_string_lossy())?)
                }
                GroupCommands::Delete { name } => {
                    Ok(mataho_service.delete_group(&name.to_string_lossy())?)
                }
                GroupCommands::AddToGroup { group, device } => Ok(mataho_service
                    .add_to_group(&group.to_string_lossy(), &device.to_string_lossy())?),
                GroupCommands::RemoveFromGroup { group, device } => Ok(mataho_service
                    .remove_from_group(&group.to_string_lossy(), &device.to_string_lossy())?),
                GroupCommands::Exec { group, command } => execute_on_group(
                    controller,
                    mataho_service,
                    &group.to_string_lossy(),
                    &command.to_string_lossy(),
                ),
            }
        }
    }
}

fn execute_on_group(
    controller: &TahomaApiController,
    mataho_service: &MatahoService,
    group: &str,
    command: &str,
) -> Result<()> {
    if let Some(group) = mataho_service.find_group_by_name(group) {
        let devices = mataho_service.get_group_devices(group);

        if !devices.iter().all(|device| device.supports_action(command)) {
            return Err(anyhow!(
                "Given command is not supported by all devices in the group"
            ));
        }

        controller.execute_multiple(devices, command, Vec::new())?;

        println!("Executing `{}` on group `{}`...", command, group.name());
        return Ok(());
    }

    Err(anyhow!("No such group: `{}`", group))
}

fn execute_on_device(
    controller: &TahomaApiController,
    mataho_service: &MatahoService,
    device_identifier: &str,
    command: &str,
    match_mode: MatchMode,
) -> Result<()> {
    let device = mataho_service.find_device(&device_identifier, match_mode)?;

    if !device.supports_action(command) {
        return Err(anyhow!(
            "Device `{}` does not support the `{}` command",
            device_identifier,
            command
        ));
    }

    controller.execute(&device, command, Vec::new())?;

    println!("Executing `{}` on `{}`...", command, device.label());
    Ok(())
}
