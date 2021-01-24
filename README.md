# Mark should add a readme here

TODO

## Raspberry Pi OS configuration

This setup focuses on getting the network configured properly.
Assuming the Scoreboard hardware is pluged in correctly, the scoreboard app
built from this repo should "just work".

I'll need to redo these steps again on a clean image to verify, but this is close enough.

- I used the January 11th 2021 release of the Raspberry Pi OS Lite
- Modify `/boot/config.txt` to contain `enable_uart=1` if you want to use the serial port console.
- enable ssh and rsync 
  - `sudo systemctl enable ssh`
  - `sudo systemctl start ssh`
  - `sudo systemctl enable rsync`
  - `sudo systemctl start rsync`
- make the scoreboard directory & copy over config files
  - `mkdir /var/lib/scoreboard`
  - `chown pi /var/lib/scorboard`
  - copy over `secrets.txt`, `scoreboard_settings.json`, and `environment.conf`.
    - `environment.conf` contains two lines `RUST_LOG="debug"` and `RUST_BACKTRACE=full`
- run the installer in this repo to copy over the scoreboard binary
- `ln -s /var/lib/scoreboard/scoreboard /usr/local/bin/scoreboard`
- Unblock built-in wlan `rfkill unblock wlan`
- Follow this guide: [https://www.raspberrypi.org/documentation/configuration/wireless/access-point-routed.md](https://www.raspberrypi.org/documentation/configuration/wireless/access-point-routed.md)
  - Skip the Enable routing and IP masquerading section
  - Substitute your own IP addresses for the Access point
  - Use wlan1 as access point
  - Use wlan0 as internet connection
- Modify the wpa systemctl service such that wpa_supplicant will actually get wlan0 to connect to the internet.
  - Change the exec line to specifiy the drivers, the interface, and the wpa_supplicant.conf file to use
  - `/etc/systemd/system/dbus-fi.w1.wpa_supplicant1.service`
  - `ExecStart=/sbin/wpa_supplicant -u -D wext -i wlan0 -c /etc/wpa_supplicant/wpa_supplicant.conf -s -O /run/wpa_supplicant`
