mod filesystem;
mod git;
mod validation;
mod xcodebuild;
mod task;

extern crate args;
extern crate getopts;

use std::any::{TypeId, Any};
use std::{env, collections::HashMap};
use std::path::Path;

use filesystem::repository_impl::FileSystemRepositoryFsImpl;
use getopts::Occur;
use task::task::{Task, TaskGenerator, Named};
use xcodebuild::{XcodebuildContext, XcodebuildContextLocalWs};

use args::{Args, ArgsError};

use crate::task::cd_local::CDLocal;
use crate::xcodebuild::xcodebuild_command_factory::XcodebuildCommandFactory;

struct FnContainer {
    f: &'static dyn Fn(&Vec<String>) -> dyn Task
}

struct TaskRegistry {
    map: HashMap<String, Box<dyn Any>>,
}

impl TaskRegistry {
    fn new() -> Self {
        TaskRegistry { 
            map: HashMap::new(),
        }
    }

    fn register<T: Named + TaskGenerator>(&mut self) {
        self.map[&T::name()] = Box::new(FnContainer { f: &T::new });
    }

    fn make_task(&self, name: &str, args: Vec<String>) -> Option<Box<dyn Task>> {
        self.map.get(name).map(
            |&task_gen| -> Box<dyn Task> {task_gen.downcast::<dyn Fn(&Vec<String>) -> dyn Task>()(args)}
        )
    }

    fn get_tasks(&self) -> Vec<String> {
        self.map.keys()
    }
}



fn main() {
    let args: Vec<String> = env::args().collect();

    let registry = TaskRegistry::new();
    registry.register::<CDLocal>();

    let task_name = args[1];

    let input: Input;
    match parse(&args).unwrap() {
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
    context.tear_down();
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
