pub trait LoggerPort {
    fn info(&self, msg: &str);

    fn success(&self, msg: &str);

    fn warn(&self, msg: &str);

    fn error(&self, msg: &str);
}
