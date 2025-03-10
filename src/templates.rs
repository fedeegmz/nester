use git2::Repository;
use std::error::Error;
use tera::{Context, Tera};

use crate::{file_system::is_dir_empty, project::Project, utils::get_templates_path};

pub fn get_content(
    project: &Project,
    template_name: &str,
    module_name: &String,
) -> Result<String, Box<dyn Error>> {
    let templates_path = get_templates_path();
    let template_files = &format!("{}/tera/*.tera", templates_path.to_str().unwrap());

    let tera = Tera::new(&template_files).expect("❌ Error loading templates");

    let mut context = Context::new();
    context.insert("pkg_name", project.package_name.as_ref().unwrap());
    context.insert("module_name", module_name);

    let rendered = tera
        .render(template_name, &context)
        .expect("❌ Error rendering template");

    Ok(rendered)
}

pub fn clone_templates_repo(repository: &str) -> Result<(), Box<dyn Error>> {
    println!("Cloning templates repository: {}", &repository);
    let templates_path = get_templates_path();
    if is_dir_empty(&templates_path)? {
        Repository::clone(repository, &templates_path)?;
    }

    Ok(())
}

// pub fn fetch_templates_repo(remote: &str, branch: &str) -> Result<(), Box<dyn Error>> {
//     let templates_path = get_templates_path();
//     if let Ok(repo) = Repository::open(&templates_path) {
//         let mut remote = repo.find_remote(remote)?;
//         remote.fetch(&[branch], None, None)?;
//     } else {
//         return Err(Box::new(std::io::Error::new(
//             std::io::ErrorKind::Other,
//             "❌ Failed to open repository",
//         )));
//     }

//     Ok(())
// }
