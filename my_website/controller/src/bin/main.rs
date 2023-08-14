#[macro_use]
extern crate rocket;

use std::{
    time::Duration,
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    // thread
};

// use controller::slow_request::handle_connection;
// use controller::ThreadPool;


use config::launch_config::rocket_note_webapp;
use launch_aw::launch_rocket::rocket_anthony_webapp;

fn main() {

    let anthony_thread = std::thread::spawn(|| {
        rocket_anthony_webapp();
    });

    // Start the note_webapp in a separate thread
    let note_thread = std::thread::spawn(|| {
        rocket_note_webapp();
    });

    // Optionally, if you want the main thread to wait for both webapps to finish (which they likely never will under normal conditions)
    anthony_thread.join().expect("Anthony webapp thread had an error");
    note_thread.join().expect("Note webapp thread had an error");

    
    
}


// fn main() {

//     let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

//     // < Limits the amount of threads to 4 > 
//     let pool = ThreadPool::new(4);

//     for stream in listener.incoming() {
//         let stream = stream.unwrap();

//         pool.execute(|| {
//             handle_connection(stream);
//         });
//     }
// }













// < allows an infinite amount of threads to be spawned > 
// fn main(){
//     let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

//     for stream in listener.incoming() {
//         let stream = stream.unwrap();

//         handle_connection(stream);
//     }
// }

// fn main() {
//         // --snip--
//         // mspc: multiple producer , single consumerk
//         let (tx, rx) = mpsc::channel();

//         let tx1 = tx.clone();
//         thread::spawn(move || {
//             let vals = vec![
//                 String::from("hi"),
//                 String::from("from"),
//                 String::from("the"),
//                 String::from("thread"),
//             ];
    
//             for val in vals {
//                 tx1.send(val).unwrap();
//                 thread::sleep(Duration::from_secs(1));
//             }
//         });
    
//         thread::spawn(move || {
//             let vals = vec![
//                 String::from("more"),
//                 String::from("messages"),
//                 String::from("for"),
//                 String::from("you"),
//             ];
    
//             for val in vals {
//                 tx.send(val).unwrap();
//                 thread::sleep(Duration::from_secs(2));
//             }
//         });
    
//         for received in rx {
//             println!("Got: {}", received);
//         }
    
//         // --snip--
    
// }

// fn main() {
//     // mspc: multiple producer, single consumer
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];

//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     for received in rx {
//         println!("Got: {}", received);
//     }
// }

// use controller::wap;

// fn main() {
//     let v = vec![1, 2, 3];


//     let handle = thread::spawn(move || {
//         println!("Here's a vector: {:?}", v);
//     });


//     handle.join().unwrap();
// }

// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });
//     handle.join().unwrap();

//     for i in 1..5 {
//         println!("hi number {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     }

// }

// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     for i in 1..5 {
//         println!("hi number {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     }

//     handle.join().unwrap();
// }

// fn main() {
//     thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     for i in 1..5 {
//         println!("hi number {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     }
// }

// fn main() -> Result<(), Box<dyn Error>> {
    
    
//     // let mut controller = wap::WebAppController::new();

//     // // Start the default web app
//     // controller.start_note_webapp()?;

//     // // Here we simulate a delay, and then switch from note_webapp to anthony_webapp
//     // thread::sleep(Duration::from_secs(100));
//     // // controller.stop_note_webapp()?;
    
//     // // controller.start_anthony_webapp()?;

//     // // To keep anthony_webapp running
//     // loop {
//     //     thread::sleep(Duration::from_secs(100));
//     // }
// }

// ------------- //

// use rocket::{routes, fs::FileServer};
// use rocket_dyn_templates::{Template, context};
// use rocket::response::Redirect;

// #[macro_use]
// extern crate rocket;

// #[post("/index")]
// fn post_index() -> Redirect {
//     Redirect::to("/index")
// }

// #[get("/")]
// fn get_index() -> Template {
//     Template::render("index", context!{field: "value"})
// }
// #[launch]
// fn rocket() -> _ {
//     rocket::build()
//         .attach(Template::fairing())
//         .mount("/", routes![
//             get_index,
//             post_index
//         ])
//         .mount("/static", FileServer::from("static"))
// }