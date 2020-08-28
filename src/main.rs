#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate rocket_contrib;
extern crate tera;

// Bring both Template and Context into scope
use rocket_contrib::templates::Template;
use tera::Context;

#[get("/")]
fn index() -> Template {
    let mut context = Context::new();
    // let context = context();

    context.insert("my_message", &"Heya from template context!");
    Template::render("layout", &context)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .launch();
}
