# mataho

A rust port of the cli from my [somfy-home-controller](https://github.com/coko7/somfy-home-controller) project.

## 🛠️ Setup

1. Create a `mataho` directory in `$XDG_CONFIG_HOME`
2. Create `config.json` in this new directory and put the following inside:
```json
{
    "hostname": "https://gateway-XXXX-XXXX-XXXX"
    "port": 8443,
    "api_token": "PUT_YOUR_SUPER_SECRET_TOKEN_HERE",
}
```

*NOTE: If you want to use a different config path, you can do so by setting the `$MATAHO_CONFIG` env variable.*

## 🐚 Usage

Typing `mataho help` will print the documentation:
```
$ mataho help

Interact with your Tahoma box in the terminal

Usage: mataho <COMMAND>

Commands:
  list  Print the list of known local devices
  info  Get information about a particular device (id, label, supported commands, etc.)
  exec  Execute a Tahoma command on a single device
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

Display local devices:
```
$ mataho list

00000001: Front gate (io:SlidingDiscreteGateOpenerIOComponent)
00000002: Garage (io:GarageOpenerIOComponent)
00000003: Coko's room (io:RollerShutterWithLowSpeedManagementIOComponent)
```

Execute a command/action on a particular device (fuzzy matching is used to find the device):
```
$ mataho exec coko open

Executing `open` on `Coko's room`...
```

## 📚 Resources

- GitHub repo of [Somfy-TaHoma-Developer-Mode](https://github.com/Somfy-Developer/Somfy-TaHoma-Developer-Mode)
- Swagger UI for [local gateway API](https://somfy-developer.github.io/Somfy-TaHoma-Developer-Mode/)
