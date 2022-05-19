use args::ArgsError;

use super::cd_local::CDLocal;

//pub trait TaskGenerator3 = &'static Fn(Vec<String>) -> Result<dyn Task, ArgsError>;

pub trait Named {
    fn name() -> String;
}

pub trait Task {
    //fn new(args: &Vec<String>) -> Result<dyn Task, ArgsError>;
    fn run(&self);
}


pub trait TaskGenerator {
    fn get_factory() -> TaskFactory<'static>;
}
pub struct TaskFactory<'a> {
    instantiate: &'a dyn Fn(&Vec<String>) -> dyn Task,
}

impl<'a> TaskFactory<'a> {
    fn create_task(&self, args: &Vec<String>) -> Box<dyn Task> {
        Box::new((self.instantiate)(args))
    }
}



impl CDLocal {
    fn new(args: &Vec<String>) -> Self {todo!()}
}

impl TaskGenerator for CDLocal {
    fn get_factory() -> TaskFactory<'static> {
        TaskFactory { instantiate: &Self::new }
    }
}
