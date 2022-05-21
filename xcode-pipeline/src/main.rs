mod filesystem;
mod git;
mod task;
mod validation;
mod xcodebuild;

extern crate args;
extern crate getopts;

use task::archive_local::ArchiveLocal;
use task::task_registry::TaskRegistry;

use args::{Args, ArgsError};

const PROGRAM_DESC: &'static str = r"Archive and export one or more schemes of your iOS app project.
USAGE: xccd [task] [options...]";
const PROGRAM_NAME: &'static str = "xccd";

fn main() {
    let mut registry = TaskRegistry::new();
    registry.register::<ArchiveLocal>();

    let input_args: Vec<String> = std::env::args().collect();
    let mut args = get_main_args();
    let cmd = parse(&mut args, &input_args, &registry).unwrap();

    let task_name = match cmd {
        ParseResult::Help => return,
        ParseResult::Input(s) => s,
    };

    let task = registry
        .make_task(&task_name, &input_args)
        .expect(&format!(
            "Task {} not found. Use --help to see a list of supported tasks.",
            task_name
        ))
        .unwrap();

    task.run();
    /*println!("Executing with inputs: {:?}", input);

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

#[derive(Debug, Clone, Copy)]
enum TokenType {
    Word,
    Option,
}

fn get_token_type(token: &str) -> TokenType {
    if token.starts_with("-") || token.starts_with("--") {
        return TokenType::Option;
    }
    TokenType::Word
}

fn get_main_args() -> Args {
    let mut args = Args::new(PROGRAM_NAME, PROGRAM_DESC);
    args.flag("h", "help", "Print the usage menu");
    args
}

fn parse(
    args: &mut Args,
    input: &Vec<String>,
    registry: &TaskRegistry,
) -> Result<ParseResult<String>, args::ArgsError> {
    match get_token_type(&input[1]) {
        TokenType::Word => {
            return Ok(ParseResult::Input(input[1].to_owned()));
        }
        _ => {}
    };

    args.parse(input)?;

    let help = args.value_of("help")?;
    if help {
        print_help(args, registry);
        return Ok(ParseResult::Help);
    }

    Err(ArgsError::new(
        "global",
        "First arg must be a task name. Use --help to see a list of supported tasks.",
    ))
}

fn print_help(args: &Args, registry: &TaskRegistry) {
    println!("{}", args.full_usage());
    println!("Tasks:\n    {}", registry.task_names().join(", "));
}
