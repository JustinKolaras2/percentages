#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket_dyn_templates::{context, Template};

use percentages::run;

#[derive(FromForm)]
struct Equation {
    equation: String,
}

/// GET.

#[get("/")]
async fn home() -> Option<NamedFile> {
    NamedFile::open("static/home.html").await.ok()
}

// Manually added routes.
#[get("/style.css")]
async fn style() -> Option<NamedFile> {
    NamedFile::open("styles/style.css").await.ok()
}

#[get("/homeLogic.js")]
async fn home_logic() -> Option<NamedFile> {
    NamedFile::open("scripts/homeLogic.js").await.ok()
}

/// POST.

#[post("/results", data = "<equation>")]
async fn results(equation: Form<Equation>) -> Template {
    let equation: Equation = equation.into_inner();
    let equation: String = equation.equation;
    let equation: &str = equation.trim();

    match run(equation.to_string()) {
        Ok(result) => Template::render(
            "success",
            context! {
                elements: result.elements,
                percentage: result.percentage
            },
        ),
        Err(error) => Template::render(
            "error",
            context! {
                error: error.error,
                emphasis: error.emphasis
            },
        ),
    }
}

/// Catchers.

#[catch(404)]
async fn not_found() -> Option<NamedFile> {
    NamedFile::open("static/catchers/404.html").await.ok()
}

#[catch(500)]
async fn internal_error() -> Option<NamedFile> {
    NamedFile::open("static/catchers/500.html").await.ok()
}

/// Launch.

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found, internal_error])
        .mount("/", routes![home, results, style, home_logic])
        .attach(Template::fairing())
}
