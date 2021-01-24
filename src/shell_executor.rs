use crate::common;
use std::fs;
use std::io;
use std::process::{Command, ExitStatus};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
extern crate system_shutdown;
use std::thread::sleep;

use system_shutdown::reboot;
use users::{get_current_uid, get_user_by_uid};

pub struct CommandExecutor {
    webserver_sender: mpsc::Sender<common::WebserverResponse>,
    matrix_sender: mpsc::Sender<common::MatrixCommand>,
    receiver: mpsc::Receiver<common::ShellCommand>,
}

impl CommandExecutor {
    pub fn new(
        webserver_sender: mpsc::Sender<common::WebserverResponse>,
        matrix_sender: mpsc::Sender<common::MatrixCommand>,
        receiver: mpsc::Receiver<common::ShellCommand>,
    ) -> CommandExecutor {
        CommandExecutor {
            webserver_sender,
            matrix_sender,
            receiver,
        }
    }
    fn send_webserver_response(self: &Self, response: common::WebserverResponse) {
        self.webserver_sender.send(response).unwrap();
    }
    fn send_matrix_response(self: &Self, response: common::MatrixCommand) {
        self.matrix_sender.send(response).unwrap();
    }

    // DHCPCD configures the `wlan0` or `wlan1` interfaces
    //      See /etc/dhcpcd.conf
    // However, DHCPCD requires a "carrier" to actually use the interface for anything
    // WPA_SUPPLICANT is for the usual wifi where you are a client connecting to another access point to get internet access
    // HOSTAPD is for when you are hosting the wifi access point
    fn carrier_control(self: &Self, enable: bool, service: &str) -> ExitStatus {
        self.execute(
            "sudo",
            &[
                "systemctl",
                if enable { "restart" } else { "stop" },
                service,
            ],
        )
        .expect("Failed to run systemctl on service")
    }

    fn set_wifi(self: &Self, enable: bool) -> ExitStatus {
        self.carrier_control(enable, "wpa_supplicant.service")
    }

    fn set_access_point(self: &Self, enable: bool) -> ExitStatus {
        self.carrier_control(enable, "hostapd.service")
    }

    fn execute(self: &Self, command: &str, args: &[&str]) -> io::Result<ExitStatus> {
        let result = Command::new(command).args(args).output()?;
        if result.stdout.len() > 0 {
            info!("{:?}", &String::from_utf8(result.stdout).unwrap());
        }
        if result.stderr.len() > 0 {
            error!("{:?}", &String::from_utf8(result.stderr).unwrap());
        }
        Ok(result.status)
    }

    fn setup_wifi(self: &Self, ssid: &str, password: &str) -> io::Result<ExitStatus> {
        // First, write to the WPA Supplicant file
        let supplicant = format!(
            "country=US
ctrl_interface=DIR=/var/run/wpa_supplicant GROUP=netdev
update_config=1

network={{
  ssid=\"{}\"
  scan_ssid=1
  psk=\"{}\"
  key_mgmt=WPA-PSK
}}",
            ssid, password
        );
        info!("Attempting to connect with supplicant:\n{}\n", supplicant);
        fs::write("/etc/wpa_supplicant/wpa_supplicant.conf", supplicant)?;

        let output = self.set_wifi(true);

        info!("Sleeping to allow wpa_supplicant to catch up after being restarted");
        sleep(Duration::from_secs(10));
        info!("Done waiting for wpa_supplicant to catch up");
        Ok(output)
    }

    pub fn run(self: &Self) {
        let user = get_user_by_uid(get_current_uid()).unwrap();
        info!("Shell exec: Hello, {}!", user.name().to_string_lossy());

        let output = Command::new("whoami").output().unwrap();
        info!("{:?}", String::from_utf8(output.stdout).unwrap());
        loop {
            let command = self.receiver.recv().unwrap();
            match command {
                common::ShellCommand::Reboot { settings } => {
                    if let Some(settings) = settings {
                        self.send_webserver_response(common::WebserverResponse::RebootResponse(
                            Some(settings),
                        ));
                    }
                    // Sleep for a second to let the response happen
                    thread::sleep(Duration::from_secs(3));
                    match reboot() {
                        Ok(_) => info!("Shutting down, bye!"),
                        Err(error) => error!("Failed to shut down: {}", error),
                    }
                }
                common::ShellCommand::Reset {
                    from_matrix,
                    from_webserver,
                } => {
                    // Enable wifi hotspot
                    let status = self.set_access_point(true);
                    if status.success() {
                        info!("Successfully enabled hotspot");
                    } else {
                        error!("Failed to enable hotspot, error code {:?}", status.code())
                    }

                    if let Some(settings) = from_webserver {
                        self.send_webserver_response(common::WebserverResponse::ResetResponse(
                            Some(settings),
                        ));
                    }
                    thread::sleep(Duration::from_secs(1));
                    info!("Resetting wifi");
                    // Just disable wlan0
                    let status = self.set_wifi(false);
                    if status.success() {
                        info!("Successfully disabled primary nic");
                    } else {
                        error!(
                            "Failed to disable primary nic, error code {:?}",
                            status.code()
                        )
                    }
                    if from_matrix {
                        self.matrix_sender
                            .send(common::MatrixCommand::FinishedReset(Ok(())))
                            .unwrap();
                    }
                }
                common::ShellCommand::SetHotspot(on) => {
                    let status = self.set_access_point(on);
                    if status.success() {
                        info!("Successfully set hotspot {}", on);
                    } else {
                        error!("Failed to set hotspot, error code {:?}", status.code())
                    }
                }
                common::ShellCommand::SetupWifi {
                    ssid,
                    password,
                    settings: _,
                } => {
                    // Setup the wifi
                    let mut success = true;

                    match self.setup_wifi(&ssid, &password) {
                        Ok(status) => {
                            if status.success() || status.code().unwrap_or(2) == 1 {
                                info!("Successfully setup wifi!");
                            } else {
                                success = false;
                                error!("Failed to setup wifi, error code {:?}", status.code())
                            }
                        }
                        Err(e) => {
                            success = false;
                            error!("Setup wifi failed with {:?}", e);
                        }
                    }

                    if success {
                        // If we've successfully connected, disable the hotspot
                        let hotspot_result = self.set_access_point(false);
                        if hotspot_result.success() {
                            info!("Successfully disabled hotspot");
                        } else {
                            error!(
                                "Failed to disable hotspot, error code {:?}",
                                hotspot_result.code()
                            );
                            success = false
                        }
                        if !common::is_connected() || common::get_ip_address().is_none() {
                            success = false;
                        }
                    }

                    if !success {
                        self.set_access_point(true);
                    }

                    self.send_matrix_response(common::MatrixCommand::FinishedWifiConnection(
                        if success {
                            Ok(())
                        } else {
                            Err(io::Error::new(io::ErrorKind::Other, "BLAH"))
                        },
                    ))
                }
            }
        }
    }
}
