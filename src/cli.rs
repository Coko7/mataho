use clap::{Parser, Subcommand};
use std::ffi::OsString;

use super::model::{DeviceTypeFilter, MatchMode};

#[derive(Debug, Parser)]
#[command(name = "mataho")]
#[command(about = "Interact with your Tahoma box in the terminal", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Print the list of known local devices
    #[command(visible_alias("ls"))]
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
        /// Use long listing format
        #[arg(short = 'l', action)]
        long_listing: bool,
    },
    /// Get information about a particular device (id, label, supported actions, etc.)
    Info {
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
    },
    /// Execute a Tahoma action on a single device
    #[command(visible_alias("ex"))]
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
        /// Name of the command
        command: OsString,
        /// Command arguments
        #[arg(num_args(0..))]
        args: Vec<String>,
    },
    /// Create and manage groups of devices
    #[command(visible_alias("grp"))]
    Group {
        #[command(subcommand)]
        command: GroupCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum GroupCommands {
    /// List all groups
    #[command(visible_alias("ls"))]
    List {},
    /// Create a new group
    #[command(name = "create")]
    Create {
        /// Name of the group
        name: OsString,
    },
    /// Delete a group
    #[command(name = "delete")]
    Delete {
        /// Name of the group
        name: OsString,
    },
    /// Add a device to an existing group
    #[command(name = "join")]
    AddToGroup {
        /// Name of the group
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
    /// Execute a Tahoma action on a group of devices
    #[command(visible_alias("ex"))]
    Exec {
        /// Name of the group
        group: OsString,
        /// Name of the command
        command: OsString,
        /// Command arguments
        #[arg(num_args(0..))]
        args: Vec<String>,
    },
}
