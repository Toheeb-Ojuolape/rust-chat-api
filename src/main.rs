#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rusqlite;
extern crate serde;

use rocket::{get, post, routes};
use rocket_contrib::json::{Json, JsonValue};
use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    id: i32,
    text: String,
}

#[post("/send_message", data = "<message>")]
fn send_message(conn: rocket::State<Connection>, message: Json<Message>) -> JsonValue {
    let mut stmt = conn
        .prepare("INSERT INTO messages (text) VALUES (?1)")
        .unwrap();
    stmt.execute(params![message.text]).unwrap();

    let last_id: i32 = conn
        .prepare("SELECT last_insert_rowid()")
        .unwrap()
        .query_row(rusqlite::NO_PARAMS, |row| row.get(0))
        .unwrap();

    json!({ "status": "ok", "id": last_id })
}

#[get("/get_messages")]
fn get_messages(conn: rocket::State<Connection>) -> JsonValue {
    let mut stmt = conn.prepare("SELECT id, text FROM messages").unwrap();
    let messages = stmt
        .query_map(params![], |row| Message {
            id: row.get(0),
            text: row.get(1),
        })
        .unwrap()
        .map(|result| result.unwrap())
        .collect::<Vec<_>>();

    json!({ "status": "ok", "messages": messages })
}

fn main() {
    rocket::ignite()
        .manage(Connection::open("database.db").unwrap())
        .mount("/", routes![send_message, get_messages])
        .launch();
}