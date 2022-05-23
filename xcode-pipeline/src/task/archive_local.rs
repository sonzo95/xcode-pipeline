use std::path::Path;

use args::{Args, ArgsError};
use getopts::Occur;
use macros::Task;

use crate::{
    filesystem::{FileSystemRepository, FileSystemRepositoryFsImpl},
    xcodebuild::{XcodebuildCommandFactory, XcodebuildContext, XcodebuildContextLocalWs},
    Input, ParseResult,
};

use super::task::{Task, TaskParseResult};

const TASK_DESC: &'static str =
    "Archive one or more schemes of your iOS app project using your local workspace";
const TASK_NAME: &'static str = "archiveLocal";

#[derive(Task)]
pub struct ArchiveLocal {
    xcb_context: Box<dyn XcodebuildContext>,
}

impl ArchiveLocal {
    pub fn new(input: &Vec<String>) -> Result<TaskParseResult, ArgsError> {
        let mut args = Args::new(TASK_NAME, TASK_DESC);
        args.flag("h", "help", "Print the usage menu");
        args.flag(
            "",
            "dry-run",
            "Run the script without archiving or exporting",
        );
        args.option(
            "s",
            "schema",
            "A schema to build",
            "SCHEMA",
            Occur::Multi,
            None,
        );
        args.option(
            "w",
            "workspace",
            "The path to the workspace directory",
            "WORKSPACE",
            Occur::Optional,
            None,
        );
        args.parse(input)?;

        let help = args.value_of("help")?;
        if help {
            println!("{}", args.full_usage());
            return Ok(TaskParseResult::Help);
        }

        let dry_run: bool = args.value_of("dry-run")?;
        let schemes = args.values_of::<String>("schema")?;

        let workspace = args
            .value_of::<String>("workspace")
            .unwrap_or(".".to_string());
        let workspace_path = Path::new(&workspace);

        let fs_repo = Box::new(FileSystemRepositoryFsImpl {});

        let command_factory = Box::new(XcodebuildCommandFactory::new(dry_run));
        let context = Box::new(XcodebuildContextLocalWs::new(
            workspace_path.to_path_buf(),
            Path::new("/tmp").to_path_buf(),
            fs_repo,
            command_factory,
        ));

        Ok(TaskParseResult::Task(Box::new(Self {
            xcb_context: context,
        })))
    }
}

impl Task for ArchiveLocal {
    fn run(&self) {
        // println!("Executing with inputs: {:?}", input);
        self.xcb_context.setup();
        self.xcb_context.tear_down();
    }
}
