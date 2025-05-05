use crate::core::port::logger_port::LoggerPort;

pub struct Logger {}

impl Logger {
    pub fn new() -> Self {
        Self {}
    }
}

impl LoggerPort for Logger {
    fn info(&self, msg: &str) {
        println!("{}", msg);
    }

    fn success(&self, msg: &str) {
        println!("{}", msg);
    }

    fn warn(&self, msg: &str) {
        eprintln!("{}", msg);
    }

    fn error(&self, msg: &str) {
        eprintln!("{}", msg);
        std::process::exit(1);
    }
}
