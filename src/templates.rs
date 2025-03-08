use std::error::Error;
use tera::{Context, Tera};

use crate::project::Project;

pub fn get_content(
    project: &Project,
    template_name: &str,
    module_name: &String,
) -> Result<String, Box<dyn Error>> {
    let home = dirs::home_dir().ok_or("❌ Error: home dir not found")?;
    let templates_path = home.join(".nester/templates");
    let tera = Tera::new(&format!("{}*.tera", templates_path.to_str().unwrap()))
        .expect("❌ Error loading templates");

    let mut context = Context::new();
    context.insert("pkg_name", project.package_name.as_ref().unwrap());
    context.insert("module_name", module_name);

    let rendered = tera
        .render(template_name, &context)
        .expect("❌ Error rendering template");

    Ok(rendered)
}
