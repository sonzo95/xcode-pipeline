use args::Args;
use getopts::Occur;

use crate::{ParseResult, Input};

use super::task::Task;

const PROGRAM_DESC: &'static str = "Archive and export one or more schemes of your iOS app project";
const PROGRAM_NAME: &'static str = "xc-cd";

pub struct CDLocal {

}

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