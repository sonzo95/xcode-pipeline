use std::path::Path;

use args::{Args, ArgsError};
use core_derive::Task;
use getopts::Occur;

use serde::Deserialize;
use tracing::{event, Level};

use core::{
    filesystem::FileSystemRepositoryFsImpl,
    xcodebuild::{XcodebuildCommandFactory, XcodebuildContext, XcodebuildContextLocalWs},
};

use super::task::{Task, TaskParseResult};

const TASK_DESC: &'static str =
    "Archive one or more schemes of your iOS app project using your local workspace";
const TASK_NAME: &'static str = "archiveLocal";

#[derive(Task)]
pub struct ArchiveLocal {
    xcb_context: Box<dyn XcodebuildContext>,
    schemes: Vec<String>,
    asc_username: String,
    asc_password: String,
    no_tear_down: bool,
}

#[derive(Deserialize)]
struct Config {
    pub workspace: String,
    pub export_options_plist: String,
    // maybe add the list of schemes here
}

impl ArchiveLocal {
    pub fn new(input: &Vec<String>) -> Result<TaskParseResult, ArgsError> {
        // Read config from xccd.toml
        let config_contents = std::fs::read_to_string("xccd.toml")
            .expect("Couldn't open or find `xccd.toml` in the current directory");
        let config: Config = toml::from_str(&config_contents)
            .expect("Couldn't parse `xccd.toml` as a valid toml file");

        // Parse command line args
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
        args.flag(
            "",
            "no-tear-down",
            "Ignores the tear down operation and leaves the temporary working directory intact.",
        );
        args.option(
            "u",
            "username",
            "Appstoreconnect account username",
            "USERNAME",
            Occur::Req,
            None,
        );
        args.option(
            "p",
            "password",
            "Appstoreconnect app-specific password",
            "PASSWORD",
            Occur::Req,
            None,
        );
        args.parse(input)?;

        let help = args.value_of("help")?;
        if help {
            println!("{}", args.full_usage());
            return Ok(TaskParseResult::Help);
        }

        let dry_run: bool = args.value_of("dry-run")?;
        let no_tear_down: bool = args.value_of("no-tear-down")?;
        let schemes = args.values_of::<String>("schema")?;
        let username = args.value_of("username")?;
        let password = args.value_of("password")?;

        // TODO maybe just pass the whole config struct
        let workspace_path = Path::new(&config.workspace);
        let export_options_plist_path = Path::new(&config.export_options_plist);

        let fs_repo = Box::new(FileSystemRepositoryFsImpl {});

        let command_factory = Box::new(XcodebuildCommandFactory::new(dry_run));
        let context = Box::new(XcodebuildContextLocalWs::new(
            workspace_path.to_path_buf(),
            Path::new("/tmp").to_path_buf(),
            export_options_plist_path.to_path_buf(),
            fs_repo,
            command_factory,
        ));

        Ok(TaskParseResult::Task(Box::new(Self {
            xcb_context: context,
            schemes,
            asc_username: username,
            asc_password: password,
            no_tear_down,
        })))
    }
}

impl Task for ArchiveLocal {
    fn run(&self) {
        event!(Level::TRACE, "Executing task ArchiveLocal");
        self.xcb_context.setup();
        for schema in &self.schemes {
            event!(Level::INFO, "Executing operations for schema {}", schema);

            event!(Level::INFO, "Archiving...");
            // TODO handle exit codes better
            let archive_exit = self.xcb_context.archive(&schema);
            if !archive_exit.success() {
                event!(
                    Level::ERROR,
                    "Archive of schema {} failed with status: {}",
                    schema,
                    archive_exit,
                );
                continue;
            }
            event!(Level::INFO, "Archive finished");

            event!(Level::INFO, "Exporting ipa file...");
            let export_exit = self.xcb_context.export(&schema);
            if !export_exit.success() {
                event!(
                    Level::ERROR,
                    "Export of schema {} failed with status: {}",
                    schema,
                    export_exit
                );
                continue;
            }
            event!(Level::INFO, "Export finished");

            event!(Level::INFO, "Uploading...");
            let upload_exit =
                self.xcb_context
                    .upload(&schema, &self.asc_username, &self.asc_password);
            if !upload_exit.success() {
                event!(
                    Level::ERROR,
                    "Upload of schema {} failed with status: {}",
                    schema,
                    upload_exit
                );
                continue;
            }
            event!(Level::INFO, "Upload finished");
        }
        if !self.no_tear_down {
            self.xcb_context.tear_down();
        }
    }
}
