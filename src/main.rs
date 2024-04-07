use rocket::{get, launch, response::Redirect, routes, uri};
use rocket_dyn_templates::{context, Template};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Chapter {
    it: Vec<Vec<String>>,
    en: Vec<Vec<String>>,
}

#[get("/<chapter>")]
fn chapter(chapter: u8) -> Template {
    let path = format!("./json/chapters/{}.json", chapter);
    let chapter_str = fs::read_to_string(path).expect("unable to read file");
    let chapter: Chapter = serde_json::from_str(&chapter_str).expect("json was not well formatted");
    Template::render(
        "index",
        context! { chapter: chapter, chapter_str: chapter_str },
    )
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!(chapter(1)))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, chapter])
        .attach(Template::fairing())
}
