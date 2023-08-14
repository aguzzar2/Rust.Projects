
extern crate rocket;
extern crate config;
use config::launch_config::rocket_note_webapp;
use rocket::launch;


#[launch]
fn rocket_league() -> _ {
    rocket_note_webapp()
}
