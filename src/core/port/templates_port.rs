pub trait TemplatesPort {
    fn load(&self, template: &str, name: Option<String>, pkg: Option<String>) -> String;
}
