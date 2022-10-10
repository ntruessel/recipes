use std::fs;
use std::path::Path;
use crate::generator::generate_recipe;

mod model;
mod generator;

fn main() {
    let _ = fs::remove_dir_all("public");
    fs::create_dir("public").unwrap();
    let paths = fs::read_dir("content").unwrap();
    for path in paths {
        let directory = path.unwrap();
        let recipe_file = directory.path().join("recipe.yaml");
        let recipe = serde_yaml::from_str(fs::read_to_string(recipe_file).unwrap().as_str()).unwrap();
        let content = generate_recipe(&recipe);
        let mut outpath = Path::new("public").join(directory.file_name());
        outpath.set_extension("html");
        fs::write(outpath, content).unwrap();
    }
}
