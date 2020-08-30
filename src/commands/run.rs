use crate::utils::traits::CommandConfig;
use crate::config::{Config, WhenNotify};
use crate::fst::*;
use libc::{fork, signal};
use libc::{SIGHUP, SIG_IGN};
use std::fs::{self, OpenOptions};
use std::path::PathBuf;
use std::process::{exit, Command, Stdio};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Run {
  /// Configuration path (YAML)
  #[structopt()]
  config: PathBuf,
  /// Run the task in background
  #[structopt(long = "background", short = "b")]
  background: bool,
}

impl Run {
  pub fn exec(&self) {
    if !self.config.exists() {
      eprintln!("The config file {} does not exists", self.config.display());
      return;
    }

    if self.background && unsafe { fork() } != 0 {
      // The main process should return
      return;
    } else if self.background {
      // Ignoring SIGHUP in background mode
      unsafe { signal(SIGHUP, SIG_IGN) };
    }

    if let Err(e) = self.run() {
      eprintln!("{}", e);
      exit(1);
    }
  }

  fn run(&self) -> Result<(), String> {
    let yaml = fs::read_to_string(self.config.as_path())
      .map_err(|msg| format!("Can't read the config file: {}", msg))?;

    let mut config = Config::from_str(yaml.as_str())
      .map_err(|msg| format!("Can't process the config file: {}", msg))?;

    let initial_states: Vec<String> = config
      .tasks()
      .values()
      .filter(|task| task.depends_on().len() == 0)
      .map(|task| task.id().to_owned())
      .collect();

    println!("Will start with {:?} as initial states", initial_states);

    let mut graph = TaskFst::new();
    for task in config.tasks_values_mut() {
      task.set_state(graph.add_state(task.id().to_owned()));
    }

    for task in config.tasks().values() {
      if task.depends_on().len() == 0 {
        graph.add_start_state(task.state());
      } else {
        for prev in task.depends_on().iter() {
          let err_msg = format!("{} depends on {} but does not exists", task.id(), prev);
          let prev_state = config.tasks().get(prev).ok_or(err_msg)?.state();
          graph.add_arc(prev_state, task.state());
        }
      }
    }

    if graph.is_cyclic() {
      let err_msg = "Can't execute your configuration. There is a deadlock in your tasks !";
      return Err(err_msg.to_string());
    }

    let processes: &mut Vec<Option<std::process::Child>> = &mut vec![];
    for _ in 0..graph.len() {
      processes.push(None);
    }

    let mut exit_success = 0;
    let mut exit_failure = 0;
    let graph_iter = &mut graph.iter();
    loop {
      if graph_iter.has_next()
        && (graph_iter.n_in_progress() < config.concurrency() || config.concurrency() < 0)
      {
        let task = graph_iter.next().unwrap();
        let label = task.label().to_string();
        let cmd_line = config.tasks().get(&label).unwrap().commands().join(" && ");
        let child = Command::new("sh")
          .arg("-c")
          .arg(cmd_line.to_string())
          .stdin(self.stdin())
          .stdout(self.stdout()?)
          .stderr(self.stderr()?)
          .working_dir(config.working_dir())
          .spawn()
          .map_err(|msg| format!("Can't run command `{}`: {}", cmd_line, msg))?;
        processes[task.id()] = Some(child);
      } else if graph_iter.is_done() {
        break;
      } else {
        let mut done = 0;
        for id in 0..processes.len() {
          if let Some(child) = processes[id].as_mut() {
            if let Ok(Some(exit)) = child.try_wait() {
              if exit.success() {
                exit_success = exit_success + 1;
              } else {
                exit_failure = exit_failure + 1;
              }

              done = done + 1;
              graph_iter.mark_done(id);
              processes[id] = None;

              let msg = format!(
                "Task {} ended with status code {}",
                graph.get_state_from_id(id).label(),
                exit
              );
              self.notify(&config, msg, WhenNotify::TaskEnd);
            }
          }
        }

        if done == 0 {
          std::thread::sleep(std::time::Duration::from_millis(100));
        }
      }
    }

    let msg = format!(
      "All tasks ended. Got {} success and {} failure.",
      exit_success, exit_failure
    );
    self.notify(&config, msg, WhenNotify::End);

    Ok(())
  }

  fn notify(&self, config: &Config, msg: String, when: WhenNotify) {
    if let Some(notification) = config.notification() {
      if *notification.when() == WhenNotify::Never
        || (*notification.when() != WhenNotify::Always && *notification.when() != when)
      {
        return;
      }
      if let Some(slack) = notification.slack() {
        if let Some(when_slack) = slack.when() {
          if *when_slack == WhenNotify::Never
            || (*when_slack != WhenNotify::Always && *when_slack != when)
          {
            return;
          }
        }
        if let Err(e) = crate::notification::post_slack(&slack, msg.as_str()) {
          eprintln!("Can't use slac notification: {}", e);
        }
      }
    }
  }

  fn stdout(&self) -> Result<Stdio, String> {
    if self.background {
      Ok(Stdio::from(
        OpenOptions::new()
          .create(true)
          .append(true)
          .open("task-scheduler.out")
          .map_err(|msg| format!("Can't open output file: {}", msg))?,
      ))
    } else {
      Ok(Stdio::inherit())
    }
  }

  fn stdin(&self) -> Stdio {
    if self.background {
      Stdio::null()
    } else {
      Stdio::inherit()
    }
  }

  fn stderr(&self) -> Result<Stdio, String> {
    if self.background {
      Ok(Stdio::from(
        OpenOptions::new()
          .create(true)
          .append(true)
          .open("task-scheduler.err")
          .map_err(|msg| format!("Can't open error file: {}", msg))?,
      ))
    } else {
      Ok(Stdio::inherit())
    }
  }
}
