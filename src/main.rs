#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate rocket_contrib;
extern crate mylib;

use mylib::get_data;

//use std::path::{Path, PathBuf};

//use rocket::response::NamedFile;
use rocket_contrib::{templates::Template, serve::StaticFiles};

#[macro_use]
extern crate serde_derive;

extern crate serde_json;

#[derive(Serialize)]
struct TemplateContext {
    title: &'static str,
    name: Option<String>,
    items: mylib::Obj,
    parent: &'static str,
}

#[get("/")]
fn index() -> Template {
    let p = get_data().unwrap();
    Template::render("index", &TemplateContext {
        title: "Hello",
        name: None,
        items: p,
        parent: "layout",
    })
}

/*#[get("/<file..>", rank=3)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}*/

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", StaticFiles::from("static"))
        .mount("/", routes![index])
        .attach(Template::fairing())
}
 
fn main() {
    rocket().launch();
}

#[cfg(test)]
mod test {
    use super::{rocket, TemplateContext};
    use rocket::local::{Client, LocalResponse};
    use rocket::http::Method::*;
    use rocket::http::Status;
    use rocket_contrib::templates::Template;

    extern crate mylib;
    use mylib::get_data;

    macro_rules! dispatch {
        ($method:expr, $path:expr, $test_fn:expr) => ({
            let client = Client::new(rocket()).unwrap();
            $test_fn(&client, client.req($method, $path).dispatch());
        })
    }
    

    #[test]
    fn test_root() {
        dispatch!(Get, "/", |client: &Client, mut response: LocalResponse<'_>| {
            let p = get_data();
            assert!(p.is_ok());
            let context = TemplateContext {
                title: "Hello",
                name: None,
                items: p.unwrap(),
                parent: "layout",
            };

            let expected = Template::show(client.rocket(), "index", &context).unwrap();
            assert_eq!(response.status(), Status::Ok);
            assert_eq!(response.body_string(), Some(expected));
        });
    }
}
