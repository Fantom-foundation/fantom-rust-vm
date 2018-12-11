pub mod handlers;

pub fn start_web() {
  rocket::ignite().mount("/", routes![handlers::health]).launch();
}