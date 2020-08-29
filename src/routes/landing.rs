use rocket::Rocket;

pub routes() -> Rocket{
    rocket::ignite()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .launch();
}
