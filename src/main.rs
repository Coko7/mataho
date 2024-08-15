use anyhow::{anyhow, Result};
use clap::Parser;
use log::info;
use std::{fs, path::PathBuf};

mod api;
mod cli;
mod mataho;

use api::controller::TahomaApiController;
use cli::{
    model::{Configuration, DeviceTypeFilter, MatchMode},
    parser::{Cli, Commands, GroupCommands},
};
use mataho::service::MatahoService;

fn main() -> Result<()> {
    env_logger::init();

    info!("parsing cli args");
    let args = Cli::parse();

    info!("getting config file");
    let config_file_path = MatahoService::config_file_path()?;
    if !config_file_path.exists() {
        MatahoService::create_config_file()?;
        info!("config file created");
    }

    info!("getting groups file");
    let groups_file_path = MatahoService::groups_file_path()?;
    if !groups_file_path.exists() {
        MatahoService::create_groups_file()?;
        info!("groups file created");
    }

    info!("loading config");
    let config = read_config(config_file_path)?;

    info!("init Tahoma api controller");
    let controller = TahomaApiController::new(&config);

    info!("init Mataho service");
    let mut mataho_service = MatahoService::new(controller.get_setup()?);

    info!("process cli args");
    process_args(args, &controller, &mut mataho_service)?;

    Ok(())
}

fn read_config(path: PathBuf) -> Result<Configuration> {
    let json_content = fs::read_to_string(path)?;
    info!("parsing config json");
    let config: Configuration = serde_json::from_str(&json_content)?;

    Ok(config)
}

fn process_args(
    args: Cli,
    controller: &TahomaApiController,
    mataho_service: &mut MatahoService,
) -> Result<()> {
    match args.command {
        Commands::List {
            filter,
            long_listing,
        } => {
            info!("cmd::list: {}", filter);

            Ok(mataho_service.print_devices(filter, long_listing))
        }
        Commands::Info { device, match_mode } => {
            let device = device.to_string_lossy();
            info!("cmd::info: {}", device);

            let device = mataho_service.find_device(&device, match_mode)?;
            Ok(mataho_service.print_device_info(&device))
        }
        Commands::Exec {
            command,
            device,
            match_mode,
            args,
        } => {
            let device = device.to_string_lossy();
            let command = command.to_string_lossy();
            info!("cmd::exec: {} {}", device, command);

            execute_on_device(
                &controller,
                &mataho_service,
                &device,
                match_mode,
                &command,
                &args,
            )
        }
        Commands::Group { command } => match command {
            GroupCommands::List {} => {
                info!("cmd::group::list");

                Ok(mataho_service.print_groups())
            }
            GroupCommands::Create { name } => {
                let name = name.to_string_lossy();
                info!("cmd::group::create: {}", name);

                Ok(mataho_service.create_group(&name)?)
            }
            GroupCommands::Delete { name } => {
                let name = name.to_string_lossy();
                info!("cmd::group::delete: {}", name);

                Ok(mataho_service.delete_group(&name)?)
            }
            GroupCommands::AddToGroup { group, device } => {
                let group = group.to_string_lossy();
                let device = device.to_string_lossy();
                info!("cmd::group::join: add {} to {}", device, group);

                Ok(mataho_service.add_to_group(&group, &device)?)
            }
            GroupCommands::RemoveFromGroup { group, device } => {
                let group = group.to_string_lossy();
                let device = device.to_string_lossy();
                info!("cmd::group::leave: remove {} from {}", device, group);

                Ok(mataho_service.remove_from_group(&group, &device)?)
            }
            GroupCommands::Exec {
                group,
                command,
                args,
            } => {
                let group = group.to_string_lossy();
                let command = command.to_string_lossy();
                info!("cmd::group::exec: {} {}", group, command);

                execute_on_group(controller, mataho_service, &group, &command, &args)
            }
        },
    }
}

fn execute_on_group(
    controller: &TahomaApiController,
    mataho_service: &MatahoService,
    group: &str,
    command: &str,
    args: &Vec<String>,
) -> Result<()> {
    if let Some(group) = mataho_service.find_group_by_name(group) {
        let devices = mataho_service.get_group_devices(group);

        if !devices.iter().all(|device| device.supports_action(command)) {
            return Err(anyhow!(
                "Given command is not supported by all devices in the group"
            ));
        }

        controller.execute_multiple(devices, command, &args)?;

        println!(
            "Executing `{}` on group `{} ({} devices)`...",
            command,
            group.name(),
            group.devices().len()
        );
        return Ok(());
    }

    Err(anyhow!("No such group: `{}`", group))
}

fn execute_on_device(
    controller: &TahomaApiController,
    mataho_service: &MatahoService,
    device_identifier: &str,
    match_mode: MatchMode,
    command: &str,
    args: &Vec<String>,
) -> Result<()> {
    let device = mataho_service.find_device(&device_identifier, match_mode)?;
    if !device.supports_action(command) {
        return Err(anyhow!(
            "Device `{}` does not support the `{}` command",
            device_identifier,
            command
        ));
    }

    controller.execute(&device, command, args)?;

    println!("Executing `{}` on `{}`...", command, device.label());
    Ok(())
}
