use std::process::Command as OsCommand;
use thiserror::Error;

type OxideResult<T> = Result<T, OxideError>;

#[derive(Debug, Error)]
enum OxideError {
    #[error("Invalid or insufficient arguments. Usage: oxide [run|build] <PACKAGE> | [deploy] | [setup]")]
    InvalidArgs,

    #[error("The '{0:?}' action requires a package name.")]
    MissingPackageName(Action),

    #[error("{0}")]
    ExecutionFailure(String),

    #[error("{0}")]
    CommandFailure(String),
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Run,
    Build,
    Deploy,
    Setup,
}

struct Command {
    action: Action,
    args: Vec<String>,
}

impl Command {
    fn from_args(args: &[String]) -> OxideResult<Command> {
        if args.len() < 2 {
            return Err(OxideError::InvalidArgs);
        }

        let action = match args[1].as_str() {
            "run" => Action::Run,
            "build" => Action::Build,
            "deploy" => Action::Deploy,
            "setup" => Action::Setup,
            _ => return Err(OxideError::InvalidArgs),
        };

        Ok(Command {
            action,
            args: args[2..].to_vec(),
        })
    }

    fn execute(&self) -> Result<(), OxideError> {
        match &self.action {
            Action::Run | Action::Build => {
                if let Some(package) = self.args.get(0) {
                    run_or_build(&self.action, package)
                } else {
                    Err(OxideError::MissingPackageName(self.action))
                }
            }
            Action::Deploy => fly_deploy(),
            Action::Setup => setup(),
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Err(error) = Command::from_args(&args).and_then(|command| command.execute()) {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}

fn run_command(
    command: &str,
    args: &[&str],
    execution_msg: &str,
    failure_msg: &str,
) -> OxideResult<()> {
    let status = OsCommand::new(command)
        .args(args)
        .status()
        .map_err(|_| OxideError::ExecutionFailure(execution_msg.to_string()))?;

    if status.success() {
        Ok(())
    } else {
        Err(OxideError::CommandFailure(failure_msg.to_string()))
    }
}

fn run_or_build(action: &Action, package: &str) -> OxideResult<()> {
    let action_str = match action {
        Action::Run => "run",
        Action::Build => "build",
        _ => panic!("Invalid action passed to 'run_or_build()'"),
    };

    run_command(
        "cargo",
        &[action_str, "-p", package],
        &format!("Failed to execute cargo {} command", action_str),
        &format!("Cargo {} for {} failed", action_str, package),
    )
}

fn fly_deploy() -> OxideResult<()> {
    run_command(
        "flyctl",
        &["deploy"],
        "Failed to execute fly deploy command",
        "Fly deploy failed",
    )
}

fn setup() -> OxideResult<()> {
    run_command(
        "./setup.sh",
        &[],
        "Failed to execute setup. Ensure that you are at the root of the project",
        "Setup failed",
    )
}
