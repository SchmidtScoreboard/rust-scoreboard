use crate::common;
use std::fs;
use std::io;
use std::process::{Command, ExitStatus};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
extern crate system_shutdown;

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
    fn send_webserver_response(&self, response: common::WebserverResponse) {
        self.webserver_sender.send(response).unwrap();
    }
    fn send_matrix_response(&self, response: common::MatrixCommand) {
        self.matrix_sender.send(response).unwrap();
    }

    fn set_interface(&self, interface: &str, enable: bool) -> ExitStatus {
        self.execute(
            "sudo",
            &[
                "ip",
                "link",
                "set",
                interface,
                if enable { "up" } else { "down" },
            ],
        )
        .expect("Failed to run set interfaces")
    }

    fn execute(&self, command: &str, args: &[&str]) -> io::Result<ExitStatus> {
        let result = Command::new(command).args(args).output()?;
        if !result.stdout.is_empty() {
            info!("{:?}", &String::from_utf8(result.stdout).unwrap());
        }
        if !result.stderr.is_empty() {
            error!("{:?}", &String::from_utf8(result.stderr).unwrap());
        }
        Ok(result.status)
    }

    fn setup_wifi(&self, ssid: &str, password: &str) -> io::Result<ExitStatus> {
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

        let daemon_reload = self.execute("systemctl", &["daemon-reload"])?;
        if !daemon_reload.success() {
            error!("Failed to systemctl daemon reload");
            return Ok(daemon_reload);
        }

        self.execute("sudo", &["dhclient", "-r", "wlan0"])?;

        self.execute("sudo", &["ifdown", "wlan0"])?;

        self.execute("sudo", &["ifup", "wlan0"])?;

        let output = self.execute("sudo", &["dhclient", "-v", "wlan0"])?;
        Ok(output)
    }

    pub fn run(&self) {
        let user = get_user_by_uid(get_current_uid()).unwrap();
        info!("Shell exec: Hello, {}!", user.name().to_string_lossy());

        let output = Command::new("whoami").output().unwrap();
        info!("{:?}", String::from_utf8(output.stdout).unwrap());
        loop {
            let command = self.receiver.recv().unwrap();
            match command {
                common::ShellCommand::Reboot { settings } => {
                    if let Some(settings) = settings {
                        self.send_webserver_response(common::WebserverResponse::Reboot(
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
                    let status = self.set_interface("wlan1", true);
                    if status.success() {
                        info!("Successfully enabled hotspot");
                    } else {
                        error!("Failed to enable hotspot, error code {:?}", status.code())
                    }

                    if let Some(settings) = from_webserver {
                        self.send_webserver_response(common::WebserverResponse::Reset(
                            Some(settings),
                        ));
                    }
                    thread::sleep(Duration::from_secs(1));
                    info!("Resetting wifi");
                    // Just disable wlan0
                    let status = self.set_interface("wlan0", false);
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
                    let status = self.set_interface("wlan1", on);
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
                        let hotspot_result = self.set_interface("wlan1", false);
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
                        self.set_interface("wlan1", true);
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
