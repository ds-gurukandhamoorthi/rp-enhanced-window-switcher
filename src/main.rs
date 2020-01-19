use std::env;
use std::process::Command;
use std::process;

#[derive(Debug)]
struct WinInfo {
    classname: String,
    application_name: String,
    number: u32,
    last_access: u32,
    status: char,
    screen: u32,
    title: String, //known as window_name in ratpoison
}

impl WinInfo {
    pub fn new(info: &str) -> Result<WinInfo, &'static str> {
        let mut fields = info.split(',');
        let classname = match fields.next() {
            Some(s) => s.to_string(),
            None => return Err("Unable to extract class name"),
        };
        let number: u32 = match fields.next().unwrap().parse() {
            Ok(ui) => ui,
            Err(_) => return Err("Unable to parse window number as integer"),
        };
        let last_access: u32 = match fields.next().unwrap().parse() {
            Ok(ui) => ui,
            Err(_) => return Err("Unable to last access as integer"),
        };
        let status: char = match fields.next().unwrap().parse() {
            Ok(c) => c,
            Err(_) => return Err("Unable to status as character"),
        };
        let screen: u32 = match fields.next().unwrap().parse() {
            Ok(ui) => ui,
            Err(_) => return Err("Unable to screen as integer"),
        };
        let application_name = match fields.next() {
            Some(s) => s.to_string(),
            None => return Err("Unable to extract application name"),
        };
        let title = match fields.next() {
            Some(s) => s.to_string(),
            None => return Err("Unable to extract application name"),
        };
        Ok(
        WinInfo{
            classname,
            application_name,
            number,
            last_access,
            status,
            screen,
            title,
        })
    }
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

    let windows_with_same_class = output.lines()
        .map(WinInfo::new)
        .filter_map(Result::ok)
        .filter(|w| w.classname.starts_with(&search_for_class));


    let window_searched_for = windows_with_same_class
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
