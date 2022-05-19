mod filesystem;
mod git;
mod task;
mod validation;
mod xcodebuild;

extern crate args;
extern crate getopts;

use std::any::{Any, TypeId};
use std::path::Path;
use std::{collections::HashMap, env};

use filesystem::repository_impl::FileSystemRepositoryFsImpl;
use getopts::Occur;
use task::task::{Named, Task, TaskFactory, TaskGenerator};
use xcodebuild::{XcodebuildContext, XcodebuildContextLocalWs};

use args::{Args, ArgsError};

use crate::xcodebuild::xcodebuild_command_factory::XcodebuildCommandFactory;

struct FnContainer {
    f: &'static dyn Fn(&Vec<String>) -> dyn Task,
}

struct TaskRegistry {
    map: HashMap<String, TaskFactory>,
}

impl TaskRegistry {
    fn new() -> Self {
        TaskRegistry {
            map: HashMap::new(),
        }
    }

    fn register<T: Named + TaskGenerator>(&mut self) {
        let name = T::name();
        println!("Registering component as {}", name);
        self.map.insert(name, T::get_factory());
    }

    fn make_task(&self, name: &str, args: Vec<String>) -> Option<Result<Box<dyn Task>, ArgsError>> {
        self.map
            .get(name)
            .map(|factory| (factory.instantiate)(&args))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let registry = TaskRegistry::new();

    let task_name = &args[1];

    let input: Input;
    /*match parse(&args).unwrap() {
        ParseResult::Help => return,
        ParseResult::Input(input_) => input = input_,
    }
    println!("Executing with inputs: {:?}", input);

    let fs_repo = FileSystemRepositoryFsImpl {};
    let command_factory = XcodebuildCommandFactory::new(input.dry_run);
    let context = XcodebuildContextLocalWs::new(
        Path::new(".").to_path_buf(),
        Path::new("/tmp").to_path_buf(),
        &fs_repo,
        &command_factory,
    );
    context.setup();
    context.tear_down();*/
}

#[derive(Debug, Clone)]
struct Input {
    pub dry_run: bool,
    pub schemes: Vec<String>,
    pub branch: String,
}

#[derive(Debug, Clone)]
enum ParseResult<T: Sized> {
    Help,
    Input(T),
}

mod tests {
    use args::ArgsError;
    use macros::Task;

    use crate::{
        task::task::{Named, Task, TaskFactory, TaskGenerator},
        TaskRegistry,
    };

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
            .make_task("taskImpl", vec!["param".to_owned()])
            .expect("taskImpl not registered")
            .expect("task could not be instantiated");
        task.run();
    }
}
