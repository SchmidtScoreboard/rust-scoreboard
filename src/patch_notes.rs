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
