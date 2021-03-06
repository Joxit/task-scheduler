use runtasktic::config::*;
use std::fs;

const SAMPLE_YAML: &str = "tests/resources/sample.yml";
const NOTIFICATION_YAML: &str = "tests/resources/notification.yml";
const CONCURRENCY_YAML: &str = "tests/resources/concurrency.yml";
const ON_FAILURE_YAML: &str = "tests/resources/on_failure.yml";

#[test]
fn sample_yaml() {
  let yaml = fs::read_to_string(SAMPLE_YAML).unwrap();
  let config = Config::from_str(yaml.as_str()).unwrap();

  let a = Task::new("a".to_string(), vec![echo("a"), sleep("0.5")], vec![], None);
  let b = Task::new(
    "b".to_string(),
    vec![echo("b"), sleep("0.5")],
    vec!["a".to_string()],
    None,
  );
  let c = Task::new(
    "c".to_string(),
    vec![echo("c")],
    vec!["a".to_string()],
    None,
  );
  let d = Task::new(
    "d".to_string(),
    vec![echo("d")],
    vec!["b".to_string(), "c".to_string()],
    None,
  );

  assert_eq!(*config.notification(), None);
  assert_eq!(config.concurrency(), -1);
  assert_eq!(
    *config.working_dir(),
    Some(String::from("/custom/directory"))
  );
  assert_eq!(*config.stdout(), Some(String::from("none")));
  assert_eq!(*config.stderr(), Some(String::from("none")));
  assert_eq!(config.tasks().len(), 4);
  assert_eq!(config.tasks().get(&"a".to_string()), Some(&a));
  assert_eq!(config.tasks().get(&"b".to_string()), Some(&b));
  assert_eq!(config.tasks().get(&"c".to_string()), Some(&c));
  assert_eq!(config.tasks().get(&"d".to_string()), Some(&d));
}

#[test]
fn concurrency_yaml() {
  let yaml = fs::read_to_string(CONCURRENCY_YAML).unwrap();
  let config = Config::from_str(yaml.as_str()).unwrap();

  let a = Task::new(
    "a".to_string(),
    vec![echo("Begin a"), sleep("0.5"), echo("End a")],
    vec![],
    None,
  );
  let b = Task::new(
    "b".to_string(),
    vec![echo("Begin b"), sleep("0.5"), echo("End b")],
    vec!["a".to_string()],
    None,
  );
  let c = Task::new(
    "c".to_string(),
    vec![echo("Begin c"), sleep("1"), echo("End c")],
    vec!["a".to_string()],
    None,
  );
  let d = Task::new(
    "d".to_string(),
    vec![echo("Begin d"), sleep("0.5"), echo("End d")],
    vec!["a".to_string()],
    None,
  );
  let e = Task::new(
    "e".to_string(),
    vec![echo("Begin e"), sleep("0.5"), echo("End e")],
    vec!["b".to_string(), "c".to_string(), "d".to_string()],
    None,
  );

  assert_eq!(*config.notification(), None);
  assert_eq!(config.concurrency(), 2);
  assert_eq!(config.tasks().len(), 5);
  assert_eq!(config.tasks().get(&"a".to_string()), Some(&a));
  assert_eq!(config.tasks().get(&"b".to_string()), Some(&b));
  assert_eq!(config.tasks().get(&"c".to_string()), Some(&c));
  assert_eq!(config.tasks().get(&"d".to_string()), Some(&d));
  assert_eq!(config.tasks().get(&"e".to_string()), Some(&e));
}

#[test]
fn notification_yaml() {
  let yaml = fs::read_to_string(NOTIFICATION_YAML).unwrap();
  let config = Config::from_str(yaml.as_str()).unwrap();

  let a = Task::new(
    "a".to_string(),
    vec![echo("Begin a"), sleep("0.5"), echo("End a")],
    vec![],
    None,
  );
  let b = Task::new(
    "b".to_string(),
    vec![echo("Begin b"), sleep("0.5"), echo("End b")],
    vec!["a".to_string()],
    None,
  );
  let c = Task::new(
    "c".to_string(),
    vec![echo("Begin c"), sleep("1"), echo("End c")],
    vec!["a".to_string()],
    None,
  );
  let d = Task::new(
    "d".to_string(),
    vec![echo("Begin d"), sleep("0.5"), echo("End d")],
    vec!["a".to_string()],
    None,
  );
  let e = Task::new(
    "e".to_string(),
    vec![echo("Begin e"), sleep("0.5"), echo("End e")],
    vec!["b".to_string(), "c".to_string(), "d".to_string()],
    None,
  );

  let slack = Slack::new(
    "https://hooks.slack.com/services/XXXXX/XXXXX/XXXXX".to_string(),
    "#channel".to_string(),
    Some("runtasktic".to_string()),
    Some(":rocket:".to_string()),
    None,
  );
  let print = Print::new("stderr".to_string(), None);
  let notification = Notification::new(
    Some(slack),
    Some(print),
    WhenNotify::Always,
    Messages::default(),
  );

  assert_eq!(*config.notification(), Some(notification));
  assert_eq!(config.concurrency(), -1);
  assert_eq!(config.tasks().len(), 5);
  assert_eq!(config.tasks().get(&"a".to_string()), Some(&a));
  assert_eq!(config.tasks().get(&"b".to_string()), Some(&b));
  assert_eq!(config.tasks().get(&"c".to_string()), Some(&c));
  assert_eq!(config.tasks().get(&"d".to_string()), Some(&d));
  assert_eq!(config.tasks().get(&"e".to_string()), Some(&e));
}

#[test]
fn on_failure_yaml() {
  let yaml = fs::read_to_string(ON_FAILURE_YAML).unwrap();
  let config = Config::from_str(yaml.as_str()).unwrap();

  let a = Task::new(
    "a".to_string(),
    vec![echo("Begin a"), format!("unknown-cmd"), echo("End a")],
    vec![],
    Some(OnFailure::Continue),
  );
  let b = Task::new(
    "b".to_string(),
    vec![echo("Begin b"), format!("unknown-cmd"), echo("End b")],
    vec!["a".to_string()],
    Some(OnFailure::Exit),
  );
  let c = Task::new(
    "c".to_string(),
    vec![echo("Begin c"), sleep("1"), echo("End c")],
    vec!["a".to_string()],
    None,
  );
  let d = Task::new(
    "d".to_string(),
    vec![echo("Begin d"), sleep("0.5"), echo("End d")],
    vec!["a".to_string()],
    None,
  );
  let e = Task::new(
    "e".to_string(),
    vec![echo("Begin e"), sleep("0.5"), echo("End e")],
    vec!["b".to_string(), "c".to_string(), "d".to_string()],
    None,
  );

  assert_eq!(config.on_failure(), &OnFailure::Continue);
  assert_eq!(config.notification(), &None);
  assert_eq!(config.concurrency(), 2);
  assert_eq!(config.tasks().len(), 5);
  assert_eq!(config.tasks().get(&"a".to_string()), Some(&a));
  assert_eq!(config.tasks().get(&"b".to_string()), Some(&b));
  assert_eq!(config.tasks().get(&"c".to_string()), Some(&c));
  assert_eq!(config.tasks().get(&"d".to_string()), Some(&d));
  assert_eq!(config.tasks().get(&"e".to_string()), Some(&e));
}

fn echo(msg: &str) -> String {
  format!("echo {}", msg)
}

fn sleep(time: &str) -> String {
  format!("sleep {}", time)
}
