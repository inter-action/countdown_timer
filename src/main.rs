#[macro_use]
extern crate clap;
extern crate notify_rust;


use clap::{App, AppSettings, Arg};
use std::time::Duration;
use std::{thread, process};
use notify_rust::Notification;

#[cfg(target_os = "macos")]
static SOUND: &'static str = "Ping";

fn main() {
    let matches = App::new("ctimer")
        .version(crate_version!())
        .usage("ctimer [mins]")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .arg(Arg::with_name("mins").help("mins to countdown"))
        .get_matches();

    if let Some(mins) = matches.value_of("mins") {
        let cmins = Duration::from_secs(mins.parse::<u64>().unwrap() * 60);
        let child = thread::spawn(move || {
            thread::sleep(cmins);
        });
        println!("count down for {} mins", mins);
        let res = child.join();
        match res {
            Ok(_) => {
                notify();
            }
            _ => {
                println!("failed to join thread");
                process::exit(-1);
            }
        }
    } else {
        println!("Hello, world!");
    }
}

fn notify() {
    Notification::new()
        .summary("timer done")
        .sound_name(SOUND)
        .timeout(5 * 1000)
        .show()
        .unwrap();
}