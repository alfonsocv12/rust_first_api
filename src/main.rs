#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate rocket_contrib;
extern crate tera;

use rocket_contrib::templates::Template;
use std::collections::HashMap;

trait LandingRoutes {
    fn new() -> Self;

    #[get("/")]
    fn index(&self) -> Template {
        let mut context = HashMap::new();
        context.insert("my_message", &"Heya from template context!");

        Template::render("layout", context)
    }
}

fn array_routes() -> routes{
    let routes = LandingRoutes::new();

    return routes![
        routes.index(),
    ];
}

fn main() {
    rocket::ignite()
        .mount("/", array_routes())
        .attach(Template::fairing())
        .launch();
}
