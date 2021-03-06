use std::fs::OpenOptions;
use std::path::Path;
use std::process::{Command, Stdio};

pub trait CommandConfig {
  fn working_dir(&mut self, dir: &Option<String>) -> Result<&mut Self, String>;
  fn stdout_opt(&mut self, stdout: &Option<String>, inherit: bool) -> Result<&mut Self, String>;
  fn stderr_opt(&mut self, stdout: &Option<String>, inherit: bool) -> Result<&mut Self, String>;
}

impl CommandConfig for Command {
  fn working_dir(&mut self, dir: &Option<String>) -> Result<&mut Self, String> {
    if let Some(d) = dir {
      if Path::new(d).is_dir() {
        Ok(self.current_dir(d))
      } else {
        Err(format!(
          "Invalid working directory: `{}` is not a directory",
          d
        ))
      }
    } else {
      Ok(self)
    }
  }

  fn stdout_opt(&mut self, stdout: &Option<String>, inherit: bool) -> Result<&mut Self, String> {
    let stdio = process_stdio(stdout, inherit, "runtasktic.out")?;

    Ok(self.stdout(stdio))
  }

  fn stderr_opt(&mut self, stderr: &Option<String>, inherit: bool) -> Result<&mut Self, String> {
    let stdio = process_stdio(stderr, inherit, "runtasktic.err")?;

    Ok(self.stderr(stdio))
  }
}

fn process_stdio(
  file: &Option<String>,
  inherit: bool,
  default_file: &'static str,
) -> Result<Stdio, String> {
  let res = if let Some(stdio) = file {
    match stdio.as_str() {
      "none" | "/dev/null" => Stdio::null(),
      _ => Stdio::from(
        OpenOptions::new()
          .create(true)
          .append(true)
          .open(stdio)
          .map_err(|msg| format!("Can't open output file {}: {}", stdio, msg))?,
      ),
    }
  } else if !inherit {
    Stdio::from(
      OpenOptions::new()
        .create(true)
        .append(true)
        .open(default_file)
        .map_err(|msg| format!("Can't open output file: {}", msg))?,
    )
  } else {
    Stdio::inherit()
  };

  Ok(res)
}
