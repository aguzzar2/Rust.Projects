extern crate rocket;
extern crate anthony_webapp;

pub mod launch_rocket{
    use rocket::{routes, fs::FileServer, Config, Rocket, Build};
    use rocket_dyn_templates::Template;
    use anthony_webapp::{gt,pt};
    
    pub fn rocket_anthony_webapp() -> Rocket<Build> {
        rocket::build()
            .configure(Config{
                    port: 8001,
                    ..Config::default()
                })
            .attach(Template::fairing())
            .mount("/", routes![
                gt::index,
                gt::resume,
                gt::about,
                gt::webapp,
                pt::post_resume,
                pt::post_index,
                pt::post_about,
                pt::post_webapp
            ])
            .mount("/static", FileServer::from("static"))
            .mount("/images", FileServer::from("images"))
    }    
}
