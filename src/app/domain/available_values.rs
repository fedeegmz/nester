use std::path::Path;

pub struct AvailableValues<'a> {
    pub path: &'a Path,
    pub name: Option<String>,
    pub pkg: Option<String>,
}
