#[macro_use]
extern crate rocket;
extern crate api;

use rocket::{routes, fs::FileServer};
use rocket_dyn_templates::Template;
use anthony_webapp::{a_gt,a_pt};
use api::{n_gt,n_pt};




#[launch]
fn my_website_rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![
            a_gt::index,
            a_gt::resume,
            a_gt::about,
            a_gt::webapp,
            // a_gt::navbar,
            a_pt::post_resume,
            a_pt::post_index,
            a_pt::post_about,
            a_pt::post_webapp,
            a_pt::note_webapp_login,
            n_gt::loginpage,
            n_gt::login_page,
            n_gt::homescreen,
            n_gt::createdeck,
            n_gt::library,
            n_gt::addtodeck,
            n_gt::practice,
            n_pt::library_redirect,
            n_pt::remove_deck,
            n_pt::create_deck_redirect,
            n_pt::sign_out,
            n_pt::add_to_deck,
            n_pt::add_note_to_deck,
            n_pt::login,
            n_pt::signup,
            n_pt::create_deck,
            n_pt::practice_deck,
            n_pt::check_answer,
            n_pt::goto_index,

        ])
        .mount("/static", FileServer::from("static"))
        .mount("/images", FileServer::from("images"))
        .mount("/scripts", FileServer::from("scripts"))
        .mount("/pdf", FileServer::from("pdf"))
        
}    