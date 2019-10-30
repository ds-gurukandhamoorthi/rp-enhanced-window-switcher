use std::process;
use std::process::Command;

fn main() {
    let output = Command::new("ratpoison").arg("-c").arg("windows %c,%n,%l,%s,%a,%t").output().ok().expect("Failed to retrieve list of windows from Ratpoison");
    let output = String::from_utf8_lossy(output.stdout.as_slice());
    for line in output.lines()
        .filter(|l| l.starts_with("Firefox")) {
            let mut fields = line.split(',');
            fields.next();
            let window_number = fields.next().unwrap();
            let rp_command = format!("select {}", window_number);
            Command::new("ratpoison").arg("-c").arg(rp_command).output().ok().expect("Failed to switch windows in Ratpoison");
            process::exit(0);
        }
}
