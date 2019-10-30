use std::process;
use std::process::Command;

fn main() {
    let output = Command::new("ratpoison").arg("-c").arg("windows %c,%n,%l,%s,%a,%t").output().ok().expect("Failed to retrieve list of windows from Ratpoison");
    let output = String::from_utf8_lossy(output.stdout.as_slice());
    for line in output.lines() {
        let mut vec = line.split(',');
        let classname = vec.next();
        match classname {
            Some(classname) if classname == "Firefox" => {
                println!("{}", classname);
                let window_number = vec.next().unwrap();
                let rp_command = format!("select {}", window_number);
                Command::new("ratpoison").arg("-c").arg(rp_command).output().ok().expect("Failed to switch windows in Ratpoison");
                process::exit(0);
            }
            Some(_) => (),
            None => (),
        }
    }
}
