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
use xcodebuild::{XcodebuildContext, XcodebuildContextImpl};

use args::{Args, ArgsError};

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
    let context = XcodebuildContextImpl::new(
        Path::new(".").to_path_buf(),
        "Development",
        false,
        Path::new("/tmp").to_path_buf(),
        &fs_repo,
    );
    context.setup();
    context.tear_down();
}

#[derive(Debug, Clone)]
struct Input {
    dry_run: bool,
    schemes: Vec<String>,
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
    args.parse(input)?;

    let help = args.value_of("help")?;
    if help {
        println!("{}", args.full_usage());
        return Ok(ParseResult::Help);
    }

    let dry_run: bool = args.value_of("dry-run")?;
    let schemes = args.values_of::<String>("schema")?;

    Ok(ParseResult::Input(Input { dry_run, schemes }))
}
