use rocket::{get, launch, routes};
use rocket_dyn_templates::{context, Template};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Chapter {
    it: Vec<Vec<String>>,
    en: Vec<Vec<String>>,
}

#[get("/")]
fn index() -> Template {
    let chapter_str = fs::read_to_string("./json/chapters/1.json").expect("unable to read file");
    let chapter: Chapter = serde_json::from_str(&chapter_str).expect("json was not well formatted");
    Template::render(
        "index",
        context! { chapter: chapter, chapter_str: chapter_str },
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}
