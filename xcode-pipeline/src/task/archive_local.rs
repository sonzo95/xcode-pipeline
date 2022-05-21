use args::{Args, ArgsError};
use getopts::Occur;
use macros::Task;

use crate::{Input, ParseResult};

use super::task::Task;

const TASK_DESC: &'static str =
    "Archive one or more schemes of your iOS app project using your local workspace";
const TASK_NAME: &'static str = "archiveLocal";

#[derive(Task)]
pub struct ArchiveLocal {}

impl ArchiveLocal {
    pub fn new(input: &Vec<String>) -> Result<Box<dyn Task>, ArgsError> {
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
            "b",
            "branch",
            "The branch of the repository to clone and process",
            "BRANCH",
            Occur::Optional,
            None,
        );
        args.parse(input)?;

        let help = args.value_of("help")?;
        if help {
            println!("{}", args.full_usage());
            return Ok(Box::new(Self {})); // todo should return help
        }

        let dry_run: bool = args.value_of("dry-run")?;
        let schemes = args.values_of::<String>("schema")?;
        let branch = args.value_of::<String>("branch")?;

        Ok(Box::new(Self {}))
    }
}

impl Task for ArchiveLocal {
    fn run(&self) {
        todo!()
    }
}
