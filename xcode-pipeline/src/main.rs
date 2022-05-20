mod filesystem;
mod git;
mod task;
mod validation;
mod xcodebuild;

extern crate args;
extern crate getopts;

use std::any::{Any, TypeId};
use std::fmt::Display;
use std::path::{Path};
use std::{collections::HashMap, env};

use filesystem::repository_impl::FileSystemRepositoryFsImpl;
use getopts::Occur;
use task::archive_local::ArchiveLocal;
use task::task::{Named, Task, TaskFactory, TaskGenerator};
use task::task_registry::TaskRegistry;
use xcodebuild::{XcodebuildContext, XcodebuildContextLocalWs};

use args::{Args, ArgsError};

use crate::xcodebuild::xcodebuild_command_factory::XcodebuildCommandFactory;

const PROGRAM_DESC: &'static str = 
r"Archive and export one or more schemes of your iOS app project.
USAGE: xccd [task] [options...]";
const PROGRAM_NAME: &'static str = "xccd";

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut registry = TaskRegistry::new();
    registry.register::<ArchiveLocal>();
    parse(&args, &registry);

    //let task_name = &args[1];

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

enum XccdArgs {
    Help,
}

mod arg {
    pub const HELP: &'static str = "help";
}

fn parse(input: &Vec<String>, registry: &TaskRegistry) -> Result<ParseResult<String>, args::ArgsError> {
    let mut args = Args::new(PROGRAM_NAME, PROGRAM_DESC);
    args.flag("h", arg::HELP, "Print the usage menu");
    args.parse(input)?;

    let help = args.value_of(arg::HELP)?;
    if help {
        println!("{}", args.full_usage());
        println!("Tasks:\n    {}", registry.task_names().join(", "));
        return Ok(ParseResult::Help);
    }

    Err(ArgsError::new("asd", "dd"))
    //Ok(ParseResult::Input())
}