# Configuring a New Scoreboard

Setting up a new scoreboard from a fresh RaspberryPi OS install is not super simple. Here's some of the things you'll need to do:

# Configuring Scoreboard Service

Create an entry at `/etc/systemd/system/scoreboard.service`

```
[Unit]
Description=Scoreboard application
Requires=network.target rc-local.service getty.target systemd-user-sessions.service dnsmasq.service
After=network.target rc-local.service getty.target systemd-user-sessions.service dnsmasq.service

[Service]
EnvironmentFile=/var/lib/scoreboard/environment.conf
ExecStart=/usr/local/bin/scoreboard -w

[Install]
WantedBy=multi-user.target
```

Create a working directory at `/var/lib/scoreboard`. This needs a few files:

1. The `scoreboard` binary, built from this repository
2. A `logs` directory
3. An environment.conf file. Default looks like ```RUST_LOG="debug"
RUST_BACKTRACE=full```
4. A `secrets.txt` file with your Scoreboard API Key
5. A default `scoreboard_settings.json` file 
6. A default `custom_message.json` file

You'll also need to create a symlink from `/usr/local/bin/scoreboard` to `/var/lib/scoreboard/scoreboard` so that the systemd service can access the scoreboard binary

# Setting up WiFi / AP Hosting

During the setup process, Scoreboard broadcasts its own wifi network to allow the configuration application to connect and send the user's home Wi-Fi information. Typically, Scoreboard uses a TP Link USB WiFi dongle for better Wi-Fi performance (the Raspberry Pi's built in WiFi has some connectivity issues)

To configure this, we set the USB dongle to WLAN0 as the primary NIC and the RPIs to WLAN1 as the secondary NIC. Add this to `/etc/network/interfaces`:

```
# interfaces(5) file used by ifup(8) and ifdown(8)

# Please note that this file is written to be used with dhcpcd
# For static IP, consult /etc/dhcpcd.conf and 'man dhcpcd.conf'

# Include files from /etc/network/interfaces.d:
source-directory /etc/network/interfaces.d

auto lo
iface lo inet loopback

auto wlan1
iface wlan1 inet static
    address 42.42.42.1
    network 255.255.255.0

allow-hotplug wlan0
auto wlan0
iface wlan0 inet dhcp
pre-up wpa_supplicant -B w -D wext -i wlan0 -c /etc/wpa_supplicant/wpa_supplicant.conf
post-down killall -q wpa_supplicant
```

## Configuring AP stuff

Install necessary components:

```
sudo apt-get install dnsmasq
sudo apt-get install hostapd
sudo systemctl unmask hostapd
sudo systemctl enable hostapd
sudo rfkill unblock wifi
```

Configure dnsmasq by updating  `/etc/dnsmasq.conf`

```
interface=wlan1
dhcp-range=42.42.42.2, 42.42.42.20, 255.255.255.0, 24h
```

Configure hostapd by updating/creating `/etc/hostapd/hostapd.conf`:

```
interface=wlan1
driver=nl80211
ssid=SSB42
channel=1
```

You'll want to disable `dhcpcd.service` to prevent it from interfering:
`sudo systemctl mask dhcpcd.service`
