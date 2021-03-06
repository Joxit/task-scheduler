use crate::commands::completion::Completion;
use crate::commands::dot::Dot;
use crate::commands::exec::Exec;
use crate::commands::run::Run;
use structopt::StructOpt;

mod completion;
mod dot;
mod exec;
mod run;

#[derive(Debug, StructOpt)]
pub enum Command {
  /// Run tasks.
  #[structopt(name = "run")]
  Run(Run),
  /// Export the configuration to a graph (needs graphviz/dot).
  #[structopt(name = "dot")]
  Dot(Dot),
  /// Execute a single command with notification.
  #[structopt(name = "exec")]
  Exec(Exec),
  /// Generate completion script for your shell.
  #[structopt(name = "completion")]
  Completion(Completion),
}

impl Command {
  pub fn exec(&self) {
    match self {
      Command::Run(executable) => executable.exec(),
      Command::Exec(executable) => executable.exec(),
      Command::Dot(executable) => executable.exec(),
      Command::Completion(executable) => executable.exec(),
    }
  }
}
