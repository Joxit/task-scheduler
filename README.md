# runtasktic

[![Rust](https://github.com/Joxit/runtasktic/workflows/Rust/badge.svg)](https://github.com/Joxit/runtasktic/actions?query=workflow%3ARust)
[![Crates.io version shield](https://img.shields.io/crates/v/runtasktic.svg)](https://crates.io/crates/runtasktic)
[![Crates.io license shield](https://img.shields.io/crates/l/runtasktic.svg)](https://crates.io/crates/runtasktic)

Runtasktic is a *fantastic* command-line task management tool for execution of regular long sequential or parallel tasks.

There are often tasks that we repeat several times in predefined orders. Some of these tasks can take time and we would like to be notified when it ends. This is why this project exists.

Describe your tasks in a YAML file, execute all of them with runtasktic in foreground or background. Configure the notification system, and be notified at the end of each task or when they are all finished.

## When should I need runtasktic ?

- when you have a redundant long running task list
- when you need a notification after a long task completion

## CLI

```
runtasktic 0.4.0
Jones Magloire @Joxit
Command-line task management tool for execution of regular long sequential or parallel tasks.

USAGE:
    runtasktic <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    completion    Generate completion script for your shell
    dot           Export the configuration to a graph (needs graphviz/dot)
    exec          Execute a single command with notification
    help          Prints this message or the help of the given subcommand(s)
    run           Run tasks
```

### Run: all tasks of a configuration file

```
runtasktic-run 0.4.0
Run tasks

USAGE:
    runtasktic run [FLAGS] [OPTIONS] [--] [config]...

FLAGS:
    -b, --background    Run the task in background
    -h, --help          Prints help information
    -V, --version       Prints version information

OPTIONS:
    -s, --start <starts>...    Override the starting task if the job had already been started before. When using many
                               configuration files, start states must be in the first configuration file. Can be many
                               task ids with comma separated values

ARGS:
    <config>...    Configurations path (YAML)
```

### Exec: Simple command, just like nohup with notification

```
runtasktic-exec 0.4.0
Execute a single command with notification

USAGE:
    runtasktic exec [FLAGS] [OPTIONS] [command]...

FLAGS:
    -b, --background    Exec the command in background
    -h, --help          Prints help information
    -V, --version       Prints version information

OPTIONS:
    -c, --config <config>    Configuration path (YAML)
    -t, --task <task>        Run a single task from the configuration file

ARGS:
    <command>...    Command to execute
```

### Dot: Create a graph using graphviz of your configuration file

```
runtasktic-dot 0.4.0
Export the configuration to a graph (needs graphviz/dot)

USAGE:
    runtasktic dot <config> <image>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <config>    Path of the configuration file to visualize
    <image>     Path for the image. `dot` command is required
```

### Completion: Generate completion script for your shell

```
runtasktic-completion 0.4.0
Generate completion script for your shell

USAGE:
    runtasktic completion <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    bash      Generates a .bash completion file for the Bourne Again SHell (BASH). Save the output in
              `/etc/bash_completion.d/runtasktic` or `~/.local/share/bash-completion/completions/runtasktic`
    elvish    Generates a completion file for Elvish
    fish      Generates a .fish completion file for the Friendly Interactive SHell (fish)
    help      Prints this message or the help of the given subcommand(s)
    zsh       Generates a completion file for the Z SHell (ZSH)
```

## Configuration

```yaml
tasks:
  a: # The id of the task
    commands: # Commands to execute, they must exist with a 0 exit code
      - echo Begin a 
      - sleep 0.5
      - echo End a
    on_failure: exit # `continue` or `exit` when the tasks ends with a non 0 exit code
  b:
    commands:
      - echo Begin b
      - sleep 0.25
      - echo End b
    depends_on: [ a ] # This task will be executed after a. 
notification:
  slack: # send notification to slack
    url: https://hooks.slack.com/services/XXXXX/XXXXX/XXXXX # The slack server url
    channel: '#channel' # channel to send message
    emoji: ':rocket:' # emoji to use (optional)
    username: runtasktic # the username to use, default is runtasktic.
  print:
    output: stderr # print notification on `stdout`, `stedrr`, `none` or `/custom/path`
  when: always # `always`, `task-end`, `end` or `never` when should I send notification
  messages:
    task_end: Task {task.id} ended with status code {task.status_code} # Availables templates are {task.id}, {task.short_cmd}, {task.full_cmd}, {task.status_code}, {hostname}, {env.*} for environment variables 
    all_task_end: All tasks ended. Got {resume.success} success and {resume.failures} failure. # Availables templates are {resulme.success}, {resume.failures}, {hostname}, {env.*} for environment variables
    task_failed: Tasks ended prematurely. Got {resume.success} success and {resume.failures} failure. Contains one critical failure. # Availables templates are {resulme.success}, {resume.failures}, {hostname}, {env.*} for environment variables. Triggered when `on_failure: exit` is used.

concurrency: 2 # how many task can run simultaneously
working_dir: /custom/directory # Where is the workind directory, default is where your are using runtasktic
stdout: none # `none`, `/custom/path` where should I save standard logs
stderr: /var/log/runtasktic.err # `none`, `/custom/path` where should I save error logs
on_failure: continue # `continue` or `exit` default behaviour when a task fail, default is `continue`
```

### Configuration examples

[Simple sample](https://github.com/Joxit/task-scheduler/blob/master/tests/resources/sample.yml)

[Play with concurrency](https://github.com/Joxit/task-scheduler/blob/master/tests/resources/concurrency.yml)

[Play with notification](https://github.com/Joxit/task-scheduler/blob/master/tests/resources/notification.yml)