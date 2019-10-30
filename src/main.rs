use std::env;
use std::process::Command;

fn main() {
    let mut args = env::args();
    args.next();

    let search_for_class = args.next().unwrap();
    let program_to_execute = args.next().unwrap();


    let output = Command::new("ratpoison").arg("-c").arg("windows %c,%n,%l,%s,%a,%t").output().ok().expect("Failed to retrieve list of windows from Ratpoison");
    let output = String::from_utf8_lossy(output.stdout.as_slice());
    let window_searched_for = output.lines()
        .filter(|l| l.starts_with(&search_for_class))
        .nth(0);

    match window_searched_for {
        Some(window) => {
            let mut fields = window.split(',');
            fields.next();
            let window_number = fields.next().unwrap();
            let rp_command = format!("select {}", window_number);
            Command::new("ratpoison").arg("-c").arg(rp_command).output().ok().expect("Failed to switch windows in Ratpoison");
        },
        None => {Command::new(program_to_execute).spawn().expect("Failed to open program");},
    }
}
