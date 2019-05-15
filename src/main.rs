#[macro_use]
extern crate clap;
extern crate notify_rust;

use clap::{App, AppSettings, Arg};
use notify_rust::Notification;
use std::thread;
use std::time::Duration;

#[cfg(target_os = "macos")]
static SOUND: &'static str = "Ping";

fn main() {
    let matches = App::new("ctimer")
        .version(crate_version!())
        .usage("ctimer [mins]")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .arg(
            Arg::with_name("mins")
                .help("mins to countdown")
                .required(true),
        )
        .get_matches();

    if let Some(mins) = matches.value_of("mins") {
        let cmins = Duration::from_secs(mins.parse::<u64>().unwrap() * 60);
        let child = thread::spawn(move || {
            thread::sleep(cmins);
        });
        println!("count down for {} mins", mins);
        child.join().unwrap();
        notify().unwrap();
    }
}

fn notify() -> Result<notify_rust::NotificationHandle, notify_rust::Error> {
    Notification::new()
        .summary("timer done")
        .sound_name(SOUND)
        .timeout(5 * 1000)
        .show()
}
