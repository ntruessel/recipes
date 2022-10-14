use std::fs;
use std::path::Path;
use crate::generator::{generate_index, generate_recipe};

mod model;
mod generator;

fn main() {
    let _ = fs::remove_dir_all("public");
    fs::create_dir("public").unwrap();
    let paths = fs::read_dir("content").unwrap();
    let mut recipes = Vec::new();
    for path in paths {
        let directory = path.unwrap();
        recipes.push(directory.file_name().to_str().unwrap().to_string());
        let recipe_file = directory.path().join("recipe.yaml");
        let recipe = serde_yaml::from_str(fs::read_to_string(recipe_file).unwrap().as_str()).unwrap();
        let content = generate_recipe(&recipe);
        let mut outpath = Path::new("public").join(directory.file_name());
        outpath.set_extension("html");
        fs::write(outpath, content).unwrap();
    }
    let index = generate_index(&recipes);
    let outpath = Path::new("public/index.html");
    fs::write(outpath, index).unwrap();
    fs::copy(Path::new("templates/main.css"), Path::new("public/main.css")).unwrap();
}
