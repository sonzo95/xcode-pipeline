use std::collections::HashMap;

use args::ArgsError;

use super::task::{Named, Task, TaskFactory, TaskGenerator};

pub struct TaskRegistry {
    map: HashMap<String, TaskFactory>,
}

impl TaskRegistry {
    pub fn new() -> Self {
        TaskRegistry {
            map: HashMap::new(),
        }
    }

    pub fn register<T: Named + TaskGenerator>(&mut self) {
        let name = T::name();
        println!("Registering component as {}", name);
        self.map.insert(name, T::get_factory());
    }

    pub fn make_task(
        &self,
        name: &str,
        args: &Vec<String>,
    ) -> Option<Result<Box<dyn Task>, ArgsError>> {
        self.map
            .get(name)
            .map(|factory| (factory.instantiate)(&args))
    }

    pub fn task_names(&self) -> Vec<String> {
        self.map
            .keys()
            .into_iter()
            .map(|key| key.to_owned())
            .collect()
    }
}

mod tests {
    use args::ArgsError;
    use macros::Task;

    use crate::task::{task::Task, task_registry::TaskRegistry};

    #[derive(Task)]
    struct TaskImpl {}

    impl TaskImpl {
        fn new(_: &Vec<String>) -> Result<Box<dyn Task>, ArgsError> {
            Ok(Box::new(TaskImpl {}))
        }
    }
    impl Task for TaskImpl {
        fn run(&self) {
            println!("TaskImpl running!");
        }
    }

    #[test]
    fn task_registry() {
        let mut registry = TaskRegistry::new();
        registry.register::<TaskImpl>();
        let task = registry
            .make_task("taskImpl", &vec!["param".to_owned()])
            .expect("taskImpl not registered")
            .expect("task could not be instantiated");
        task.run();
        assert_eq!(registry.task_names(), vec!("taskImpl"));
    }
}
