
#[macro_use]
extern crate rocket;

pub mod a_pt{
    use rocket::response::Redirect;

    #[post("/notewebapp_login")]
    pub fn note_webapp_login() -> Redirect {
        Redirect::to("/login")
    }

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



pub mod a_gt {
    use rocket_dyn_templates::Template;
    use std::collections::HashMap;
    fn get_navbar_html() -> String {
        String::from("
        <nav id = 'nav-container' class = 'navbar-container'>
            <div class = 'top-navbar'>
                <form id = 'title-form' method = 'post' action = '/index'>
                    <button class ='homepage__button' type = 'submit'>
                        Anthony Guzzardo
                    </button>
                </form>
            </div>

            <div class='bottom-navbar'>
                <form id='app-form' method='post' action='/webapp'>
                    <button class='nav__links-button' type='submit'>Projects</button>
                </form>
                <form id='resume-form' method='post' action='/resume'>
                    <button class='nav__links-button' type='submit'>Resume</button>
                </form>
                
            </div>
        </nav>")
        // <form id='about-form' method='post' action='/about'>
        //             <button class='nav__links-button' type='submit'>About</button>
        //         </form>
        // <form action='https://github.com/aguzzar2'>
        //             <button class='homepage__button' type='submit'>
        //                 <img id = 'link__icons' src = '/images/github-logo.png' alt = 'linkedin'>
        //             </button>
                    
        //         </form>
        //         <form action='https://www.linkedin.com/in/anthony-guzzardo-648b7116a/'>
        //             <button class='homepage__button' type='submit'>
        //                 <img id = 'link__icons' src = '/images/linkedin-logo.png' alt = 'linkedin'>
        //             </button>
        //         </form>

    }

    #[get("/")]
    pub fn index() -> Template {
        let mut context = HashMap::new();
        context.insert("navbar_html", get_navbar_html());
        Template::render("index", context)
    }
    #[get("/resume")]
    pub fn resume() -> Template{
        let mut context = HashMap::new();
        context.insert("navbar_html", get_navbar_html());
        Template::render("resume", context)
    }

    #[get("/about")]
    pub fn about() -> Template {
        let mut context = HashMap::new();
        context.insert("navbar_html", get_navbar_html());
        Template::render("about", context)
    }

    #[get("/webapp")]
    pub fn webapp() -> Template {
        let mut context = HashMap::new();
        context.insert("navbar_html", get_navbar_html());
        Template::render("webapp", context)
    }
    

    
}
