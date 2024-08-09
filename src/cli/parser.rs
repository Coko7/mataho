use std::ffi::OsString;

use clap::{Parser, Subcommand};

use super::model::{DeviceTypeFilter, MatchMode};

#[derive(Debug, Parser)]
#[command(name = "mataho")]
#[command(about = "Interact with your Tahoma box in the terminal", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
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
    /// Create and manage groups of devices
    Group {
        #[command(subcommand)]
        command: GroupCommands
    }
}

#[derive(Debug, Subcommand)]
pub enum GroupCommands {
    /// List all groups
    List { },
    /// Create a new group
    #[command(name = "create")]
    CreateGroup {
        /// Name of the group
        name: OsString
    },
    /// Add a device to an existing group
    #[command(name = "join")]
    AddToGroup {
        /// Name of the group in which to add the device
        group: OsString,
        /// ID or label of the device. See match-mode for label matching
        device: OsString,
    },
    /// Remove a device from an exiting group
    #[command(name = "leave")]
    RemoveFromGroup {
        /// Name of the group from which to remove the device
        group: OsString,
        /// ID or label of the device. See match-mode for label matching
        device: OsString,
    },
    /// Delete a group
    #[command(name = "delete")]
    DeleteGroup {
        /// Name of the group
        name: OsString
    }
}

