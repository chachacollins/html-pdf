use std::env::args;
use std::fs;
use std::process::Command;
use std::str;
fn main() {
    let args: Vec<String> = args().collect();
    let output = Command::new("curl")
        .arg(args[1].clone())
        .output()
        .map_err(|err| {
            eprintln!("failed to execute process: {}", err);
            std::process::exit(1);
        });

    let binding = output.unwrap();
    let pdf = str::from_utf8(&binding.stdout).unwrap();
    let file = args[2].clone();
    let create_file = fs::write(file, pdf).map_err(|err| {
        eprintln!("failed to execute process: {}", err);
        std::process::exit(1);
    });
}
