extern crate rocket;

pub mod run_rocket {
    pub fn rocket_config() {
        println!("FUCK");
    }

}

pub mod fun {
    use rocket::response::status::BadRequest;
    use lazy_static::lazy_static;
    use rusqlite::{params, Connection, Result};
    use std::sync::Mutex;
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
    
    pub fn check_id_exists(conn: &Connection, id: i32) -> Result<bool> {
        let query = "SELECT COUNT(*) FROM users WHERE id = ?";
        let count: i64 = conn.query_row(query, [id], |row| row.get(0))?;
        Ok(count > 0)
    }
    pub fn check_if_user_exists(conn: &Connection, user: String, pass: String) -> Result<bool> {
        let query = "SELECT COUNT(*) FROM users WHERE username = ? AND password = ?";
        let count: i64 = conn.query_row(query, [&user, &pass], |row| row.get(0))?;
        Ok(count > 0)
    }
    pub fn attempt_add_user(conn: &Connection, user: String) -> Result<bool> {
        let query = "SELECT COUNT(*) FROM users WHERE username = ?";
        let count: i64 = conn.query_row(query, [&user], |row| row.get(0))?;
        Ok(count > 0)
    }
    
    impl NewUser {
        pub fn add(username: &str, password: &str) -> Result<()> {
            let conn: Connection = Connection::open("data/fpdb.db")?;
    
            let mut rng = rand::thread_rng();
            let mut id: i32;
    
            loop {
                id = rng.gen_range(0..1000);
    
                if check_id_exists(&conn, id)? {
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
    
    // User Status Checking ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    
    // CHECKING FOR CORRECT ANSWERS **************************************
    
    pub fn check_practice_answers(conn: &Connection, table: &str, english: &str, answer: &str) -> Result<bool> {
        let query = format!(
            "SELECT COUNT(*) FROM {}
                WHERE english = ?
                and japanese = ?
            ",
            table,
        );
        
        let count: i64 = conn.query_row(&query, [&english,&answer], |row| row.get(0))?;
        Ok(count > 0)
    }
    
    
    // CHECKING FOR CORRECT ANSWERS END  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    //Create Deck Form ******************************************************
    pub fn check_if_table_exists(table_name: &str) -> Result<bool, rusqlite::Error> {
        let conn = DECKDB_CONN.lock().unwrap();
        let query = "SELECT name FROM sqlite_master WHERE type='table' AND name=?";
        let result: Result<String, rusqlite::Error> = conn.query_row(query, [table_name], |row| row.get(0));

        match result {
            Ok(name) => {
                eprintln!("Table '{}' exists", name);
                Ok(true)  // Table exists
            },
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(false),  // Table does not exist
            Err(err) => Err(err),  // Error occurred while checking
        }
    }

    pub fn add_new_deck(deck_name: &str) -> Result<(), rusqlite::Error> {
        let conn = DECKDB_CONN.lock().unwrap();
        let table_name = format!("{}", deck_name);

        conn.execute(
            &format!(
                "CREATE TABLE {} (
                    id INTEGER PRIMARY KEY,
                    english TEXT,
                    japanese TEXT
                )",
                table_name
            ),
            [],
        )?;

        Ok(())
    }
    
    pub fn count_rows(conn: &Connection, table: &str) -> Result<i64> {
        let query = format!("SELECT COUNT(*) FROM {}",table);
        let count: Option<i64> = conn.query_row(&query, [], |row| row.get(0))?;
        Ok(count.unwrap_or(0))
    }
    pub fn get_next_english_word(conn: &Connection, table: &str, current_word: &str) -> Result<String, BadRequest<String>> {

        let id_query = format!(
            "SELECT id from {} where
                english = ?",
                table
        );
        
    
        let current_id: Result<i64, rusqlite::Error> = conn.query_row(&id_query, [current_word], |row| row.get(0));
        let num_rows = count_rows(&conn, &table);
        if let Ok(current_id) = current_id {
            if current_id >= num_rows.unwrap() {
                return Err(BadRequest(Some("This is the final word!".to_string())));
            }
        } else {
            return Err(BadRequest(Some("Failed to retrieve current_id".to_string())));
        }
    
        let next_id = current_id.unwrap() + 1;
    
        let word_query = format!(
            "SELECT english from {} where
                id = ?",
                table
        );
    
        let next_word: Result<String, rusqlite::Error> = conn.query_row(&word_query, [&next_id], |row| row.get(0));
    
        match next_word {
            Ok(word) => Ok(word),
            Err(_) => Err(BadRequest(Some("Failed to retrieve next word".to_string()))),
        }
        
    }
    // GETTING TABLES FOR LIBRARY *****************************************
    pub fn get_table_names() -> Result<Vec<String>, rusqlite::Error> {
        let conn = DECKDB_CONN.lock().unwrap();
        let query = "SELECT name FROM sqlite_master WHERE type='table'";
        let mut stmt = conn.prepare(query)?;

        let table_names: Result<Vec<String>, rusqlite::Error> = stmt
            .query_map([], |row| row.get(0))
            .map(|result| result.collect())
            .unwrap();

        // Add a debug print here
        println!("Table names: {:?}", table_names);

        table_names
    }
    // GETTING TABELS FOR LIBRARY END ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


}

