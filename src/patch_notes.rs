pub struct Patch {
    version: &'static str,
    notes: &'static str,
}

impl Patch {
    fn create(version: &'static str, notes: &'static str) -> Patch {
        Patch { version, notes }
    }

    fn log(self: &Self) {
        info!("Version {}:\n\n{}", self.version, self.notes);
    }
}

pub fn get_patches() -> Vec<Patch> {
    vec![
        Patch::create("2.3.2", "Better Error Handling, fixes to football"),
        Patch::create("2.3.1", "Not hideous default custom message"),
        Patch::create("2.3.0", "Show football stats, custom message screen"),
        Patch::create("2.2.0", "Show on base during baseball mode"),
        Patch::create("2.1.1", "Clock auto power"),
        Patch::create("2.1.0", "Golf + Flappy Bird!"),
        Patch::create("2.0.2", "Nightly reboot + update"),
        Patch::create("2.0.1", "Fix to smart power mode"),
        Patch::create("2.0.0", "Smart switch mode. Also variable width fonts for more readable text. Also, football modes"),
        Patch::create("1.6.0", "Basketball!"),
        Patch::create("1.5.0", "College basketball!"),
        Patch::create("1.3.0", "Allow brightness control"),
        Patch::create("1.2.6", "Fix baseball screen"),
        Patch::create("1.2.6", "Fix games overflow"),
        Patch::create("1.2.5", "Fix layout issues"),
        Patch::create("1.2.4", "Fix reboot button"),
        Patch::create("1.2.3", "Fix powerplay display"),
        Patch::create("1.2.2", "Update version number"),
        Patch::create("1.2.1", "Fixes rotation time bug"),
        Patch::create("1.2", "Adds global favorite teams"),
        Patch::create("1.1.5", "Fixes delayed commands ordered incorrectly"),
        Patch::create("1.1.4", "Adds update service"),
        Patch::create("1.1.3", "Test patch, please ignore"),
        Patch::create("1.1.2", "Adds patch notes"),
        Patch::create(
            "1.1.1",
            "Fixes character encoding in JSON responses from webserver",
        ),
        Patch::create(
            "1.1.0",
            "Adds support for Pi4 host devices and adds slowdown flag",
        ),
    ]
}


pub fn log_patch_notes() {
    info!("Patch Notes:");
    get_patches().into_iter().for_each(|p| p.log());
}
