extern crate rocket;
extern crate config;


pub mod launch_noteapp{
    use rocket::launch;
    use config::launch_config::rocket_note_webapp;

    #[launch]
    pub fn rocket_launch() -> _ {
        rocket_note_webapp()
    }

}