use std::env;
use std::process::Command;
use std::process;

use serde::Deserialize;

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
    let mut args = env::args();
    args.next();

    let search_for_class = args.next().unwrap();
    let program_to_execute = args.next().unwrap();
    let extra_args: Vec<String> = args.collect();


    let output = Command::new("ratpoison").arg("-c").arg("windows %c,%n,%l,%s,%S,%a,%t").output();
    let output = output.unwrap();
    let output = String::from_utf8_lossy(output.stdout.as_slice());

    if output.clone().starts_with("No"){ //Quickfix to solve the opening program when there is no ratpoison windows
        Command::new(&program_to_execute).spawn().expect("Failed to open program");
        process::exit(0);
    }

    let mut windows_with_same_class = Vec::new();

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
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
