use std::path::Path;

use args::{validations::Validation, ArgsError};

struct PathValidation {
    absolute: bool,
    relative: bool,
    dir: bool,
    file: bool,
}

impl PathValidation {
    fn new(absolute: bool, relative: bool, dir: bool, file: bool) -> Self {
        Self {
            absolute,
            relative,
            dir,
            file,
        }
    }
}

impl Validation for PathValidation {
    type T = String;

    fn error(&self, value: &Self::T) -> ArgsError {
        ArgsError::new("", "{value} is not a valid filesystem path.")
    }

    fn is_valid(&self, value: &Self::T) -> bool {
        let path = Path::new(value);
        if self.absolute && !path.is_absolute() {
            return false;
        };
        if self.relative && !path.is_relative() {
            return false;
        };
        if self.dir && !path.is_dir() {
            return false;
        };
        if self.file && !path.is_file() {
            return false;
        };
        true
    }
}
