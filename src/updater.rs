use self_update::cargo_crate_version;
use std::{env, process::Command};

pub fn update() {
    match download_release(){
        Ok(_) => { println!("Update successful!"); },
        Err(e) => { println!("Update failed: {}", e); }
    }
}

pub fn download_release() -> Result<(), Box<dyn ::std::error::Error>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("jamiroferrara")
        .repo_name("mayday.remoterack.listener")
        .bin_name("github")
        .show_download_progress(true)
        .no_confirm(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;

    if status.updated() {
        println!("Updated to {}", status.version());
        let path = env::current_exe()?.display().to_string();
        Command::new(path).spawn()?;
    } else {
        println!("Already up to date: {}", status.version());
    }

    Ok(())
}
