use args::ArgsError;

//pub trait TaskGenerator3 = &'static Fn(Vec<String>) -> Result<dyn Task, ArgsError>;

pub trait Named {
    fn name() -> String;
}

pub trait Task {
    //fn new(args: &Vec<String>) -> Result<dyn Task, ArgsError>;
    fn run(&self);
}

pub trait TaskGenerator {
    fn get_factory() -> TaskFactory;
}

pub struct TaskFactory {
    pub instantiate: fn(&Vec<String>) -> Result<Box<dyn Task>, ArgsError>,
}