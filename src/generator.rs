use std::collections::HashMap;
use handlebars::Handlebars;
use crate::model::Recipe;

pub fn generate_recipe(r: &Recipe) -> String {
    let mut templates = Handlebars::new();
    templates.register_template_string("recipe", include_str!("../templates/recipe.handlebars")).expect("Failed to load templates");
    templates.render("recipe", r).expect("Failed to render recipe")
}

pub fn generate_index(recipes: &Vec<String>) -> String {
    let mut templates = Handlebars::new();
    let data = HashMap::from([("recipes", recipes)]);
    templates.register_template_string("index", include_str!("../templates/index.handlebars")).expect("Failed to load templates");
    templates.render("index", &data).expect("Failed to render index")
}
