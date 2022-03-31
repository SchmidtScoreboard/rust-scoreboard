use std::thread;
use std::time::Duration;

pub struct Updater {
    _update_available: bool,
}

impl Updater {
    pub fn new() -> Updater {
        Updater {
            _update_available: false,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.update().unwrap_or(());
            thread::sleep(Duration::from_secs(60 * 60)); // Every hour
        }
    }

    pub fn _check_now(&mut self) {
        self.update().unwrap_or(());
    }

    fn update(&mut self) -> Result<(), Box<dyn ::std::error::Error>> {
        info!("Starting update check");
        let start_version = self_update::cargo_crate_version!();
        info!("Updating, version is '{}'", start_version);
        let status = self_update::backends::github::Update::configure()
            .repo_owner("SchmidtScoreboard")
            .repo_name("rust-scoreboard")
            .bin_name("scoreboard")
            .no_confirm(true)
            .current_version(self_update::cargo_crate_version!())
            .build()?
            .update()?;
        info!(
            "Update status: `{}`, old version '{}'!",
            status.version(),
            start_version
        );
        if start_version != status.version() {
            self._update_available = true;
        }
        Ok(())
    }
}
