use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        println!("Usage: oxide [run|build] <PACKAGE>");
        return;
    }

    let command = &args[1];
    let package = &args[2];

    match command.as_str() {
        "run" => run_package(package),
        "build" => build_package(package),
        _ => {
            println!("Invalid command. Usage: oxide [run|build] <PACKAGE>");
        }
    }
}

fn run_package(package: &str) {
    let mut cmd = Command::new("cargo");
    cmd.args(["run", "-p", package]);
    cmd.spawn().expect("Failed to run the package");
}

fn build_package(package: &str) {
    let mut cmd = Command::new("cargo");
    cmd.args(["build", "-p", package]);
    cmd.spawn().expect("Failed to build the package");
}

fn fly_deploy() {
    todo!("Implement fly deploy in CLI tool")
}

