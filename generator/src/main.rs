use std::fs;
use crate::generator::generate_recipe;

mod model;
mod generator;

fn main() {
    let paths = fs::read_dir("./recipes").unwrap();
    for path in paths {
        let recipe = serde_yaml::from_str(fs::read_to_string(path.unwrap().path()).unwrap().as_str()).unwrap();
        generate_recipe(&recipe);
    }
}
