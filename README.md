# tahoma-cli

A rust port of the cli from my [somfy-home-controller](https://github.com/coko7/somfy-home-controller) project.

## 🛠️ Setup

1. Create a `tahoma-cli` directory in `$XDG_CONFIG_HOME`
2. Create `config.json` in this new directory and put the following inside:
```json
{
    "hostname": "https://gateway-XXXX-XXXX-XXXX"
    "port": 8443,
    "api_token": "PUT_YOUR_SUPER_SECRET_TOKEN_HERE",
}
```

*NOTE: If you want to use a different config path, you can do so by setting the `$TAHOMA_CLI_CONFIG` env variable.*

## 🐚 Usage

![image](https://github.com/user-attachments/assets/700a0ed5-9225-4114-8bc7-2f90e0d2e384)

## 📚 Resources

- GitHub repo of [Somfy-TaHoma-Developer-Mode](https://github.com/Somfy-Developer/Somfy-TaHoma-Developer-Mode)
- Swagger UI for [local gateway API](https://somfy-developer.github.io/Somfy-TaHoma-Developer-Mode/)
