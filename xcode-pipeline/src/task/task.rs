use args::ArgsError;

pub enum TaskParseResult {
    Help,
    Task(Box<dyn Task>),
}

pub trait Named {
    fn name() -> String;
}

pub trait Task {
    fn run(&self);
}

pub trait TaskGenerator {
    fn get_factory() -> TaskFactory;
}

pub struct TaskFactory {
    pub instantiate: fn(&Vec<String>) -> Result<TaskParseResult, ArgsError>,
}
