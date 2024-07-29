#[allow(dead_code)]
#[allow(unused_imports)]

#[macro_use] extern crate rocket;
use rocket::tokio::sync::broadcast::{channel, Sender};
use rocket::tokio::sync::broadcast::error::RecvError;
use rocket::response::stream::{EventStream, Event};
use rocket::tokio::select;
use rocket::fs::{relative, FileServer};
use rocket::form::Form;
use rocket::{State, Shutdown};
use rocket::serde::{Serialize, Deserialize};

#[get("/word2")]
fn world() -> & 'static str{
    "Hello World..."
}

#[derive(Debug, Clone, FromForm, Deserialize, Serialize)]
#[serde(crate="rocket::serde")]
struct Message {
    #[field(validate = len(..30))]
    pub room: String,
    #[field(validate = len(..20))]
    pub username: String,
    pub message: String
}

#[post("/message", data="<form>")]
fn post_message(form: Form<Message>, queue: &State<Sender<Message>>){
    let _res = queue.send(form.into_inner()); // what into_inner() does?
}

#[get("/events")]
async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![]{
    let mut rx = queue.subscribe();

    EventStream!{
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue
                },
                _ = &mut end => break,
            };
            yield Event::json(&msg);
        }
    }
}


#[launch]
fn main_rocket() -> _{
    rocket::build()
        .manage(channel::<Message>(1024).0)
        .mount("/", routes![world, post_message, events])
        .mount("/", FileServer::from(relative!("static")))
}