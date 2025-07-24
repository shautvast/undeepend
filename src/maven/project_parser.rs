use crate::maven::pom::Pom;
use crate::maven::pom_parser::get_pom;
use std::fs;
use std::path::Path;

pub fn parse_project(project_dir: &Path) -> Result<Project, String> {
    if !project_dir.is_dir() {
        return Err(format!("{:?} is not a directory", project_dir));
    }

    let mut pom_file = project_dir.to_path_buf();
    pom_file.push(Path::new("pom.xml"));

    let pom_file = fs::read_to_string(pom_file).map_err(|e| e.to_string())?;
    let root = get_pom(pom_file).map_err(|e| e.to_string())?;

    let modules= root.modules
        .iter()
        .map(|module| read_module_pom(project_dir, module))
        .collect();

    Ok(Project {
        root,
        modules,
    })
}

fn read_module_pom(project_dir: &Path, module: &String) -> Pom {
    let mut module_file = project_dir.to_path_buf();
    module_file.push(Path::new(module));
    module_file.push(Path::new("pom.xml"));
    let module_pom = fs::read_to_string(module_file)
        .expect(format!("Cannot read file {}", module).as_str());
    get_pom(module_pom).expect(format!("Cannot create module pom {}", module).as_str())
}

#[derive(Debug)]
pub struct Project {
    pub root: Pom,
    pub modules: Vec<Pom>,
}
