use args::Args;
use getopts::Occur;

use crate::{Input, ParseResult};

use super::task::Task;

const TASK_DESC: &'static str =
    "Archive one or more schemes of your iOS app project, cloned from a remote git repository";
const TASK_NAME: &'static str = "xc-cd";

pub struct ArchiveGit {}
/*
impl Task for CDLocal {
    fn name() -> String {
        "local"
    }

    fn new(args: &Vec<String>) -> Result<Self, args::ArgsError> {
        let mut args = Args::new(PROGRAM_NAME, PROGRAM_DESC);
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
            return Ok(ParseResult::Help);
        }

        let dry_run: bool = args.value_of("dry-run")?;
        let schemes = args.values_of::<String>("schema")?;
        let branch = args.value_of("branch")?;

        Ok(ParseResult::Input(Input {
            dry_run,
            schemes,
            branch,
        }))
    }

    fn run(&self) {
        todo!()
    }
}
*/
