mod filesystem;
mod git;
mod xcodebuild;

extern crate args;
extern crate getopts;

use std::env;

use filesystem::repository_impl::FileSystemRepositoryFsImpl;
use getopts::Occur;
use std::process::exit;
use xcodebuild::{XcodebuildContext, XcodebuildContextImpl};

use args::validations::{Order, OrderValidation};
use args::{Args, ArgsError};

const PROGRAM_DESC: &'static str = "Run this program";
const PROGRAM_NAME: &'static str = "program";

fn main() {
    let args: Vec<String> = env::args().collect();

    let fs_repo = FileSystemRepositoryFsImpl {};
    let context = XcodebuildContextImpl::new(".", "Development", false, "/tmp/prova", &fs_repo);
    context.setup();
    context.tear_down();

    match parse(&args) {
        Ok(_) => println!("Successfully parsed args"),
        Err(error) => {
            println!("{}", error);
            exit(1);
        }
    };
}

fn parse(input: &Vec<String>) -> Result<(), ArgsError> {
    let mut args = Args::new(PROGRAM_NAME, PROGRAM_DESC);
    args.flag("h", "help", "Print the usage menu");
    args.option(
        "i",
        "iter",
        "The number of times to run this program",
        "TIMES",
        Occur::Req,
        None,
    );
    args.option(
        "l",
        "log_file",
        "The name of the log file",
        "NAME",
        Occur::Optional,
        Some(String::from("output.log")),
    );

    args.parse(input)?;

    let help = args.value_of("help")?;
    if help {
        println!("{}", args.full_usage());
        return Ok(());
    }

    let gt_0 = Box::new(OrderValidation::new(Order::GreaterThan, 0u32));
    let lt_10 = Box::new(OrderValidation::new(Order::LessThanOrEqual, 10u32));

    let iters = args.validated_value_of("iter", &[gt_0, lt_10])?;
    for iter in 0..iters {
        println!("Working on iteration {}", iter);
    }
    println!("All done.");

    Ok(())
}
