// log level
pub mod level {
    use std::fmt::Display;

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
    pub enum Level {
        Debug,
        Info,
        Warn,
        Error,
    }
    
    impl Display for Level {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[{:#?}]", self)
        }
    }
}

// log message
pub mod message {
    use std::fmt::Display;

    use crate::{log_field::LogField, level::Level};

    #[derive(Clone)]
    pub struct Message {
        level: Level,
        message: String,
        format: Vec<LogField>,
    }
    
    impl Message {
        pub fn new(level: Level, message: String, format: Vec<LogField>) -> Self {
            Self { level, message, format }
        }
    
        pub fn handle_formatter(&self, formatter: &LogField) -> String {
            match formatter {
                LogField::Time => chrono::Local::now().format("%Y-%m-%d").to_string(),
                LogField::Date => chrono::Local::now().format("%H:%M:%S.%3f").to_string(),
                LogField::Level => self.level.to_string(),
                LogField::Message => self.message.clone(),
                LogField::Seperator(separator) => separator.to_owned(),
            }
        }
    }

    impl Display for Message {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let string = self.format.iter().map(|formatter| self.handle_formatter(formatter)).collect::<Vec<String>>().join("");
            write!(f, "{}\n", string)
        }
    }
}

// log fields
pub mod log_field {
    #[derive(Clone)]
    pub enum LogField {
        Time,
        Date,
        Level,
        Message,
        Seperator(String),
    }
}

// logger struct
pub mod logger {
    use crate::{level::Level, log_field::LogField, message::Message};

    pub struct Logger {
        // logging level
        max_level: Level,
        // logging handle
        writers: Vec<Box<dyn std::io::Write>>,
        // formatting
        format: Vec<LogField>,
    }
    
    // logger implementation
    impl Logger {
        pub fn new(
            max_level: Level,
            writers: Vec<Box<dyn std::io::Write>>,
            format: Vec<LogField>,
        ) -> Self {
            Self {
                max_level,
                writers,
                format,
            }
        }
    
        pub fn log(&mut self, level: Level, content: String) -> std::io::Result<()> {
            let message = Message::new(level.clone(), content, self.format.clone());
            for writer in &mut self.writers {
                if self.max_level <= level {
                    writer.write_all(message.clone().to_string().as_bytes())?;
                }
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{level::Level, logger::Logger, log_field::LogField::*};

    #[test]
    pub fn level_format() {
        assert_eq!("[Info]", format!("{}", Level::Info.to_string()))
    }

    #[test]
    pub fn test() {
        let format = vec![
            Date,
            Seperator(String::from("T")),
            Time,
            Seperator(String::from(" ")),
            Level,
            Seperator(String::from(" ")),
            Message
        ];
        let mut logger = Logger::new(Level::Info, vec![Box::new(std::io::stdout())], format);
        logger.log(Level::Info, "we logging".to_owned()).unwrap();
    }
}
