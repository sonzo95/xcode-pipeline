mod filesystem;
mod git;
mod validation;
mod xcodebuild;

extern crate args;
extern crate getopts;

use std::env;
use std::path::Path;

use filesystem::repository_impl::FileSystemRepositoryFsImpl;
use getopts::Occur;
use xcodebuild::{XcodebuildContext, XcodebuildContextLocalWs};

use args::{Args, ArgsError};

use crate::xcodebuild::xcodebuild_command_factory::XcodebuildCommandFactory;

const PROGRAM_DESC: &'static str = "Archive and export one or more schemes of your iOS app project";
const PROGRAM_NAME: &'static str = "xc-cd";

fn main() {
    let args: Vec<String> = env::args().collect();

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
enum ParseResult {
    Help,
    Input(Input),
}

fn parse(input: &Vec<String>) -> Result<ParseResult, ArgsError> {
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
