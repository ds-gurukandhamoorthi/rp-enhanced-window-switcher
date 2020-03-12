use std::process::Command;
use std::process;

use serde::Deserialize;

use clap::{Arg, App, SubCommand};

//order is important as w use serde and csv format with no headers
#[derive(Debug, Deserialize)]
struct WinInfo {
    classname: String,
    number: u32,
    last_access: u32,
    status: char,
    screen: u32,
    application_name: String,
    title: String, //known as window_name in ratpoison
}

fn main() {
    let app = App::new("Enhanced Window Switcher")
        .about("Switch windows in Ratpoison wm")
        .subcommand(SubCommand::with_name("run-or-raise")
            .about("Raises a window or runs a given command")
            .arg(Arg::with_name("search_for_class")
                .takes_value(true)
                .help("classname of the window to switch to: (r)aise"))
            .arg(Arg::with_name("program_to_execute")
                .takes_value(true)
                .help("program to execute: (r)un"))
            .arg(Arg::with_name("extra_args")
                .takes_value(true)
                .required(false)
                .min_values(1)
                .help("optional extra arguments for the program"))
        );

    let matches = app.get_matches();

    if let ("run-or-raise", Some(ror_matches)) = matches.subcommand() {

        let search_for_class = ror_matches.value_of("search_for_class").unwrap();
        let program_to_execute = ror_matches.value_of("program_to_execute").unwrap();
        let extra_args: Vec<_> = match ror_matches.values_of("extra_args") {
            Some(v) => v.collect(),
            _ => vec![]
        };


        let output = Command::new("ratpoison").arg("-c").arg("windows %c\t%n\t%l\t%s\t%S\t%a\t%t").output();
        let output = output.unwrap();
        let output = String::from_utf8_lossy(output.stdout.as_slice());

        if output.clone().starts_with("No"){ //Quickfix to solve the opening program when there is no ratpoison windows
            Command::new(&program_to_execute).spawn().expect("Failed to open program");
            process::exit(0);
        }

        let mut windows_with_same_class = Vec::new();

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .from_reader(output.as_bytes());

        for record in rdr.deserialize() {
            let winfo: WinInfo = record.unwrap();
            if winfo.classname.starts_with(&search_for_class) {
                windows_with_same_class.push(winfo);
            }
        }

        let window_searched_for = windows_with_same_class
            .iter()
            .max_by_key(|w| w.last_access);


        match window_searched_for {
            Some(window) => {
                let window_number = window.number;
                let rp_command = format!("select {}", window_number);
                Command::new("ratpoison").arg("-c").arg(rp_command).output().expect("Failed to switch windows in Ratpoison");
            },
            None => {Command::new(program_to_execute).args(&extra_args).spawn().expect("Failed to open program");},
        }
    }
}
