use crate::common;
use std::fs;
use std::io;
use std::process::{Command, ExitStatus};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

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

    fn set_hotspot(self: &Self, enable: bool) -> io::Result<ExitStatus> {
        Command::new("ip")
            .arg("link")
            .arg("set")
            .arg("wlan1")
            .arg(if enable { "up" } else { "down" })
            .status()
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

        let daemon_reload = Command::new("systemctl").arg("daemon-reload").status()?;
        if !daemon_reload.success() {
            error!("Failed to systemctl daemon reload");
            return Ok(daemon_reload);
        }

        let dhclient1 = Command::new("dhclient").args(&["-r", "wlan0"]).status()?;
        if !dhclient1.success() {
            error!("Failed to stop dhclient");
            return Ok(dhclient1);
        }

        let ifdown = Command::new("ifdown").arg("wlan0").status()?;
        if !ifdown.success() {
            error!("Failed to bring interface down");
            return Ok(ifdown);
        }

        let ifup = Command::new("ifup").arg("wlan0").status()?;
        if !ifup.success() {
            error!("Failed to bring interface up");
            return Ok(ifup);
        }

        Command::new("dhclient").args(&["-v", "wlan0"]).status()
    }

    pub fn run(self: &Self) {
        loop {
            let command = self.receiver.recv().unwrap();
            match command {
                common::ShellCommand::Reboot { settings } => {
                    let mut reboot_command = Command::new("reboot");
                    info!("Running command {:?}", reboot_command);
                    self.send_webserver_response(common::WebserverResponse::RebootResponse(Some(
                        settings,
                    )));
                    // Sleep for a second to let the response happen
                    thread::sleep(Duration::from_secs(3));
                    let result = reboot_command.status();
                    match result {
                        Ok(_) => {
                            warn!("Printing after we're rebooting... this shouldn't happen");
                        }
                        Err(e) => {
                            error!("Errror rebooting {:?}", e);
                        }
                    }
                }
                common::ShellCommand::Reset { from_webserver } => {
                    // Enable wifi hotspot
                    let hotspot_result = self.set_hotspot(true);
                    match hotspot_result {
                        Ok(status) => {
                            if status.success() {
                                info!("Successfully reenabled hotspot");
                            } else {
                                error!("Failed to reenable hotspot, error code {:?}", status.code())
                            }
                        }
                        Err(e) => {
                            error!("Error when enabling hotspot {:?}", e);
                        }
                    }

                    if let Some(settings) = from_webserver {
                        self.send_webserver_response(common::WebserverResponse::ResetResponse(
                            Some(settings),
                        ));
                    }

                    self.send_matrix_response(common::MatrixCommand::FinishedReset(Ok(())));
                }
                common::ShellCommand::SetupWifi {
                    ssid,
                    password,
                    settings,
                } => {
                    // Setup the wifi
                    let mut success = true;

                    match self.setup_wifi(&ssid, &password) {
                        Ok(status) => {
                            if status.success() {
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
                        let hotspot_result = self.set_hotspot(false);
                        match hotspot_result {
                            Ok(status) => {
                                if status.success() {
                                    info!("Successfully reenabled hotspot");
                                } else {
                                    error!(
                                        "Failed to reenable hotspot, error code {:?}",
                                        status.code()
                                    );
                                    success = false
                                }
                            }
                            Err(e) => {
                                error!("Error when enabling hotspot {:?}", e);
                                success = false;
                            }
                        }
                    }

                    self.send_webserver_response(
                        common::WebserverResponse::GotWifiDetailsResponse(if success {
                            Some(settings)
                        } else {
                            None
                        }),
                    );

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
