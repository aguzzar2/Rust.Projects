extern crate rocket;
extern crate note_webapp;

pub mod n_pt {
    use note_webapp::fun;
    use rocket::{FromForm, post};
    use rocket::response::Redirect;
    use rusqlite::{params, Connection, Result};
    use rusqlite::ToSql;
    use rocket::form::Form;
    use lazy_static::lazy_static;
    use std::sync::Mutex;
    use urlencoding::encode;
    use rand::Rng;


    lazy_static! {
        static ref DB_CONN: Mutex<Connection> = Mutex::new(Connection::open("data/fpdb.db")
            .expect("Failed to open database connection"));
        static ref DECKDB_CONN: Mutex<Connection> = Mutex::new(Connection::open("data/deckdb.db")
            .expect("Failed to open database connection"));
    }

    // User Status Checking *******************************************
    pub struct NewUser {
        username: String,
        password: String,
    }

    impl NewUser {
        pub fn add(username: &str, password: &str) -> Result<()> {
            let conn: Connection = Connection::open("data/fpdb.db")?;
    
            let mut rng = rand::thread_rng();
            let mut id: i32;
    
            loop {
                id = rng.gen_range(0..1000);
    
                if fun::check_id_exists(&conn, id)? {
                    println!("ID already exists. Please enter a unique ID.");
                } else {
                    break;
                }
            }
    
            let new_user = NewUser {
                username: username.to_owned(),
                password: password.to_owned(),
            };
    
            conn.execute(
                "INSERT INTO users(id, username, password) VALUES(?1, ?2, ?3)",
                params![id, new_user.username, new_user.password],
            )?;
    
            Ok(())
        }
    }
    
    // POSTS Redirect Templates  **********************************************
    #[post("/library")]
    pub fn library_redirect() -> Redirect {
        Redirect::to("/library")
    }

    #[derive(FromForm)]
    pub struct RemoveDeck {
        table: String,
    }

    #[post("/remove-deck", data = "<form>")]
    pub fn remove_deck(form: Form<RemoveDeck>) -> Redirect {
        let table_name = form.table.clone();

        let conn = DECKDB_CONN.lock().unwrap();

        match conn.execute(
            &format!(
                "DROP TABLE {}",
                table_name
            ),
            [],
        ) {
            Ok(_) => Redirect::to("/library"),
            Err(e) => {
                eprintln!("Failed to drop table: {}", e);
                Redirect::to("/error")
            }
        }
    }

    #[post("/create-deck")]
    pub fn create_deck_redirect() -> Redirect {
        Redirect::to("/createdeck")
    }

    #[post("/sign-out")]
    pub fn sign_out() -> Redirect {
        Redirect::to("/login")
    }

    #[derive(FromForm)]
    pub struct AddToDeck {
        table: String,
    }

    #[post("/addto-deck", data = "<form>")]
    pub fn add_to_deck(form: Form<AddToDeck>) -> Redirect {
        let table_name = form.table.clone();

        eprintln!("Table name is {}, rerouting to addtodeck.html", &table_name);

        Redirect::to(format!("/addtodeck/{}", &table_name))
    }

    #[derive(FromForm)]
    pub struct AddNotes {
        english: String,
        japanese: String,
        table: String,
    }

    #[post("/add", data = "<form>")]
    pub fn add_note_to_deck(form: Form<AddNotes>) -> Redirect {
        let eng_word = form.english.clone();
        let jap_word = form.japanese.clone();
        let table_name: String = form.table.clone();

        let conn = DECKDB_CONN.lock().unwrap();

        let count: i64= note_webapp::fun::count_rows(&conn, &table_name).unwrap();

        let query = format!(
            "INSERT INTO {} (id, english, japanese) VALUES (?,?,?)",
            table_name
        );

        let params: &[&(dyn ToSql)] = &[&count, &eng_word, &jap_word];
        conn.execute(&query, params).expect("Failed to insert Values");


        eprintln!("ID ... is, English Word is {}, Japanese Word is {}, Table name is {}", &eng_word, &jap_word, &table_name);
        Redirect::to(format!("/addtodeck/{}", &table_name))

    

    }
        
    // LOGIN/ SIGNUP ATTEMPTS ******************************************
    #[derive(FromForm)]
    pub struct LoginForm {
        username: String,
        password: String,
    }

    #[post("/login", data = "<form>")]
    pub fn login(form: Form<LoginForm>) -> Redirect {
        let username = form.username.clone();
        let password = form.password.clone();

        let conn = DB_CONN.lock().unwrap();

        let user_exists = fun::check_if_user_exists(&conn, username.clone(), password.clone())
            .unwrap_or(false);

        if user_exists {
            eprintln!("User: {} Signing in!",username.clone());
            eprintln!("User: {} Signing in!",password.clone());
            Redirect::to("/homescreen")
        }else {
            eprintln!("User: {} Doesn't Exist!",username.clone());
            Redirect::to("/login")
        }
    }

    #[derive(FromForm)]
    pub struct SignUpForm {
        username: String,
        password: String,
    }

    #[post("/signup", data = "<form>")]
    pub fn signup(form: Form<SignUpForm>) -> Redirect {
        let username = form.username.clone();
        let password = form.password.clone();

        let conn = DB_CONN.lock().unwrap();

        let username_exists = fun::attempt_add_user(&conn, username.clone())
            .unwrap_or(false);

        if username_exists {
            eprintln!("Username: {} is taken!",username.clone());
            Redirect::to("/login")
            
        } else {
            match NewUser::add(&username, &password) {
                Ok(_) => {
                    println!("User added: {}", username);
                    eprintln!("Password Added: {}",password.clone());
                    Redirect::to("/login")
                }
                Err(err) => {
                    eprintln!("Failed to add user: {}", err);
                    Redirect::to("/login")
                }
                }
            }
    }
    
    #[derive(FromForm)]
    pub struct AddDeck{
        deckname : String,
    }

    #[post("/newdeck", data = "<form>")]
    pub fn create_deck(form: Form<AddDeck>) -> Redirect {
        let deck_name = form.deckname.clone();

        let status = fun::check_if_table_exists(&deck_name).unwrap_or(false);


        if status{
            println!("Deck Name: {} is Taken", &deck_name);
            Redirect::to("/createdeck")
        } else {
            let _ = fun::add_new_deck(&deck_name);
            eprintln!("{} Deck Added!", &deck_name);
            Redirect::to("/createdeck")
        }
    }
    #[derive(FromForm)]
    pub struct PracticeDeck {
        table: String,
    }

    #[post("/practice-deck", data = "<form>")]
    pub fn practice_deck(form: Form<PracticeDeck>) -> Redirect {
        let table_name = form.table.clone();

        eprintln!("Table name is {}, rerouting to addtodeck.html", &table_name);

        let conn = DECKDB_CONN.lock().unwrap();

        let query = format!(
            "SELECT english from {} where
                id = 0",
            table_name
        );

        let english_word: String = match conn.query_row(&query, [], |row| row.get(0)) {
            Ok(word) => word,
            Err(err) => {
                eprintln!("Failed to retrieve English word: {}", err);
                return Redirect::to("/library");
            }
        };

        let encoded_english_word = encode(&english_word);
        let encoded_url = format!("/practice/{}/{}", &table_name, &encoded_english_word);

        Redirect::to(encoded_url)
    }


    #[derive(FromForm)]
    pub struct Answers {
        table: String,
        answer: String,
        english_word: String,
    }

    #[post("/check-answer", data = "<form>")]
    pub fn check_answer(form: Form<Answers>) -> Redirect {
        let table_name = form.table.clone();
        let jap_word = form.answer.clone();
        let eng_word = form.english_word.clone();

        eprintln!("English Word {}, Jap Answer: {} from Table: {}", &eng_word, &jap_word, &table_name);

        let conn = DECKDB_CONN.lock().unwrap();

        let query: bool = fun::check_practice_answers(&conn, &table_name, &eng_word, &jap_word).unwrap_or(false);

        if query {
            let is_last_word = format!(
                "select id from {} where
                    english = ?",
                table_name
            );

            let num_rows = fun::count_rows(&conn, &table_name).unwrap_or(0);

            let curr_id: Result<i64, rusqlite::Error> = conn.query_row(&is_last_word, [&eng_word], |row| row.get(0));
            if (curr_id.unwrap()) + 1 == num_rows {
                eprintln!("Your Answer {} was correct!", &jap_word);
                Redirect::to("/library")
            } else {
                eprintln!("Your Answer {} was correct!", &jap_word);
                let next_english_word = fun::get_next_english_word(&conn, &table_name, &eng_word).unwrap();
                let encoded_table_name = encode(&table_name);
                let encoded_next_english_word = encode(&next_english_word);
                Redirect::to(format!("/practice/{}/{}", encoded_table_name, encoded_next_english_word))
            }
        } else {
            eprintln!("Your Answer {} was incorrect!", &jap_word);
            let encoded_table_name = encode(&table_name);
            let encoded_english_word = encode(&eng_word);
            Redirect::to(format!("/practice/{}/{}", encoded_table_name, encoded_english_word))
        }
    }
    #[post("/goto-index")]
    pub fn goto_index() -> Redirect {
        Redirect::to("/")
    }
}



pub mod n_gt {
    use rocket::{response::status::BadRequest, get};
    use rocket_dyn_templates::{Template, context};
    use rocket::serde::json::json;

    #[get("/loginpage")]
    pub fn loginpage() -> Template {
        Template::render("login", context! { field: "value" })
    }

    #[get("/login")]
    pub fn login_page() -> Template {
        Template::render("login", context! {})
    }

    #[get("/homescreen")]
    pub fn homescreen() -> Template {
        Template::render("homescreen", context! { field: "value" })
    }
    #[get("/createdeck")]
    pub fn createdeck() -> Template {
        Template::render("createdeck", context! { field: "value" })
    }

    #[get("/library")]
    pub fn library() -> Template {
        let table_names = note_webapp::fun::get_table_names().unwrap_or_else(|_| Vec::new());

        println!("Existing Decks Are {:?}", table_names);

        let context = json!({
            "tables": table_names,
        });
        Template::render("library", context)
    }


    #[get("/addtodeck/<table_name>")]
    pub fn addtodeck(table_name: String) -> Template {
        eprintln!("Rerouting to addtodeck.html.tera");
        let context = json!({
            "table": table_name,
        });
        Template::render("addtodeck", context)
    }

    #[get("/practice/<table_name>/<english_word>")]
    pub fn practice(table_name: String, english_word: String) -> Result<Template, BadRequest<String>> {
        eprintln!("Rerouting to practice.html.tera");
        let japanese_word = "asdf";
        let context = json!(
            {
                "table": table_name,
                "japaneseWord": japanese_word,
                "englishWord": english_word, 
            }
        );
        Ok(Template::render("practice", context))
        
    }

}

