use std::env;
use std::process::Command;

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
    pub fn new(info: &str) -> WinInfo {
        let mut fields = info.split(',');
        let classname = fields.next().unwrap().to_string();
        let number: u32 = fields.next().unwrap().parse().expect("Failed to parse window number");
        let last_access: u32 = fields.next().unwrap().parse().expect("Failed to parse last access");
        let status: char = fields.next().unwrap().parse().unwrap();
        let screen: u32 = fields.next().unwrap().parse().expect("Failed to parse screen number");
        let application_name = fields.next().unwrap().to_string();
        let title = fields.next().unwrap().to_string();
        WinInfo{
            classname,
            application_name,
            number,
            last_access,
            status,
            screen,
            title,
        }
    }
}

fn main() {
    let mut args = env::args();
    args.next();

    let search_for_class = args.next().unwrap();
    let program_to_execute = args.next().unwrap();


    let output = Command::new("ratpoison").arg("-c").arg("windows %c,%n,%l,%s,%S,%a,%t").output().expect("Failed to retrieve list of windows from Ratpoison");
    let output = String::from_utf8_lossy(output.stdout.as_slice());
    let windows_with_same_class = output.lines()
        .map(WinInfo::new)
        .filter(|w| w.classname.starts_with(&search_for_class));

    let window_searched_for = windows_with_same_class
        .max_by_key(|w| w.last_access);

    match window_searched_for {
        Some(window) => {
            let window_number = window.number;
            let rp_command = format!("select {}", window_number);
            Command::new("ratpoison").arg("-c").arg(rp_command).output().expect("Failed to switch windows in Ratpoison");
        },
        None => {Command::new(program_to_execute).spawn().expect("Failed to open program");},
    }
}
