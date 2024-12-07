use std::process::Command;

use log::info;

pub(crate) fn main() {
    info!("Running npm install on frontend");
    Command::new("npm")
        .current_dir("frontend")
        .arg("install")
        .status().expect("Running npm update");

        info!("Running npm run build on frontend");
        Command::new("npm")
        .current_dir("frontend")
        .args(["run", "build"])
        .status().expect("Running npm run build");
}
