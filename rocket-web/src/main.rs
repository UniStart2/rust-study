#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{delete, get, post, put, routes};

#[get("/<_id>")]
fn get_user(_id: u32) -> String {
    format!("login user id: {}", _id)
}

#[post("/create")]
fn create_user() -> String {
    "create user".to_string()
}

#[put("/update/<_id>")]
fn update_user(_id: u32) -> String {
    format!("updating user info that user id is {}", _id)
}

#[delete("/delete/<_id>")]
fn delete_user(_id: u32) -> String {
    format!("deleted profile that user id is {}", _id)
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![get_user, create_user, update_user, delete_user],
        )
        .mount(
            "/base",
            routes![get_user, create_user, update_user, delete_user],
        )
        .launch();
}
