use linked_hash_map::LinkedHashMap;
use yaml_rust::Yaml;

const TASKS_KEY: &str = "tasks";
const COMMAND_KEY: &str = "commands";
const DEPENDS_ON_KEY: &str = "depends_on";
const CONCURRENCY_KEY: &str = "concurrency";

pub trait YamlTasksScheduler {
  fn get_tasks(&self) -> Result<&LinkedHashMap<Yaml, Yaml>, String>;
  fn get_commands(&self) -> Vec<String>;
  fn get_depends_on(&self) -> Vec<String>;
  fn get_concurrency(&self) -> Result<i64, String>;
}

impl YamlTasksScheduler for LinkedHashMap<Yaml, Yaml> {
  fn get_tasks(&self) -> Result<&LinkedHashMap<Yaml, Yaml>, String> {
    if let Some(tasks) = self.get(&Yaml::String(String::from(TASKS_KEY))) {
      if let Some(tasks) = tasks.as_hash() {
        return Ok(tasks);
      }
    }
    Err(String::from("Tasks not found"))
  }

  fn get_commands(&self) -> Vec<String> {
    if let Some(commands) = self.get(&Yaml::String(String::from(COMMAND_KEY))) {
      if let Some(commands) = commands.as_vec() {
        return commands
          .iter()
          .map(|c| c.as_str())
          .filter(|c| c.is_some())
          .map(|c| c.unwrap().to_string())
          .collect();
      }
    }
    vec![]
  }

  fn get_depends_on(&self) -> Vec<String> {
    if let Some(commands) = self.get(&Yaml::String(String::from(DEPENDS_ON_KEY))) {
      if let Some(commands) = commands.as_vec() {
        return commands
          .iter()
          .map(|c| c.as_str())
          .filter(|c| c.is_some())
          .map(|c| c.unwrap().to_string())
          .collect();
      }
    }
    vec![]
  }

  fn get_concurrency(&self) -> Result<i64, String> {
    if let Some(concurrency) = self.get(&Yaml::String(String::from(CONCURRENCY_KEY))) {
      if let Some(concurrency) = concurrency.as_i64() {
        if concurrency < 1 {
          Err(String::from("Concurrency must be greater than 0 !"))
        } else {
          Ok(concurrency)
        }
      } else {
        Err(String::from("Incorrect value, should be an integer"))
      }
    } else {
      Ok(-1)
    }
  }
}

impl YamlTasksScheduler for Yaml {
  fn get_tasks(&self) -> Result<&LinkedHashMap<Yaml, Yaml>, String> {
    self.as_hash().unwrap().get_tasks()
  }

  fn get_commands(&self) -> Vec<String> {
    if let Some(commands) = self.as_hash() {
      commands.get_commands()
    } else {
      vec![]
    }
  }

  fn get_depends_on(&self) -> Vec<String> {
    if let Some(depends_on) = self.as_hash() {
      depends_on.get_depends_on()
    } else {
      vec![]
    }
  }

  fn get_concurrency(&self) -> Result<i64, String> {
    if let Some(depends_on) = self.as_hash() {
      depends_on.get_concurrency()
    } else {
      Ok(-1)
    }
  }
}
