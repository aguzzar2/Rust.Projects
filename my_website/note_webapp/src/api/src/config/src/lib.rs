extern crate rocket;
extern crate api;



pub mod launch_config{
    use api::gt;
    use api::pt;

    use rocket::{Rocket, routes, fs::FileServer, Build};
    use rocket_dyn_templates::Template;

    pub fn rocket_note_webapp() -> Rocket<Build> {

        rocket::build()
            .attach(Template::fairing())
            .mount("/", routes![
                gt::loginpage,
                pt::login,
                gt::login_page,
                pt::signup,
                gt::homescreen,
                pt::create_deck,
                gt::createdeck,
                pt::sign_out,
                pt::create_deck_redirect,
                pt::library_redirect,
                gt::library,
                gt::addtodeck,
                pt::add_to_deck,
                pt::add_note_to_deck,
                gt::practice,
                pt::practice_deck,
                pt::check_answer,
            ])
            .mount("/static", FileServer::from("static"))
    }
        
}


