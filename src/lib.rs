// log level
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

// log message
pub struct Message {
    level: String,
    message: String,
    format: Vec<LogField>,
}

impl Message {
    pub fn log_to_std(&self) {}
    pub fn log_to_file(&self, path: String) -> Result<(), std::io::Error> {
        Ok(())
    }
    pub fn generate__string(&self) -> String {
        String::new()
    }
}

pub enum LogField {
    Time,
    Date,
    Event,
    Message,
    Seperator(char),
}

// logger struct
pub struct Logger {
    // logging level
    level_std: Level,
    level_io: Level,
    // path for file logging
    path_io: Option<String>,
    // formatting
    format_std: Vec<LogField>,
    format_io: Vec<LogField>,
}

// logger implementation
impl Logger {
    pub fn new(
        level_std: Level,
        level_io: Level,
        path_io: Option<String>,
        format_std: Vec<LogField>,
        format_io: Vec<LogField>,
    ) -> Self {
        Self {
            level_std,
            level_io,
            path_io,
            format_std,
            format_io,
        }
    }

    pub fn log(&self, level: Level, content: String) {
        // log to file
        if self.path_io.is_some() && self.level_comparison(&self.level_io, &level) {}
        // log to std
        if self.level_comparison(&self.level_std, &level) {}
    }

    // determines if the given log level should be logged
    pub fn level_comparison(&self, current: &Level, given_level: &Level) -> bool {
        current <= given_level
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn higher_loglevel() {
        let logger = Logger::new(Level::Info, Level::Debug, None, Vec::new(), Vec::new());
        assert_eq!(true, logger.level_comparison(&Level::Info, &Level::Error));
    }

    #[test]
    pub fn same_loglevel() {
        let logger = Logger::new(Level::Info, Level::Debug, None, Vec::new(), Vec::new());
        assert_eq!(true, logger.level_comparison(&Level::Info, &Level::Info));
    }

    #[test]
    pub fn lower_loglevel() {
        let logger = Logger::new(Level::Info, Level::Debug, None, Vec::new(), Vec::new());
        assert_eq!(false, logger.level_comparison(&Level::Info, &Level::Debug));
    }
}

// mod logger {
//     use super::*;
// }
