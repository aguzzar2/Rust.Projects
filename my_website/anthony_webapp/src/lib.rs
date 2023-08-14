
#[macro_use]
extern crate rocket;

pub mod gt {
    use rocket_dyn_templates::{Template, context};

    #[get("/")]
    pub fn index() -> Template {
        Template::render("index", context!{})
    }
    #[get("/resume")]
    pub fn resume() -> Template{
        Template::render("resume", context!{})
    }

    #[get("/about")]
    pub fn about() -> Template {
        Template::render("about", context!{})
    }

    #[get("/webapp")]
    pub fn webapp() -> Template {
        Template::render("webapp", context!{})
    }
}
pub mod pt{
    use rocket::response::Redirect;


    #[post("/index")]
    pub fn post_index() -> Redirect {
        Redirect::to("/")
    }

    #[post("/resume")]
    pub fn post_resume() -> Redirect {
        Redirect::to("/resume")
    }

    #[post("/about")]
    pub fn post_about() -> Redirect {
        Redirect::to("/about")
    }

    #[post("/webapp")]
    pub fn post_webapp() -> Redirect {
        Redirect::to("/webapp")
    }

}

