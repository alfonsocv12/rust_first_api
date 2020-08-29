extern crate rocket_contrib;
extern crate tera;

pub mod LangindController {
    use rocket_contrib::templates::Template;
    use std::collections::HashMap;

    #[get("/")]
    pub fn index() -> Template {
        let mut context = HashMap::new();
        context.insert("my_message", &"Heya from template context!");

        Template::render("layout", context)
    }
}
