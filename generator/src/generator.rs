use handlebars::Handlebars;
use crate::model::Recipe;

pub fn generate_recipe(r: &Recipe) {
    let mut templates = Handlebars::new();
    templates.register_template_string("recipe", include_str!("../templates/recipe.handlebars")).expect("Failed to load templates");
    println!("{}", templates.render("recipe", r).expect("Failed to render recipe"));
}
