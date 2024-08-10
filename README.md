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

Usage: mataho <COMMAND>

Commands:
  list   Print the list of known local devices [aliases: ls]
  info   Get information about a particular device (id, label, supported actions, etc.)
  exec   Execute a Tahoma action on a single device
  group  Create and manage groups of devices [aliases: grp]
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

Display local devices:
```
$ mataho ls

00000001: Front gate (io:SlidingDiscreteGateOpenerIOComponent)
00000002: Garage (io:GarageOpenerIOComponent)
00000003: Coko's room (io:RollerShutterWithLowSpeedManagementIOComponent)
```

Execute a command/action on a particular device (fuzzy matching is used to find the device):
```
$ mataho exec coko open

Executing `open` on `Coko's room`...
```

Manage groups:
```
$ mataho grp -h

Create and manage groups of devices

Usage: mataho group <COMMAND>

Commands:
  list    List all groups [aliases: ls]
  create  Create a new group
  join    Add a device to an existing group
  leave   Remove a device from an exiting group
  delete  Delete a group
  exec    Execute a Tahoma action on a group of devices
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## 📚 Resources

- GitHub repo of [Somfy-TaHoma-Developer-Mode](https://github.com/Somfy-Developer/Somfy-TaHoma-Developer-Mode)
- Swagger UI for [local gateway API](https://somfy-developer.github.io/Somfy-TaHoma-Developer-Mode/)
