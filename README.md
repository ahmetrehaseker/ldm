# LightWeight Device Monitor
Main purpose of this tool is monitoring devices that has limited resources.

### Current Metrics:
* Cpu Usage
* Disk Usage
* Disk Write IO (In Progress)
* Disk Read IO (In Progress)
* Memory Usage
* Network Rx (kb/s)
* Network Tx (kb/s)
* Network Rx Total
* Network Tx Total
* Cpu Temperature
* Process Cpu Metrics (In Progress)
* Process Memory Metrics (In Progress)
* Process Network Metrics (In Progress)
 
### Notification Methods:
* Opsgenie Alerts
* Slack Messages (In Progress)

### Visualisation:
* In Progress

### Example Configurations can be found in [here](https://github.com/ahmetrehaseker/ldm/blob/master/config/config.toml)

### Setup Scripts and Released Artifacts will be uploaded soon
### Installation(Arm):
* From source code: 

```
rustup target add armv7-unknown-linux-gnueabihf
cargo --release --target=armv7-unknown-linux-gnueabihf
```

create a folder with name `ldm` in configuration folder (~/.config/)

Put `config.toml` and `log4rs.yaml` in to this folder.

Copy `ldm.service`(from installation folder) file to `/etc/systemd/system/`

Copy release artifact `target/release/ldm_service` to `/usr/local/bin/`

run command `sudo systemctl start ldm`