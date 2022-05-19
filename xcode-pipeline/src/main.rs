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
use task::task::{Named, Task, TaskGenerator, TaskFactory};
use xcodebuild::{XcodebuildContext, XcodebuildContextLocalWs};

use args::{Args, ArgsError};

use crate::task::cd_local::CDLocal;
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
        self.map.insert(T::name(), T::get_factory());
    }

    
    fn make_task(&self, name: &str, args: Vec<String>) -> Option<Box<dyn Task>> {
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
    use crate::{TaskRegistry, task::task::{Task, Named, TaskGenerator, TaskFactory}};

    struct TaskImpl {}
    impl TaskImpl {
        fn new(_: &Vec<String>) -> Box<dyn Task> {
            Box::new(TaskImpl {  })
        }
    }
    impl Task for TaskImpl {
        fn run(&self) {
            println!("TaskImpl running!");
        }
    }
    // Named and TaskGenerator could be implemented via derive macro
    impl Named for TaskImpl {
        fn name() -> String {
            "impl".to_owned()
        }
    }
    impl TaskGenerator for TaskImpl {
        fn get_factory() -> TaskFactory {
            TaskFactory { 
                instantiate: TaskImpl::new,
            }
        }
    }

    #[test]
    fn task_registry() {
        let mut registry = TaskRegistry::new();
        registry.register::<TaskImpl>();
        let task = registry.make_task("impl", vec!("parm".to_owned())).unwrap();
        task.run();
    }
}