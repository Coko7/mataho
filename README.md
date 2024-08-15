# mataho

A rust command line tool that allows you to interact with your local Tahoma box.
This is a port of the cli from my [somfy-home-controller](https://github.com/coko7/somfy-home-controller) project.

New features have been added:
- Fuzzy matching for device labels
- Group management
- Executing an action on a group of devices

## 🛠️ Setup

Create a config file at `$HOME/.config/mataho/config.json`:
```json
{
    "hostname": "https://gateway-XXXX-XXXX-XXXX"
    "port": 8443,
    "api_token": "PUT_YOUR_SUPER_SECRET_TOKEN_HERE",
}
```

*NOTE: If you want to use a different path for the configuration direction, you can do so by setting the `$MATAHO_CONFIG` env variable.*

*You must keep the filename as `config.json` though.*

## 🐚 Usage

Typing `mataho help` will print the documentation:
```
$ mataho help

Interact with your Tahoma box in the terminal

Usage: mataho [OPTIONS] <COMMAND>

Commands:
  list   Print the list of known local devices [aliases: ls]
  info   Get information about a particular device (id, label, supported actions, etc.)
  exec   Execute a Tahoma action on a single device [aliases: ex]
  group  Create and manage groups of devices [aliases: grp]
  help   Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help
```

Display local devices:
```
$ mataho ls

+----------+-----------------+----------------+
| ID       | Label           | Type           |
+----------+-----------------+----------------+
| 00000001 | Front gate      | sliding gate   |
+----------+-----------------+----------------+
| 00000002 | Garage          | garage opener  |
+----------+-----------------+----------------+
| 00000003 | Coko's room     | roller shutter |
+----------+-----------------+----------------+
```

Execute a command/action on a particular device (fuzzy matching is used to find the device):
```
$ mataho ex coko open

Executing `open` on `Coko's room`...
```

Action arguments are supported as well:
```
$ mataho exec coco setClosureAndLinearSpeed 20 lowspeed

Executing `setClosureAndLinearSpeed` on `Coko's room`...
```

Manage groups:
```
$ mataho grp -h

Create and manage groups of devices

Usage: mataho group [OPTIONS] <COMMAND>

Commands:
  list    List all groups [aliases: ls]
  create  Create a new group
  delete  Delete a group
  join    Add a device to an existing group
  leave   Remove a device from an exiting group
  exec    Execute a Tahoma action on a group of devices [aliases: ex]
  help    Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help
```

## 📚 Resources

- GitHub repo of [Somfy-TaHoma-Developer-Mode](https://github.com/Somfy-Developer/Somfy-TaHoma-Developer-Mode)
- Swagger UI for [local gateway API](https://somfy-developer.github.io/Somfy-TaHoma-Developer-Mode/)
